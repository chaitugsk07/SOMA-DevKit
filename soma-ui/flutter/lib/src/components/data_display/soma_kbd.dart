import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaKbd extends StatelessWidget {
  final Widget child;

  const SomaKbd({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 3),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: [
            Color.alphaBlend(Colors.white.withAlpha(20), c.muted),
            c.muted,
          ],
        ),
        borderRadius: BorderRadius.circular(4),
        border: Border.all(color: c.border),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(18), blurRadius: 0, offset: const Offset(0, 2), spreadRadius: 0),
          BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 3, offset: const Offset(0, 1)),
        ],
      ),
      child: DefaultTextStyle(
        style: TextStyle(
          fontFamily: 'Rajdhani',
          fontSize: 12,
          fontWeight: FontWeight.w600,
          letterSpacing: 0.5,
          color: c.foreground,
        ),
        child: child,
      ),
    );
  }
}
