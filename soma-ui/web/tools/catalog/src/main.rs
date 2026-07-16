// ponytail: assumes hand-written component files with one-attr-per-line style and
// non-generic enum definitions. Ceiling: macro-heavy or multi-line-generic signatures
// will be misparsed. Upgrade path: replace line-scanning with syn-based AST parsing.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn to_title_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug)]
struct EnumDef {
    name: String,
    variants: Vec<(String, bool)>, // (name, is_default)
}

#[derive(Debug)]
struct PropDef {
    name: String,
    ty: String,
    notes: String,
}

#[derive(Debug)]
struct ComponentDef {
    name: String,
    props: Vec<PropDef>,
    has_children: bool,
}

#[derive(Debug)]
struct HookDef {
    name: String,
    params: String,
    ret: String,
}

#[derive(Debug)]
struct FileDef {
    enums: Vec<EnumDef>,
    components: Vec<ComponentDef>,
    playground_path: Option<String>,
}

/// Collect names of `pub fn`s preceded by `#[component]` — same logic as parse_file.
/// Returns (count, names) for the parity tripwire; names enable diff diagnostics.
fn scan_pub_components(content: &str) -> (usize, Vec<String>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut names: Vec<String> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].trim() == "#[component]" {
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len() && lines[j].trim().starts_with("pub fn ") {
                let after_pub_fn = &lines[j].trim()["pub fn ".len()..];
                let name = after_pub_fn
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if !name.is_empty() {
                    names.push(name);
                }
            }
        }
        i += 1;
    }
    let count = names.len();
    (count, names)
}

