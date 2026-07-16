use crate::analytics::types::{SomaColumn, SomaResultSet};
use crate::charts::{
    AreaChart, BarChart, ChartPoint, ChartSeries, LineChart, PieChart, RadarChart, RadialChart,
};
use crate::components::{
    Card, CardContent, CardHeader, CardTitle, Table, TableBody, TableCell, TableHead, TableHeader,
    TableRow,
};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Index of the first "category" column (string / time / boolean).
/// Falls back to 0 if none found.
fn category_col_index(columns: &[SomaColumn]) -> usize {
    columns
        .iter()
        .position(|c| matches!(c.data_type.as_str(), "string" | "time" | "boolean"))
        .unwrap_or(0)
}

/// Indices of every "number" column.
fn measure_col_indices(columns: &[SomaColumn]) -> Vec<usize> {
    columns
        .iter()
        .enumerate()
        .filter(|(_, c)| c.data_type == "number")
        .map(|(i, _)| i)
        .collect()
}

/// Convert a `serde_json::Value` row cell to a display string for the x axis.
fn cell_to_label(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Null => "—".into(),
        other => other.to_string(),
    }
}

/// Convert a `serde_json::Value` to f64 (0.0 on failure).
fn cell_to_f64(v: &serde_json::Value) -> f64 {
    v.as_f64().unwrap_or(0.0)
}

/// Build one `ChartSeries` per measure column.
fn build_series(result: &SomaResultSet) -> (Vec<ChartSeries>, Vec<String>) {
    let cat_idx = category_col_index(&result.columns);
    let measure_idxs = measure_col_indices(&result.columns);

    let labels: Vec<String> = result
        .rows
        .iter()
        .map(|row| row.get(cat_idx).map(cell_to_label).unwrap_or_default())
        .collect();

    let series: Vec<ChartSeries> = measure_idxs
        .iter()
        .map(|&mi| {
            let points = result
                .rows
                .iter()
                .enumerate()
                .map(|(ri, row)| ChartPoint {
                    label: labels.get(ri).cloned().unwrap_or_default(),
                    value: row.get(mi).map(cell_to_f64).unwrap_or(0.0),
                })
                .collect();
            ChartSeries { points }
        })
        .collect();

    (series, labels)
}

// ---------------------------------------------------------------------------
// AnalyticsPanel
// ---------------------------------------------------------------------------

#[component]
pub fn AnalyticsPanel(title: String, chart_type: String, result: SomaResultSet) -> impl IntoView {
    let (series, _labels) = build_series(&result);

    // First series' points (used by single-series charts).
    let first_points = series
        .first()
        .cloned()
        .map(|s| s.points)
        .unwrap_or_default();

    let chart_node: AnyView = match chart_type.as_str() {
        "bar" => view! {
            <BarChart data=first_points series=series />
        }
        .into_any(),

        "line" => view! {
            <LineChart data=first_points series=series />
        }
        .into_any(),

        "area" => view! {
            <AreaChart data=first_points series=series />
        }
        .into_any(),

        "pie" => view! {
            <PieChart data=first_points />
        }
        .into_any(),

        "radar" => view! {
            <RadarChart data=first_points series=series />
        }
        .into_any(),

        "radial" => view! {
            <RadialChart data=first_points series=series />
        }
        .into_any(),

        "number" => {
            // Big stat: first measure's first row value.
            let value = first_points
                .first()
                .map(|p| format!("{}", p.value))
                .unwrap_or_else(|| "—".into());
            let label = result
                .columns
                .iter()
                .find(|c| c.data_type == "number")
                .map(|c| c.name.clone())
                .unwrap_or_default();
            view! {
                <div class="flex flex-col items-center justify-center gap-1 py-6">
                    <span class="text-5xl font-semibold tabular-nums text-foreground">
                        {value}
                    </span>
                    <span class="text-sm text-muted-foreground">{label}</span>
                </div>
            }
            .into_any()
        }

        // "table" or anything unknown → table view
        _ => {
            let columns = result.columns.clone();
            let rows = result.rows.clone();

            let header_cells: Vec<_> = columns
                .iter()
                .map(|c| {
                    let name = c.name.clone();
                    view! { <TableHead>{name}</TableHead> }
                })
                .collect();

            let body_rows: Vec<_> = rows
                .iter()
                .map(|row| {
                    let cells: Vec<_> = row
                        .iter()
                        .map(|v| {
                            let text = cell_to_label(v);
                            view! { <TableCell>{text}</TableCell> }
                        })
                        .collect();
                    view! { <TableRow>{cells}</TableRow> }
                })
                .collect();

            view! {
                <Table>
                    <TableHeader>
                        <TableRow>{header_cells}</TableRow>
                    </TableHeader>
                    <TableBody>{body_rows}</TableBody>
                </Table>
            }
            .into_any()
        }
    };

    let title_clone = title.clone();
    view! {
        <Card>
            <CardHeader>
                <CardTitle>{title_clone}</CardTitle>
            </CardHeader>
            <CardContent>
                {chart_node}
            </CardContent>
        </Card>
    }
}
