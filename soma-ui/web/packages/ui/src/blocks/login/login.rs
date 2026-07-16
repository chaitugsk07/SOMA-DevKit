use crate::components::forms::label::Label;
use crate::components::inputs::button::{Button, ButtonVariant};
use crate::components::inputs::input::Input;
use crate::components::layout::separator::Separator;
use leptos::prelude::*;

#[component]
pub fn LoginBlock() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    view! {
        <div class="min-h-[600px] flex items-center justify-center bg-background p-8">
            <div class="w-full max-w-sm space-y-6">
                <div class="text-center space-y-1">
                    <h1 class="font-heading text-2xl font-bold tracking-tight text-foreground">"Soma UI"</h1>
                    <p class="text-sm text-muted-foreground">"Sign in to your account"</p>
                </div>

                <div class="space-y-4">
                    <div class="space-y-1.5">
                        <Label for_id="email".to_string()>"Email"</Label>
                        <Input input_type="email".to_string() placeholder="you@example.com".to_string() value=email />
                    </div>
                    <div class="space-y-1.5">
                        <Label for_id="password".to_string()>"Password"</Label>
                        <Input input_type="password".to_string() placeholder="••••••••".to_string() value=password />
                    </div>
                    <Button class="w-full".to_string()>"Sign in"</Button>
                </div>

                <div class="flex items-center gap-3">
                    <Separator class="flex-1".to_string() />
                    <span class="text-xs text-muted-foreground">"or"</span>
                    <Separator class="flex-1".to_string() />
                </div>

                <Button variant=ButtonVariant::Outline class="w-full".to_string()>
                    "Continue with GitHub"
                </Button>

                <p class="text-center text-xs text-muted-foreground">
                    "Don't have an account? "
                    <a href="#" class="text-primary hover:underline">"Sign up"</a>
                </p>
            </div>
        </div>
    }
}
