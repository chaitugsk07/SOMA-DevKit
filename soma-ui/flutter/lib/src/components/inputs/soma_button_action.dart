import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

/// Press-and-hold button. Hold for [durationMs] milliseconds to fire [onAction].
/// Shows a primary@20% fill overlay growing left→right while held.
class SomaButtonAction extends StatefulWidget {
  final VoidCallback onAction;
  final int durationMs;
  final Widget child;

  const SomaButtonAction({
    super.key,
    required this.onAction,
    this.durationMs = 1000,
    required this.child,
  });

  @override
  State<SomaButtonAction> createState() => _SomaButtonActionState();
}

class _SomaButtonActionState extends State<SomaButtonAction>
    with SingleTickerProviderStateMixin {
  late AnimationController _ctrl;
  bool _hovered = false;
  bool _focused = false;

  @override
  void initState() {
    super.initState();
    _ctrl = AnimationController(
      vsync: this,
      duration: Duration(milliseconds: widget.durationMs),
    )..addStatusListener((status) {
        if (status == AnimationStatus.completed) {
          _ctrl.reset();
          widget.onAction();
        }
      });
  }

  @override
  void didUpdateWidget(SomaButtonAction old) {
    super.didUpdateWidget(old);
    if (old.durationMs != widget.durationMs) {
      _ctrl.duration = Duration(milliseconds: widget.durationMs);
    }
  }

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  void _start() => _ctrl.forward();

  void _cancel() {
    _ctrl.reverse();
    // Reset after reverse finishes so next press starts clean.
    _ctrl.addStatusListener(_clearOnDismissed);
  }

  void _clearOnDismissed(AnimationStatus s) {
    if (s == AnimationStatus.dismissed) {
      _ctrl.removeStatusListener(_clearOnDismissed);
      _ctrl.reset();
    }
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Focus(
      onFocusChange: (f) => setState(() => _focused = f),
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) {
          setState(() => _hovered = false);
          _cancel();
        },
        child: GestureDetector(
          onTapDown: (_) => _start(),
          onTapUp: (_) => _cancel(),
          onTapCancel: _cancel,
          child: AnimatedBuilder(
            animation: _ctrl,
            builder: (context, child) {
              final baseColor = _hovered
                  ? Color.fromARGB(
                      255,
                      (c.primary.r * 0.9).round(),
                      (c.primary.g * 0.9).round(),
                      (c.primary.b * 0.9).round(),
                    )
                  : c.primary;
              return Container(
                height: 40,
                padding: const EdgeInsets.symmetric(horizontal: 16),
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    begin: Alignment.topCenter,
                    end: Alignment.bottomCenter,
                    colors: [Color.alphaBlend(Colors.white.withAlpha(18), baseColor), baseColor],
                  ),
                  borderRadius: BorderRadius.circular(6),
                  boxShadow: _focused
                      ? [BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 2)]
                      : [BoxShadow(color: Colors.black.withAlpha(14), blurRadius: 8, offset: const Offset(0, 2))],
                ),
                clipBehavior: Clip.antiAlias,
                child: Stack(
                  children: [
                    // Fill overlay growing left→right
                    Positioned.fill(
                      child: FractionallySizedBox(
                        alignment: Alignment.centerLeft,
                        widthFactor: _ctrl.value,
                        child: ColoredBox(color: c.primaryForeground.withAlpha(50)),
                      ),
                    ),
                    // Content
                    Center(
                      child: DefaultTextStyle(
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 14,
                          fontWeight: FontWeight.w500,
                          color: c.primaryForeground,
                        ),
                        child: IconTheme(
                          data: IconThemeData(color: c.primaryForeground, size: 16),
                          child: child!,
                        ),
                      ),
                    ),
                  ],
                ),
              );
            },
            child: widget.child,
          ),
        ),
      ),
    );
  }
}
