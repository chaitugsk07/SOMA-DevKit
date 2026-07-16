import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

// Shared title/description/footer widget bodies for drawer and sheet panels.
// Headers differ per panel type (drag handle vs X button) and remain per-file.

// Fade + scale entrance animation for floating overlay panels.
class FadeScaleIn extends StatefulWidget {
  final Widget child;
  final Duration duration;

  const FadeScaleIn({
    super.key,
    required this.child,
    this.duration = const Duration(milliseconds: 140),
  });

  @override
  State<FadeScaleIn> createState() => _FadeScaleInState();
}

class _FadeScaleInState extends State<FadeScaleIn>
    with SingleTickerProviderStateMixin {
  late final AnimationController _ctrl;
  late final Animation<double> _anim;

  @override
  void initState() {
    super.initState();
    _ctrl = AnimationController(vsync: this, duration: widget.duration);
    _anim = CurvedAnimation(parent: _ctrl, curve: Curves.easeOutCubic);
    _ctrl.forward();
  }

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => FadeTransition(
        opacity: _anim,
        child: ScaleTransition(
          scale: Tween(begin: 0.95, end: 1.0).animate(_anim),
          child: widget.child,
        ),
      );
}

class PanelTitle extends StatelessWidget {
  final String text;

  const PanelTitle({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Text(
      text,
      style: TextStyle(
        fontFamily: 'Rajdhani',
        fontSize: 20,
        fontWeight: FontWeight.w600,
        color: c.cardForeground,
        height: 1.0,
      ),
    );
  }
}

class PanelDescription extends StatelessWidget {
  final String text;

  const PanelDescription({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Text(
      text,
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 14,
        color: c.mutedForeground,
      ),
    );
  }
}

class PanelFooter extends StatelessWidget {
  final List<Widget> children;

  const PanelFooter({super.key, required this.children});

  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.only(top: 16),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.end,
          children: children
              .map(
                (w) => Padding(
                  padding: const EdgeInsets.only(left: 8),
                  child: w,
                ),
              )
              .toList(),
        ),
      );
}
