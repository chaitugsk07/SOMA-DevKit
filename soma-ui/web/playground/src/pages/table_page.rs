use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};

#[derive(Clone)]
struct InvoiceRow {
    id: &'static str,
    status: &'static str,
    method: &'static str,
    amount: &'static str,
}

const ALL_ROWS: &[InvoiceRow] = &[
    InvoiceRow {
        id: "INV-001",
        status: "Paid",
        method: "Credit Card",
        amount: "$250.00",
    },
    InvoiceRow {
        id: "INV-002",
        status: "Pending",
        method: "Bank Transfer",
        amount: "$150.00",
    },
    InvoiceRow {
        id: "INV-003",
        status: "Unpaid",
        method: "PayPal",
        amount: "$350.00",
    },
    InvoiceRow {
        id: "INV-004",
        status: "Paid",
        method: "Credit Card",
        amount: "$450.00",
    },
];

const TOTALS: &[&str] = &["$250.00", "$400.00", "$750.00", "$1,200.00"];

#[component]
pub fn TablePage() -> impl IntoView {
    let show_caption = RwSignal::new(true);
    let show_footer = RwSignal::new(true);
    let row_count = RwSignal::new(4usize);

    view! {
        <PageShell
            title=Signal::derive(|| "Table".to_string())
            subtitle=Signal::derive(|| "Semantic table primitives for structured data display.".to_string())
        >
            // Preview
            <PreviewPanel>
                {move || {
                    let count = row_count.get().min(ALL_ROWS.len());
                    let rows: Vec<InvoiceRow> = ALL_ROWS[..count].to_vec();
                    let total = TOTALS[count.saturating_sub(1)];
                    view! {
                        <Table>
                            {move || show_caption.get().then(|| view! {
                                <TableCaption>"A list of recent invoices."</TableCaption>
                            })}
                            <TableHeader>
                                <TableRow>
                                    <TableHead>"Invoice"</TableHead>
                                    <TableHead>"Status"</TableHead>
                                    <TableHead>"Method"</TableHead>
                                    <TableHead class="text-end".to_string()>"Amount"</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {rows.into_iter().map(|r| view! {
                                    <TableRow>
                                        <TableCell class="font-medium".to_string()>{r.id}</TableCell>
                                        <TableCell>{r.status}</TableCell>
                                        <TableCell>{r.method}</TableCell>
                                        <TableCell class="text-end".to_string()>{r.amount}</TableCell>
                                    </TableRow>
                                }).collect::<Vec<_>>()}
                            </TableBody>
                            {move || show_footer.get().then(|| view! {
                                <TableFooter>
                                    <TableRow>
                                        <TableCell>"Total"</TableCell>
                                        <TableCell>""</TableCell>
                                        <TableCell>""</TableCell>
                                        <TableCell class="text-end".to_string()>{total}</TableCell>
                                    </TableRow>
                                </TableFooter>
                            })}
                        </Table>
                    }
                }}
            </PreviewPanel>

            // Controls
            <ControlsPanel>
                <ControlRow label="Row count">
                    <select
                        class="bg-secondary border border-border rounded-md px-3 py-1.5 text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        on:change=move |e| {
                            let v: usize = event_target_value(&e).parse().unwrap_or(4);
                            row_count.set(v);
                        }
                    >
                        <option value="1">"1"</option>
                        <option value="2">"2"</option>
                        <option value="3">"3"</option>
                        <option value="4" selected>"4"</option>
                    </select>
                </ControlRow>
                <ControlRow label="Show caption">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_caption.get()
                        on:change=move |e| show_caption.set(event_target_checked(&e))
                    />
                </ControlRow>
                <ControlRow label="Show footer">
                    <input
                        type="checkbox"
                        class="w-4 h-4 rounded border-border bg-secondary text-primary focus:ring-ring focus:ring-offset-card"
                        prop:checked=move || show_footer.get()
                        on:change=move |e| show_footer.set(event_target_checked(&e))
                    />
                </ControlRow>
            </ControlsPanel>
        </PageShell>
    }
}
