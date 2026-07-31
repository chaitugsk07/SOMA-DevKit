use leptos::prelude::*;

/// A minimal terminal chrome: one-line title bar with host path, no traffic-light
/// dots. Children are the content rendered inside the frame.
#[component]
pub fn TerminalFrame(
    /// Displayed in the title bar, e.g. "sri@prod:~/soma"
    #[prop(optional, into)]
    host: String,
    children: Children,
) -> impl IntoView {
    let host_str = if host.is_empty() {
        "sri@prod:~/soma".to_string()
    } else {
        host
    };

    view! {
        <div class="terminal-frame" role="region" aria-label="Terminal output">
            <div class="terminal-frame__title" aria-hidden="true">
                <span class="terminal-frame__tick">"╭─"</span>
                <span class="terminal-frame__host">{host_str}</span>
                <span class="terminal-frame__rule"></span>
            </div>
            <div class="terminal-frame__body">
                {children()}
            </div>
        </div>
    }
}

/// A single prompt line: `$ command` followed by optional output lines.
#[component]
pub fn PromptLine(
    /// The command shown after `$ `
    #[prop(into)]
    command: String,
    /// Optional output lines rendered below the command
    #[prop(optional)]
    output: Option<Vec<&'static str>>,
) -> impl IntoView {
    view! {
        <div class="prompt-line">
            <div class="prompt-line__cmd">
                <span class="prompt-ps" aria-hidden="true">"$ "</span>
                <span class="prompt-command">{command}</span>
            </div>
            {output.map(|lines| {
                view! {
                    <div class="prompt-output">
                        {lines.into_iter().map(|l| view! {
                            <div class="prompt-output__line">{l}</div>
                        }).collect_view()}
                    </div>
                }
            })}
        </div>
    }
}
