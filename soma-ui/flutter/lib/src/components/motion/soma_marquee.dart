import 'package:flutter/material.dart';

class SomaMarquee extends StatefulWidget {
  final Widget child;
  final bool reverse;
  final Duration duration;

  const SomaMarquee({
    super.key,
    required this.child,
    this.reverse = false,
    this.duration = const Duration(seconds: 8),
  });

  @override
  State<SomaMarquee> createState() => _SomaMarqueeState();
}

class _SomaMarqueeState extends State<SomaMarquee>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(vsync: this, duration: widget.duration)
      ..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ClipRect(
      child: AnimatedBuilder(
        animation: _controller,
        builder: (context, child) {
          // t goes 0→1; we shift one full copy width (50% of row = one child)
          final t = widget.reverse ? _controller.value : (1 - _controller.value);
          // offset is a fraction of child width; row = 2 copies, each = 50%
          // translate by -t * 50% of total row width = -t * childWidth
          return FractionalTranslation(
            translation: Offset(-t, 0),
            child: child,
          );
        },
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [widget.child, widget.child],
        ),
      ),
    );
  }
}
