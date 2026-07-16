import 'package:flutter/material.dart';

/// Wraps [child] in a [Directionality] widget to set LTR/RTL for a subtree.
///
/// Mirrors the web soma-ui `DirectionProvider` in
/// `packages/ui/src/components/interaction/direction_provider.rs`.
class SomaDirectionProvider extends StatelessWidget {
  final TextDirection direction;
  final Widget child;

  const SomaDirectionProvider({
    super.key,
    this.direction = TextDirection.ltr,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Directionality(textDirection: direction, child: child);
  }
}
