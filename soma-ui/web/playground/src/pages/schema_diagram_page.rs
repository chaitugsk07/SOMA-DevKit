use crate::ui::*;
use leptos::prelude::*;
use soma_ui::{SchemaColumn, SchemaDiagram, SchemaRelation, SchemaTable};

#[component]
pub fn SchemaDiagramPage() -> impl IntoView {
    let tables = vec![
        SchemaTable {
            name: "users".to_string(),
            schema: Some("public".to_string()),
            x: 40.0,
            y: 40.0,
            columns: vec![
                SchemaColumn {
                    name: "id".to_string(),
                    col_type: "uuid".to_string(),
                    pk: true,
                    fk: false,
                    nullable: false,
                    unique: true,
                },
                SchemaColumn {
                    name: "email".to_string(),
                    col_type: "text".to_string(),
                    pk: false,
                    fk: false,
                    nullable: false,
                    unique: true,
                },
                SchemaColumn {
                    name: "name".to_string(),
                    col_type: "text".to_string(),
                    pk: false,
                    fk: false,
                    nullable: true,
                    unique: false,
                },
                SchemaColumn {
                    name: "created_at".to_string(),
                    col_type: "timestamptz".to_string(),
                    pk: false,
                    fk: false,
                    nullable: false,
                    unique: false,
                },
            ],
        },
        SchemaTable {
            name: "posts".to_string(),
            schema: Some("public".to_string()),
            x: 380.0,
            y: 40.0,
            columns: vec![
                SchemaColumn {
                    name: "id".to_string(),
                    col_type: "uuid".to_string(),
                    pk: true,
                    fk: false,
                    nullable: false,
                    unique: true,
                },
                SchemaColumn {
                    name: "user_id".to_string(),
                    col_type: "uuid".to_string(),
                    pk: false,
                    fk: true,
                    nullable: false,
                    unique: false,
                },
                SchemaColumn {
                    name: "title".to_string(),
                    col_type: "text".to_string(),
                    pk: false,
                    fk: false,
                    nullable: false,
                    unique: false,
                },
                SchemaColumn {
                    name: "body".to_string(),
                    col_type: "text".to_string(),
                    pk: false,
                    fk: false,
                    nullable: true,
                    unique: false,
                },
            ],
        },
        SchemaTable {
            name: "comments".to_string(),
            schema: Some("public".to_string()),
            x: 380.0,
            y: 280.0,
            columns: vec![
                SchemaColumn {
                    name: "id".to_string(),
                    col_type: "uuid".to_string(),
                    pk: true,
                    fk: false,
                    nullable: false,
                    unique: true,
                },
                SchemaColumn {
                    name: "post_id".to_string(),
                    col_type: "uuid".to_string(),
                    pk: false,
                    fk: true,
                    nullable: false,
                    unique: false,
                },
                SchemaColumn {
                    name: "user_id".to_string(),
                    col_type: "uuid".to_string(),
                    pk: false,
                    fk: true,
                    nullable: false,
                    unique: false,
                },
                SchemaColumn {
                    name: "body".to_string(),
                    col_type: "text".to_string(),
                    pk: false,
                    fk: false,
                    nullable: false,
                    unique: false,
                },
            ],
        },
        SchemaTable {
            name: "tags".to_string(),
            schema: Some("public".to_string()),
            x: 40.0,
            y: 280.0,
            columns: vec![
                SchemaColumn {
                    name: "id".to_string(),
                    col_type: "uuid".to_string(),
                    pk: true,
                    fk: false,
                    nullable: false,
                    unique: true,
                },
                SchemaColumn {
                    name: "name".to_string(),
                    col_type: "text".to_string(),
                    pk: false,
                    fk: false,
                    nullable: false,
                    unique: true,
                },
            ],
        },
    ];
    let relations = vec![
        SchemaRelation {
            from_table: "posts".to_string(),
            from_column: "user_id".to_string(),
            to_table: "users".to_string(),
            to_column: "id".to_string(),
        },
        SchemaRelation {
            from_table: "comments".to_string(),
            from_column: "post_id".to_string(),
            to_table: "posts".to_string(),
            to_column: "id".to_string(),
        },
        SchemaRelation {
            from_table: "comments".to_string(),
            from_column: "user_id".to_string(),
            to_table: "users".to_string(),
            to_column: "id".to_string(),
        },
    ];

    view! {
        <PageShell
            title=Signal::derive(move || "Schema Diagram".to_string())
            subtitle=Signal::derive(move || "Interactive ERD canvas. Drag table headers to reposition, scroll to zoom, drag canvas background to pan.".to_string())
        >
            <SchemaDiagram tables=tables relations=relations />
        </PageShell>
    }
}
