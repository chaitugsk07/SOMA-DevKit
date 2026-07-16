use crate::components::data_display::badge::{Badge, BadgeVariant};
use crate::components::data_display::table::{
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ComparisonCell {
    Text(String),
    Yes,
    No,
}

#[derive(Clone, Debug)]
pub struct ComparisonRow {
    pub label: String,
    pub cells: Vec<ComparisonCell>,
}

#[component]
pub fn ComparisonTable(
    headers: Vec<String>,
    rows: Vec<ComparisonRow>,
    #[prop(optional)] highlight_col: Option<usize>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let header_cells = headers
        .into_iter()
        .enumerate()
        .map(|(i, h)| {
            let is_highlighted = highlight_col == Some(i);
            let extra = if is_highlighted {
                " bg-primary/5 font-bold text-foreground"
            } else {
                ""
            };
            view! { <TableHead class=extra.to_string()>{h}</TableHead> }
        })
        .collect::<Vec<_>>();

    let body_rows = rows
        .into_iter()
        .map(|row| {
            let label = row.label;
            let data_cells = row
                .cells
                .into_iter()
                .enumerate()
                .map(|(i, cell)| {
                    let is_highlighted = highlight_col == Some(i);
                    let extra = if is_highlighted { "bg-primary/5" } else { "" };
                    view! {
                        <TableCell class=extra.to_string()>
                            {match cell {
                                ComparisonCell::Yes => view! {
                                    <Badge variant=BadgeVariant::Success>"✓"</Badge>
                                }.into_any(),
                                ComparisonCell::No => view! {
                                    <span class="text-muted-foreground text-sm">"✗"</span>
                                }.into_any(),
                                ComparisonCell::Text(t) => view! {
                                    <span class="text-sm">{t}</span>
                                }.into_any(),
                            }}
                        </TableCell>
                    }
                })
                .collect::<Vec<_>>();
            view! {
                <TableRow>
                    <TableCell class="font-medium text-foreground".to_string()>{label}</TableCell>
                    {data_cells}
                </TableRow>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <Table class=class>
            <TableHeader>
                <TableRow>
                    <TableHead>"Feature"</TableHead>
                    {header_cells}
                </TableRow>
            </TableHeader>
            <TableBody>
                {body_rows}
            </TableBody>
        </Table>
    }
}
