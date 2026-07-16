use crate::ui::*;
use leptos::prelude::*;
use soma_ui::CodeBlock;

#[component]
pub fn CodeBlockPage() -> impl IntoView {
    let sql = "CREATE TABLE users (\n  id SERIAL PRIMARY KEY,\n  email TEXT NOT NULL UNIQUE,\n  created_at TIMESTAMPTZ DEFAULT NOW()\n);\n\nCREATE INDEX ON users (email);";

    view! {
        <PageShell
            title=Signal::derive(|| "Code Block".to_string())
            subtitle=Signal::derive(|| "Monospace code display with optional SQL highlighting and copy button.".to_string())
        >
            <PreviewPanel>
                <CodeBlock
                    code=sql.to_string()
                    language="sql".to_string()
                    filename="migration.sql".to_string()
                    class="w-full max-w-xl".to_string()
                />
            </PreviewPanel>

            <div class="bg-card border border-border rounded-md p-6 space-y-4">
                <h2 class="text-sm font-semibold text-foreground">"Plain text (no language)"</h2>
                <CodeBlock
                    code="cargo build --release\n./target/release/soma-schema up".to_string()
                />
            </div>
        </PageShell>
    }
}
