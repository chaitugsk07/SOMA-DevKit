// Sonner parity shim — re-exports the unified toast implementation.
// sonner.rs in the web crate is a compact single-message variant of toast.rs;
// Flutter consolidates both into soma_toast.dart (one stacked-overlay system).
export 'soma_toast.dart' show SomaToastVariant, showSomaSonner, showSomaToast;
