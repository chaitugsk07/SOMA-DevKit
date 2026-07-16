import 'package:flutter/material.dart';

/// Constrains a child widget to a W:H aspect ratio.
///
/// ponytail: thin wrapper around Flutter's native AspectRatio widget.
/// Web default ratio is 16/9; no additional bg/rounding on the web side.
class SomaAspectRatio extends StatelessWidget {
  final double ratio;
  final Widget child;

  const SomaAspectRatio({
    super.key,
    this.ratio = 16 / 9,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return AspectRatio(
      aspectRatio: ratio,
      child: ClipRect(child: child),
    );
  }
}