fn parse_file(content: &str, stem: &str, root: &Path) -> FileDef {
    let lines: Vec<&str> = content.lines().collect();
    let mut enums: Vec<EnumDef> = Vec::new();
    let mut components: Vec<ComponentDef> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Parse multi-line pub enum
        if line.starts_with("pub enum ") {
            let name = line
                .trim_start_matches("pub enum ")
                .split(|c: char| c == '{' || c == ' ')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            // Collect variants until closing brace
            let mut variants: Vec<(String, bool)> = Vec::new();
            let mut pending_default = false;
            i += 1;
            while i < lines.len() {
                let vline = lines[i].trim();
                if vline == "}" {
                    i += 1;
                    break;
                }
                if vline == "#[default]" {
                    pending_default = true;
                } else if !vline.is_empty() && !vline.starts_with("//") && !vline.starts_with("#[")
                {
                    // It's a variant line like "Default," or "Outline,"
                    let vname = vline.trim_end_matches(',').trim().to_string();
                    if !vname.is_empty() {
                        variants.push((vname, pending_default));
                        pending_default = false;
                    }
                } else if vline.starts_with("#[") && vline != "#[default]" {
                    // other attrs — skip, reset pending
                    pending_default = false;
                }
                i += 1;
            }
            if !name.is_empty() {
                enums.push(EnumDef { name, variants });
            }
            continue;
        }

        // Parse #[component] followed by pub fn
        if line == "#[component]" {
            // Find the next non-blank line
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j >= lines.len() {
                i += 1;
                continue;
            }
            let fn_line = lines[j].trim();
            // Only pub fn (skip private fn helpers)
            if !fn_line.starts_with("pub fn ") {
                i += 1;
                continue;
            }
            // Extract function name
            let after_pub_fn = &fn_line["pub fn ".len()..];
            let fn_name = after_pub_fn
                .split('(')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if fn_name.is_empty() {
                i += 1;
                continue;
            }

            // Collect argument lines until `) ->` or `-> impl IntoView`
            let mut props: Vec<PropDef> = Vec::new();
            let mut has_children = false;

            // Fast path: entire signature is on one line — no multi-line args to parse.
            // e.g. `pub fn Foo(children: Children) -> impl IntoView {`
            if fn_line.contains("-> impl") || fn_line.contains(") ->") {
                // Parse args from the fn_line itself (between first `(` and last `)` before `->`)
                let after_open = fn_line.find('(').map(|p| &fn_line[p + 1..]).unwrap_or("");
                let close_paren = after_open
                    .rfind(')')
                    .map(|p| &after_open[..p])
                    .unwrap_or(after_open);
                // Each comma-separated arg
                for arg in close_paren.split(',') {
                    let arg = arg.trim();
                    if arg.is_empty() || arg.starts_with('#') || arg.starts_with("//") {
                        continue;
                    }
                    if let Some(colon_pos) = arg.find(':') {
                        let arg_name = arg[..colon_pos].trim().to_string();
                        let mut arg_type = arg[colon_pos + 1..].trim().to_string();
                        if arg_type.ends_with(',') {
                            arg_type.pop();
                        }
                        let arg_type = arg_type.trim().to_string();
                        if arg_name.is_empty() || arg_type.is_empty() {
                            continue;
                        }
                        if arg_name == "children"
                            && (arg_type == "Children"
                                || arg_type == "ChildrenFn"
                                || arg_type == "ChildrenMaybeSignal")
                        {
                            has_children = true;
                            continue;
                        }
                        props.push(PropDef {
                            name: arg_name,
                            ty: arg_type,
                            notes: "required".to_string(),
                        });
                    }
                }
                components.push(ComponentDef {
                    name: fn_name,
                    props,
                    has_children,
                });
                i = j + 1;
                continue;
            }

            let mut k = j + 1;
            while k < lines.len() {
                let aline = lines[k].trim();
                // Stop at close paren + arrow
                if aline.starts_with(") ->") || aline.starts_with("-> impl") {
                    break;
                }
                // Skip empty lines
                if aline.is_empty() {
                    k += 1;
                    continue;
                }
                // Parse prop line: may contain #[prop(...)] or just a plain arg
                if aline.contains(':') && !aline.starts_with("//") {
                    let (prop_modifier, arg_part) = if aline.contains("#[prop(") {
                        // Extract modifier text between #[prop( and closing )]
                        let start = aline.find("#[prop(").unwrap() + "#[prop(".len();
                        // Find )] — the attribute close. Could have () inside (e.g. String::new())
                        let end = aline[start..]
                            .find(")]")
                            .map(|p| start + p)
                            .unwrap_or(aline.len());
                        let modifier = aline[start..end].trim().to_string();
                        // arg part is after )] followed by space
                        let after = aline
                            .find(")]")
                            .map(|p| &aline[p + 2..])
                            .unwrap_or("")
                            .trim();
                        (Some(modifier), after)
                    } else {
                        (None, aline)
                    };

                    // arg_part should be "name: Type,"
                    if let Some(colon_pos) = arg_part.find(':') {
                        let arg_name = arg_part[..colon_pos].trim().to_string();
                        let mut arg_type = arg_part[colon_pos + 1..].trim().to_string();
                        // Strip trailing comma and optional trailing )
                        if arg_type.ends_with(',') {
                            arg_type.pop();
                        }
                        // If last arg line before `) ->`, may have trailing `)`
                        // but we already stop before `) ->` lines, so this is safe
                        let arg_type = arg_type.trim().to_string();

                        if arg_name.is_empty() || arg_type.is_empty() {
                            k += 1;
                            continue;
                        }

                        // children check
                        if arg_name == "children"
                            && (arg_type == "Children"
                                || arg_type == "ChildrenFn"
                                || arg_type == "ChildrenMaybeSignal")
                        {
                            has_children = true;
                            k += 1;
                            continue;
                        }

                        let notes = match &prop_modifier {
                            Some(m) if m.starts_with("default =") => {
                                let val = m["default =".len()..].trim().to_string();
                                format!("default: `{}`", val)
                            }
                            Some(m) if m.contains("optional") => "optional".to_string(),
                            Some(m) if m.contains("into") => "into".to_string(),
                            Some(m) => m.clone(),
                            None => "required".to_string(),
                        };

                        props.push(PropDef {
                            name: arg_name,
                            ty: arg_type,
                            notes,
                        });
                    }
                }
                k += 1;
            }

            components.push(ComponentDef {
                name: fn_name,
                props,
                has_children,
            });
            i = k;
            continue;
        }

        i += 1;
    }

    // Check playground page
    let page_rel = format!("playground/src/pages/{}_page.rs", stem);
    let page_path = root.join(&page_rel);
    let playground_path = if page_path.exists() {
        Some(page_rel)
    } else {
        None
    };

    FileDef {
        enums,
        components,
        playground_path,
    }
}

