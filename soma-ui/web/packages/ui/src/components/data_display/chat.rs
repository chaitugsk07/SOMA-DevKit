use leptos::prelude::*;

#[derive(Clone)]
pub struct ChatMsg {
    pub role: String,
    pub content: String,
    pub timestamp: Option<String>,
}

#[component]
pub fn ChatThread(
    messages: Vec<ChatMsg>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let combined = format!("flex flex-col gap-3 overflow-y-auto {}", class);
    view! {
        <div class=combined>
            {messages.into_iter().map(|msg| {
                view! {
                    <ChatMessage role=msg.role>
                        {msg.content}
                        {msg.timestamp.map(|t| view! {
                            <div class="text-[10px] text-muted-foreground mt-1">{t}</div>
                        })}
                    </ChatMessage>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn ChatMessage(#[prop(into)] role: String, children: Children) -> impl IntoView {
    let is_user = role == "user";
    let wrapper_class = if is_user {
        "flex flex-col items-end self-end"
    } else {
        "flex flex-col items-start self-start"
    };
    let bubble_class = if is_user {
        "bg-foreground text-background rounded-[12px_12px_3px_12px] px-3 py-2 max-w-[85%] text-sm whitespace-pre-wrap"
    } else {
        "bg-muted rounded-[3px_12px_12px_12px] px-3 py-2 max-w-[85%] text-sm whitespace-pre-wrap"
    };
    view! {
        <div class=wrapper_class>
            <span class="text-[9px] uppercase tracking-widest text-muted-foreground">{role}</span>
            <div class=bubble_class>{children()}</div>
        </div>
    }
}
