import 'package:flutter/material.dart';

/// Controlled collapsible — the caller owns the [open] state and the trigger.
/// Mirrors web collapsible.rs: caller provides open signal + children; no built-in trigger.
class SomaCollapsible extends StatelessWidget {
  final bool open;
  final Widget child;

  const SomaCollapsible({super.key, required this.open, required this.child});

  @override
  Widget build(BuildContext context) {
    return AnimatedSize(
      duration: const Duration(milliseconds: 160),
      curve: Curves.easeOutCubic,
      alignment: Alignment.topCenter,
      child: ClipRect(
        child: open ? child : const SizedBox.shrink(),
      ),
    );
  }
}
