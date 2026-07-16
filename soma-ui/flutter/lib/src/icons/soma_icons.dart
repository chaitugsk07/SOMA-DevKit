// Lucide icon shim — mirrors the web crate's icon surface.
//
// The web soma-ui uses Lucide icons via the `icondata` crate ("lucide" feature)
// with `leptos_icons`. This shim re-exports the Flutter Lucide package so both
// platforms share the same icon vocabulary (e.g. `LucideIcons.chevronDown`,
// `LucideIcons.check`, `LucideIcons.sun`, `LucideIcons.moon`, `LucideIcons.x`).
export 'package:lucide_icons_flutter/lucide_icons.dart';
