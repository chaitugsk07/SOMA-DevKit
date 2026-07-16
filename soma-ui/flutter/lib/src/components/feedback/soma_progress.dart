import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaProgress extends StatefulWidget {
  /// Value between 0.0 and 1.0.
  final double value;

  const SomaProgress({
    super.key,
    required this.value,
  });

  @override
  State<SomaProgress> createState() => _SomaProgressState();
}

class _SomaProgressState extends State<SomaProgress>
    with SingleTickerProviderStateMixin {
  late AnimationController _ctrl;
  late Animation<double> _anim;

  @override
  void initState() {
    super.initState();
    _ctrl = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 500),
    );
    _anim = Tween<double>(begin: 0, end: widget.value.clamp(0.0, 1.0))
        .animate(CurvedAnimation(parent: _ctrl, curve: Curves.easeOutCubic));
    _ctrl.forward();
  }

  @override
  void didUpdateWidget(SomaProgress old) {
    super.didUpdateWidget(old);
    if (old.value != widget.value) {
      _anim = Tween<double>(begin: _anim.value, end: widget.value.clamp(0.0, 1.0))
          .animate(CurvedAnimation(parent: _ctrl, curve: Curves.easeOutCubic));
      _ctrl
        ..reset()
        ..forward();
    }
  }

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return ClipRRect(
      borderRadius: BorderRadius.circular(99),
      child: Container(
        height: 8,
        decoration: BoxDecoration(
          color: c.secondary,
          borderRadius: BorderRadius.circular(99),
          boxShadow: [
            BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 2, offset: const Offset(0, 1)),
          ],
        ),
        child: AnimatedBuilder(
          animation: _anim,
          builder: (context, _) {
            return FractionallySizedBox(
              widthFactor: _anim.value,
              alignment: Alignment.centerLeft,
              child: Stack(
                children: [
                  // Fill
                  Container(
                    decoration: BoxDecoration(
                      color: c.primary,
                      borderRadius: BorderRadius.circular(99),
                    ),
                  ),
                  // Sheen — thin white highlight at top edge
                  Positioned(
                    top: 0,
                    left: 0,
                    right: 0,
                    height: 3,
                    child: Container(
                      decoration: BoxDecoration(
                        borderRadius: const BorderRadius.vertical(top: Radius.circular(99)),
                        gradient: LinearGradient(
                          begin: Alignment.topCenter,
                          end: Alignment.bottomCenter,
                          colors: [
                            Colors.white.withAlpha(50),
                            Colors.white.withAlpha(0),
                          ],
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            );
          },
        ),
      ),
    );
  }
}
