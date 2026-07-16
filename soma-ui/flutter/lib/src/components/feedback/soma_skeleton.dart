import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaSkeleton extends StatefulWidget {
  final double width;
  final double height;
  final double borderRadius;

  const SomaSkeleton({
    super.key,
    this.width = double.infinity,
    this.height = 16,
    this.borderRadius = 6,
  });

  @override
  State<SomaSkeleton> createState() => _SomaSkeletonState();
}

class _SomaSkeletonState extends State<SomaSkeleton>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final Animation<double> _shimmer;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1200),
    )..repeat(reverse: true);
    // Ease in-out so shimmer feels smooth, not mechanical
    _shimmer = CurvedAnimation(parent: _controller, curve: Curves.easeInOut);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return AnimatedBuilder(
      animation: _shimmer,
      builder: (context, _) {
        return Container(
          width: widget.width,
          height: widget.height,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(widget.borderRadius),
            gradient: LinearGradient(
              begin: Alignment.centerLeft,
              end: Alignment.centerRight,
              colors: [
                c.muted,
                Color.alphaBlend(
                  Colors.white.withAlpha((20 * _shimmer.value).round()),
                  c.muted,
                ),
                c.muted,
              ],
              stops: [
                0.0,
                _shimmer.value.clamp(0.2, 0.8),
                1.0,
              ],
            ),
          ),
        );
      },
    );
  }
}
