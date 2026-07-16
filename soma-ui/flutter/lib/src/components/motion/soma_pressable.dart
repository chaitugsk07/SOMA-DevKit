import 'package:flutter/material.dart';

class SomaPressable extends StatefulWidget {
  final Widget child;
  final VoidCallback? onTap;

  const SomaPressable({
    super.key,
    required this.child,
    this.onTap,
  });

  @override
  State<SomaPressable> createState() => _SomaPressableState();
}

class _SomaPressableState extends State<SomaPressable> {
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTapDown: (_) => setState(() => _pressed = true),
      onTapUp: (_) {
        setState(() => _pressed = false);
        widget.onTap?.call();
      },
      onTapCancel: () => setState(() => _pressed = false),
      child: AnimatedScale(
        scale: _pressed ? 0.95 : 1.0,
        duration: const Duration(milliseconds: 100),
        child: widget.child,
      ),
    );
  }
}
