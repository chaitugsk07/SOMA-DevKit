import 'package:flutter/material.dart';
import '../../icons/soma_icons.dart';
import '../inputs/soma_button.dart';

/// Controlled light/dark theme toggle button.
///
/// Mirrors the web soma-ui `ThemeToggle` in
/// `packages/ui/src/components/interaction/theme_toggle.rs`:
/// shows a sun icon when dark, moon icon when light, and calls [onChanged]
/// with the flipped value on tap.
class SomaThemeToggle extends StatelessWidget {
  final bool isDark;
  final ValueChanged<bool> onChanged;

  const SomaThemeToggle({
    super.key,
    required this.isDark,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return SomaButton(
      variant: SomaButtonVariant.outline,
      size: SomaButtonSize.icon,
      onPressed: () => onChanged(!isDark),
      child: Icon(isDark ? LucideIcons.sun : LucideIcons.moon, size: 16),
    );
  }
}
