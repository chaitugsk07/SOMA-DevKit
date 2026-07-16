import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaTooltip extends StatelessWidget {
  final String message;
  final Widget child;

  const SomaTooltip({super.key, required this.message, required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Tooltip(
      message: message,
      decoration: BoxDecoration(
        color: c.card,
        borderRadius: BorderRadius.circular(6),
        border: Border.all(color: c.border),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(12),
            blurRadius: 6,
          ),
          BoxShadow(
            color: Colors.black.withAlpha(18),
            blurRadius: 12,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      textStyle: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 12,
        color: c.foreground,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
      child: child,
    );
  }
}
