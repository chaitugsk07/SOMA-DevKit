pub fn validate_email(email: String) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn greet(name: String) -> String {
    format!("Hello, {}! (from Rust)", name)
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
