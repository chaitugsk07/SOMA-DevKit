import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaShimmer extends StatefulWidget {
  final double width;
  final double height;
  final double borderRadius;

  const SomaShimmer({
    super.key,
    this.width = double.infinity,
    this.height = 16,
    this.borderRadius = 6,
  });

  @override
  State<SomaShimmer> createState() => _SomaShimmerState();
}

class _SomaShimmerState extends State<SomaShimmer>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1200),
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
    final highlight = Color.alphaBlend(c.background.withAlpha(120), c.muted);

    return AnimatedBuilder(
      animation: _controller,
      builder: (context, _) {
        final t = _controller.value;
        return Container(
          width: widget.width,
          height: widget.height,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(widget.borderRadius),
            gradient: LinearGradient(
              begin: Alignment(-1.5 + t * 3, 0),
              end: Alignment(-0.5 + t * 3, 0),
              colors: [c.muted, highlight, c.muted],
            ),
          ),
        );
      },
    );
  }
}