/// Walk a module dir that uses category subdirectories (blocks/, charts/, screens/).
/// Returns: (module_name -> Vec<(stem, FileDef)>, count_of_#[component]_in_source, source_names)
fn walk_module_dir(
    dir: &Path,
    root: &Path,
) -> (BTreeMap<String, Vec<(String, FileDef)>>, usize, Vec<String>) {
    let mut catalog: BTreeMap<String, Vec<(String, FileDef)>> = BTreeMap::new();
    let mut source_count = 0usize;
    let mut source_names: Vec<String> = Vec::new();

    if !dir.exists() {
        return (catalog, 0, Vec::new());
    }

    // Each subdir is a "group" (e.g. faq, bar_chart, dashboard)
    let mut groups: Vec<String> = std::fs::read_dir(dir)
        .expect("cannot read module dir")
        .filter_map(|e| {
            let e = e.ok()?;
            if e.file_type().ok()?.is_dir() {
                Some(e.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    groups.sort();

    for group in &groups {
        let group_dir = dir.join(group);
        let mut files: Vec<(String, FileDef)> = std::fs::read_dir(&group_dir)
            .expect("cannot read group dir")
            .filter_map(|e| {
                let e = e.ok()?;
                let path: PathBuf = e.path();
                if path.extension()?.to_str()? != "rs" {
                    return None;
                }
                let stem = path.file_stem()?.to_string_lossy().to_string();
                if stem == "mod" || stem == "shared" {
                    return None;
                }
                let content = std::fs::read_to_string(&path).unwrap_or_default();
                // Collect pub #[component] fn names for parity check
                let (cnt, mut names) = scan_pub_components(&content);
                source_count += cnt;
                source_names.append(&mut names);
                let def = parse_file(&content, &stem, root);
                Some((stem, def))
            })
            .collect();
        files.sort_by(|a, b| a.0.cmp(&b.0));
        catalog.insert(group.clone(), files);
    }

    (catalog, source_count, source_names)
}

/// Walk a flat module dir (hooks/) for `pub fn use_*` functions.
fn parse_hooks(dir: &Path) -> (Vec<HookDef>, usize) {
    let mut hooks: Vec<HookDef> = Vec::new();
    let mut source_count = 0usize;

    if !dir.exists() {
        return (Vec::new(), 0);
    }
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("cannot read hooks dir")
        .filter_map(|e| {
            let e = e.ok()?;
            let path = e.path();
            if path.extension()?.to_str()? != "rs" {
                return None;
            }
            let stem = path.file_stem()?.to_string_lossy().to_string();
            if stem == "mod" {
                return None;
            }
            Some(path)
        })
        .collect();
    files.sort();

    for path in &files {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let lines: Vec<&str> = content.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            // Match `pub fn use_*` (not preceded by #[component])
            if trimmed.starts_with("pub fn use_") {
                source_count += 1;
                let fn_name = trimmed["pub fn ".len()..]
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if fn_name.is_empty() {
                    continue;
                }

                // Collect params: everything from `(` to `)` on same or following lines
                let paren_start = match trimmed.find('(') {
                    Some(p) => p,
                    None => continue,
                };
                let rest = &trimmed[paren_start + 1..];

                // Try to find the closing `)` — may span multiple lines
                let mut param_buf = rest.to_string();
                let mut j = i + 1;
                while !param_buf.contains(')') && j < lines.len() {
                    param_buf.push(' ');
                    param_buf.push_str(lines[j].trim());
                    j += 1;
                }
                let params = if let Some(end) = param_buf.find(')') {
                    param_buf[..end].trim().to_string()
                } else {
                    param_buf.trim().to_string()
                };

                // Collect return type: find `->` after `)`
                let mut sig_rest = if let Some(end) = param_buf.find(')') {
                    param_buf[end + 1..].to_string()
                } else {
                    String::new()
                };
                // If `->` not yet in sig_rest, keep reading
                while !sig_rest.contains("->") && j < lines.len() {
                    sig_rest.push(' ');
                    sig_rest.push_str(lines[j].trim());
                    j += 1;
                }
                // Find `->` and take until `{`
                let ret = if let Some(arrow) = sig_rest.find("->") {
                    let after_arrow = sig_rest[arrow + 2..].trim();
                    // Stop at `{` or end of line
                    let end = after_arrow.find('{').unwrap_or(after_arrow.len());
                    after_arrow[..end].trim().to_string()
                } else {
                    String::new()
                };

                hooks.push(HookDef {
                    name: fn_name,
                    params,
                    ret,
                });
            }
        }
    }

    (hooks, source_count)
}

fn emit_module_section(
    out: &mut String,
    section_title: &str,
    catalog: &BTreeMap<String, Vec<(String, FileDef)>>,
    total_emitted: &mut usize,
) {
    let has_any = catalog
        .values()
        .any(|files| files.iter().any(|(_, def)| !def.components.is_empty()));
    if !has_any {
        return;
    }

    out.push_str(&format!("## {}\n\n", section_title));

    for (group, files) in catalog {
        let group_title = to_title_case(group);
        let has_group = files.iter().any(|(_, def)| !def.components.is_empty());
        if !has_group {
            continue;
        }
        out.push_str(&format!("### {}\n\n", group_title));

        for (_stem, def) in files {
            if def.components.is_empty() {
                continue;
            }

            for comp in &def.components {
                *total_emitted += 1;
                out.push_str(&format!("#### {}\n\n", comp.name));
                out.push_str(&format!("**Import:** `use soma_ui::{};`\n\n", comp.name));

                let is_first_comp = def.components.first().map(|c| &c.name) == Some(&comp.name);
                if is_first_comp && !def.enums.is_empty() {
                    out.push_str("**Enums:**\n");
                    for e in &def.enums {
                        let variants_str: String = e
                            .variants
                            .iter()
                            .map(|(v, is_def)| {
                                if *is_def {
                                    format!("{}*", v)
                                } else {
                                    v.clone()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        out.push_str(&format!("- `{}`: {}\n", e.name, variants_str));
                    }
                    out.push_str("\n");
                }

                let has_props = !comp.props.is_empty();
                if has_props || comp.has_children {
                    out.push_str("**Props:**\n\n");
                    out.push_str("| Prop | Type | Notes |\n");
                    out.push_str("|------|------|-------|\n");
                    for p in &comp.props {
                        out.push_str(&format!("| {} | {} | {} |\n", p.name, p.ty, p.notes));
                    }
                    if comp.has_children {
                        out.push_str("| + children | Children | required |\n");
                    }
                    out.push_str("\n");
                }

                if let Some(pp) = &def.playground_path {
                    out.push_str(&format!("**Playground:** `{}`\n\n", pp));
                }
            }
        }
    }
}

fn main() {
    let root = std::env::current_dir().expect("cannot get cwd");
    let components_dir = root.join("packages/ui/src/components");
    let blocks_dir = root.join("packages/ui/src/blocks");
    let charts_dir = root.join("packages/ui/src/charts");
    let screens_dir = root.join("packages/ui/src/screens");
    let hooks_dir = root.join("packages/ui/src/hooks");

    // ── Components (category subdirs) ──────────────────────────────────────
    let mut categories: Vec<String> = std::fs::read_dir(&components_dir)
        .expect("cannot read components dir")
        .filter_map(|e| {
            let e = e.ok()?;
            if e.file_type().ok()?.is_dir() {
                Some(e.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    categories.sort();

    let mut components_catalog: BTreeMap<String, Vec<(String, FileDef)>> = BTreeMap::new();
    let mut components_source_count = 0usize;
    let mut components_source_names: Vec<String> = Vec::new();

    for cat in &categories {
        let cat_dir = components_dir.join(cat);
        let mut files: Vec<(String, FileDef)> = std::fs::read_dir(&cat_dir)
            .expect("cannot read category dir")
            .filter_map(|e| {
                let e = e.ok()?;
                let path: PathBuf = e.path();
                if path.extension()?.to_str()? != "rs" {
                    return None;
                }
                let stem = path.file_stem()?.to_string_lossy().to_string();
                if stem == "mod" || stem == "shared" {
                    return None;
                }
                let content = std::fs::read_to_string(&path).unwrap_or_default();
                let (cnt, mut names) = scan_pub_components(&content);
                components_source_count += cnt;
                components_source_names.append(&mut names);
                let def = parse_file(&content, &stem, &root);
                Some((stem, def))
            })
            .collect();
        files.sort_by(|a, b| a.0.cmp(&b.0));
        components_catalog.insert(cat.clone(), files);
    }

    // ── Blocks / Charts / Screens ──────────────────────────────────────────
    let (blocks_catalog, blocks_source_count, blocks_source_names) =
        walk_module_dir(&blocks_dir, &root);
    let (charts_catalog, charts_source_count, charts_source_names) =
        walk_module_dir(&charts_dir, &root);
    let (screens_catalog, screens_source_count, screens_source_names) =
        walk_module_dir(&screens_dir, &root);

    // ── Hooks ──────────────────────────────────────────────────────────────
    let (hooks, hooks_source_count) = parse_hooks(&hooks_dir);

    // ── Build output ───────────────────────────────────────────────────────
    let mut out = String::new();
    out.push_str("<!-- AUTO-GENERATED by tools/catalog — do not edit by hand. Run: cargo run -q -p catalog -->\n\n");
    out.push_str("# soma-ui Component Catalog\n\n");
    out.push_str("> Discovery surface for AI agents. Import any component as `use soma_ui::ComponentName;`\n\n");

    let mut total_components_emitted = 0usize;
    let mut total_categories = 0usize;

    // Emit Components section (original style: H2 category, H3 component)
    for (cat, files) in &components_catalog {
        let cat_title = to_title_case(cat);
        let has_any = files.iter().any(|(_, def)| !def.components.is_empty());
        if !has_any {
            continue;
        }
        total_categories += 1;
        out.push_str(&format!("## {}\n\n", cat_title));

        for (_stem, def) in files {
            if def.components.is_empty() {
                continue;
            }

            for comp in &def.components {
                total_components_emitted += 1;
                out.push_str(&format!("### {}\n\n", comp.name));
                out.push_str(&format!("**Import:** `use soma_ui::{};`\n\n", comp.name));

                let is_first_comp = def.components.first().map(|c| &c.name) == Some(&comp.name);
                if is_first_comp && !def.enums.is_empty() {
                    out.push_str("**Enums:**\n");
                    for e in &def.enums {
                        let variants_str: String = e
                            .variants
                            .iter()
                            .map(|(v, is_def)| {
                                if *is_def {
                                    format!("{}*", v)
                                } else {
                                    v.clone()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        out.push_str(&format!("- `{}`: {}\n", e.name, variants_str));
                    }
                    out.push_str("\n");
                }

                let has_props = !comp.props.is_empty();
                if has_props || comp.has_children {
                    out.push_str("**Props:**\n\n");
                    out.push_str("| Prop | Type | Notes |\n");
                    out.push_str("|------|------|-------|\n");
                    for p in &comp.props {
                        out.push_str(&format!("| {} | {} | {} |\n", p.name, p.ty, p.notes));
                    }
                    if comp.has_children {
                        out.push_str("| + children | Children | required |\n");
                    }
                    out.push_str("\n");
                }

                if let Some(pp) = &def.playground_path {
                    out.push_str(&format!("**Playground:** `{}`\n\n", pp));
                }
            }
        }
    }

    // Blocks, Charts, Screens use H2 section + H3 group + H4 component
    emit_module_section(
        &mut out,
        "Blocks",
        &blocks_catalog,
        &mut total_components_emitted,
    );
    emit_module_section(
        &mut out,
        "Charts",
        &charts_catalog,
        &mut total_components_emitted,
    );
    emit_module_section(
        &mut out,
        "Screens",
        &screens_catalog,
        &mut total_components_emitted,
    );

    // Hooks section
    let hooks_emitted = hooks.len();
    if !hooks.is_empty() {
        out.push_str("## Hooks\n\n");
        out.push_str("> Import as `use soma_ui::use_hook_name;`\n\n");
        out.push_str("| Hook | Params | Returns |\n");
        out.push_str("|------|--------|---------|\n");
        for h in &hooks {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` |\n",
                h.name, h.params, h.ret
            ));
        }
        out.push_str("\n");
    }

    // ── Parity tripwire (T3) ───────────────────────────────────────────────
    let total_source =
        components_source_count + blocks_source_count + charts_source_count + screens_source_count;
    let total_emitted = total_components_emitted;

    if total_emitted != total_source {
        // Build the set of names actually emitted
        let mut emitted_set: std::collections::HashSet<String> = std::collections::HashSet::new();
        for (_, files) in &components_catalog {
            for (_, def) in files {
                for c in &def.components {
                    emitted_set.insert(c.name.clone());
                }
            }
        }
        for catalog in [&blocks_catalog, &charts_catalog, &screens_catalog] {
            for (_, files) in catalog {
                for (_, def) in files {
                    for c in &def.components {
                        emitted_set.insert(c.name.clone());
                    }
                }
            }
        }
        // Build the set of names found in source
        let mut all_source_names: Vec<String> = components_source_names;
        all_source_names.extend(blocks_source_names);
        all_source_names.extend(charts_source_names);
        all_source_names.extend(screens_source_names);
        let missing: Vec<&String> = all_source_names
            .iter()
            .filter(|n| !emitted_set.contains(*n))
            .collect();
        eprintln!(
            "catalog ERROR: source has {} #[component] occurrences but emitted {} entries",
            total_source, total_emitted
        );
        if !missing.is_empty() {
            eprintln!(
                "catalog: missing from catalog: {}",
                missing
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        std::process::exit(1);
    }

    // Hooks parity
    if hooks_emitted != hooks_source_count {
        eprintln!(
            "catalog ERROR: source has {} pub fn use_* but emitted {} hook entries",
            hooks_source_count, hooks_emitted
        );
        std::process::exit(1);
    }

    let out_path = root.join("COMPONENTS.md");
    std::fs::write(&out_path, &out).expect("cannot write COMPONENTS.md");
    eprintln!(
        "catalog: wrote {} ({} components [+{} blocks +{} charts +{} screens] + {} hooks across {} categories)",
        out_path.display(),
        components_source_count,
        blocks_source_count,
        charts_source_count,
        screens_source_count,
        hooks_source_count,
        total_categories
    );
}
