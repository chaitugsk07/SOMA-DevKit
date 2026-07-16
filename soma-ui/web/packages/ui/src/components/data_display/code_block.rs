use crate::components::layout::scroll_area::ScrollArea;
use crate::hooks::use_clipboard::{use_clipboard, UseClipboardReturn};
use leptos::prelude::*;

// ponytail: regex/keyword highlighter, not a real SQL parser.
// Ceiling: only handles a fixed keyword set; no nesting, strings, or comments.
// Upgrade path: swap in a WASM-based highlighter (e.g. syntect compiled to WASM).
const SQL_KEYWORDS: &[&str] = &[
    "CREATE",
    "TABLE",
    "ALTER",
    "DROP",
    "INSERT",
    "UPDATE",
    "DELETE",
    "SELECT",
    "INDEX",
    "CONSTRAINT",
    "PRIMARY",
    "KEY",
    "FOREIGN",
    "REFERENCES",
    "NOT",
    "NULL",
    "DEFAULT",
    "ON",
    "CONFLICT",
    "DO",
    "NOTHING",
    "SCHEMA",
    "EXTENSION",
    "FUNCTION",
    "TRIGGER",
    "IF",
    "EXISTS",
    "CASCADE",
    "ADD",
    "COLUMN",
    "SET",
    "WHERE",
    "FROM",
    "INTO",
    "VALUES",
    "UNIQUE",
    "CHECK",
    "BEGIN",
    "COMMIT",
    "ROLLBACK",
    "GRANT",
    "REVOKE",
    "WITH",
];

fn highlight_sql(code: &str) -> String {
    // ponytail: O(n * k) scan where k = keyword count (~40). Fine for editor-size snippets.
    let mut result = String::with_capacity(code.len() * 2);
    let mut chars = code.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_alphabetic() || ch == '_' {
            let mut word = String::new();
            word.push(ch);
            while let Some(&next) = chars.peek() {
                if next.is_alphanumeric() || next == '_' {
                    word.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            let upper = word.to_uppercase();
            if SQL_KEYWORDS.contains(&upper.as_str()) {
                result.push_str(&format!(
                    r#"<span class="text-primary font-semibold">{}</span>"#,
                    word
                ));
            } else {
                result.push_str(&word);
            }
        } else {
            match ch {
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '&' => result.push_str("&amp;"),
                _ => result.push(ch),
            }
        }
    }
    result
}

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(into, optional)] language: Option<String>,
    #[prop(into, optional)] filename: Option<String>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let UseClipboardReturn { copy, copied } = use_clipboard();
    let code_clone = code.clone();
    let on_copy = move |_| copy.run(code_clone.clone());

    let is_sql = language
        .as_deref()
        .map(|l| l.eq_ignore_ascii_case("sql"))
        .unwrap_or(false);
    let highlighted = if is_sql {
        highlight_sql(&code)
    } else {
        code.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    };

    let outer = format!(
        "relative rounded-md border border-border bg-muted/30 font-mono text-sm {}",
        class
    );

    view! {
        <div class=outer>
            {filename.map(|f| view! {
                <div class="flex items-center justify-between px-4 py-2 border-b border-border text-xs text-muted-foreground">
                    <span>{f}</span>
                </div>
            })}
            <button
                class="absolute top-2 end-2 p-2 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
                aria-label="Copy code"
                on:click=on_copy
            >
                {move || if copied.get() { "✓" } else { "⧉" }}
            </button>
            <ScrollArea class="max-h-96 p-4 pe-12".to_string()>
                <pre><code inner_html=highlighted /></pre>
            </ScrollArea>
        </div>
    }
}
