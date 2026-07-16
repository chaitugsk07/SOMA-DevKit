import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

enum SomaSpinnerSize { sm, md, lg }

class SomaSpinner extends StatefulWidget {
  final SomaSpinnerSize size;

  const SomaSpinner({
    super.key,
    this.size = SomaSpinnerSize.md,
  });

  @override
  State<SomaSpinner> createState() => _SomaSpinnerState();
}

class _SomaSpinnerState extends State<SomaSpinner>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 900),
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final double diameter;
    final double strokeWidth;
    switch (widget.size) {
      case SomaSpinnerSize.sm:
        diameter = 16;
        strokeWidth = 1.5;
      case SomaSpinnerSize.md:
        diameter = 24;
        strokeWidth = 2.0;
      case SomaSpinnerSize.lg:
        diameter = 32;
        strokeWidth = 2.5;
    }

    return RotationTransition(
      turns: _controller,
      child: SizedBox(
        width: diameter,
        height: diameter,
        child: CircularProgressIndicator(
          strokeWidth: strokeWidth,
          strokeCap: StrokeCap.round,
          valueColor: AlwaysStoppedAnimation<Color>(c.primary),
          backgroundColor: c.border,
          value: 0.75,
        ),
      ),
    );
  }
}
