import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

enum SomaBadgeVariant { primary, secondary, destructive, outline, success }

class SomaBadge extends StatelessWidget {
  final SomaBadgeVariant variant;
  final Widget child;

  const SomaBadge({
    super.key,
    this.variant = SomaBadgeVariant.primary,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    Color bg;
    Color fg;
    Color borderColor;

    switch (variant) {
      case SomaBadgeVariant.primary:
        bg = c.primary;
        fg = c.primaryForeground;
        borderColor = Colors.transparent;
      case SomaBadgeVariant.secondary:
        bg = c.secondary;
        fg = c.secondaryForeground;
        borderColor = Colors.transparent;
      case SomaBadgeVariant.destructive:
        bg = c.destructive;
        fg = c.destructiveForeground;
        borderColor = Colors.transparent;
      case SomaBadgeVariant.outline:
        bg = Colors.transparent;
        fg = c.foreground;
        borderColor = c.border;
      case SomaBadgeVariant.success:
        bg = c.success;
        fg = c.successForeground;
        borderColor = Colors.transparent;
    }

    final bool isFilled = variant != SomaBadgeVariant.outline;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 3),
      decoration: BoxDecoration(
        // For filled variants: gradient with top-edge highlight; outline: flat transparent
        gradient: isFilled
            ? LinearGradient(
                begin: Alignment.topCenter,
                end: Alignment.bottomCenter,
                colors: [
                  Color.alphaBlend(Colors.white.withAlpha(22), bg),
                  bg,
                ],
              )
            : null,
        color: isFilled ? null : bg,
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: borderColor),
        boxShadow: isFilled
            ? [BoxShadow(color: Colors.black.withAlpha(18), blurRadius: 4, offset: const Offset(0, 1))]
            : null,
      ),
      child: DefaultTextStyle(
        style: TextStyle(
          fontFamily: 'Rajdhani',
          fontSize: 12,
          fontWeight: FontWeight.w600,
          letterSpacing: 0.5,
          color: fg,
        ),
        child: child,
      ),
    );
  }
}
