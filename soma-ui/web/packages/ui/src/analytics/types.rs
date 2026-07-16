use serde::Deserialize;

/// A single column descriptor from a soma-analytics ResultSet.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SomaColumn {
    pub name: String,
    pub data_type: String,
}

/// A ResultSet returned by soma-analytics `/api/v1/query`.
/// Mirrors the wire format so it can be deserialised from the response JSON.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct SomaResultSet {
    pub columns: Vec<SomaColumn>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

/// A panel definition for the Dashboard component.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PanelDef {
    pub title: String,
    pub chart_type: String,
    pub result: SomaResultSet,
}
