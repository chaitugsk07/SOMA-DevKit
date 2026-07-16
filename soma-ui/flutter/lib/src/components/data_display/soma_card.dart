import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaCard extends StatelessWidget {
  final Widget? child;
  final EdgeInsetsGeometry? padding;

  const SomaCard({super.key, this.child, this.padding});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      padding: padding,
      decoration: BoxDecoration(
        color: c.card,
        border: Border.all(color: c.border),
        borderRadius: BorderRadius.circular(8),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 8, offset: const Offset(0, 2)),
          BoxShadow(color: Colors.black.withAlpha(5), blurRadius: 2, offset: const Offset(0, 1)),
        ],
      ),
      child: DefaultTextStyle(
        style: TextStyle(color: c.cardForeground, fontFamily: 'Outfit'),
        child: child ?? const SizedBox.shrink(),
      ),
    );
  }
}

class SomaCardHeader extends StatelessWidget {
  final Widget? child;
  final EdgeInsetsGeometry padding;

  const SomaCardHeader({
    super.key,
    this.child,
    this.padding = const EdgeInsets.all(24),
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: padding,
      child: child,
    );
  }
}

class SomaCardTitle extends StatelessWidget {
  final Widget child;

  const SomaCardTitle({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return DefaultTextStyle(
      style: TextStyle(
        fontFamily: 'Rajdhani',
        fontSize: 24,
        fontWeight: FontWeight.w600,
        color: c.cardForeground,
        height: 1.0,
        letterSpacing: -0.3,
      ),
      child: child,
    );
  }
}

class SomaCardDescription extends StatelessWidget {
  final Widget child;

  const SomaCardDescription({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return DefaultTextStyle(
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 14,
        color: c.mutedForeground,
      ),
      child: child,
    );
  }
}

class SomaCardContent extends StatelessWidget {
  final Widget? child;
  final EdgeInsetsGeometry padding;

  const SomaCardContent({
    super.key,
    this.child,
    this.padding = const EdgeInsets.fromLTRB(24, 0, 24, 24),
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: padding,
      child: child,
    );
  }
}

class SomaCardFooter extends StatelessWidget {
  final Widget? child;
  final EdgeInsetsGeometry padding;

  const SomaCardFooter({
    super.key,
    this.child,
    this.padding = const EdgeInsets.fromLTRB(24, 0, 24, 24),
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: padding,
      child: child,
    );
  }
}
