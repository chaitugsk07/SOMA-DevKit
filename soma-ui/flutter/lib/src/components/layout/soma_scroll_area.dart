import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

/// Styled scrollable container with a themed thin scrollbar.
///
/// ponytail: thin Scrollbar + SingleChildScrollView wrapper.
/// Scrollbar thumb color matches the border token (matches web [&::-webkit-scrollbar-thumb]:bg-border).
class SomaScrollArea extends StatelessWidget {
  final Widget child;
  final double? height;
  final Axis scrollDirection;

  const SomaScrollArea({
    super.key,
    required this.child,
    this.height,
    this.scrollDirection = Axis.vertical,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    Widget scrollable = Scrollbar(
      thickness: 4,
      radius: const Radius.circular(4),
      thumbVisibility: true,
      child: SingleChildScrollView(
        scrollDirection: scrollDirection,
        child: child,
      ),
    );

    if (height != null) {
      scrollable = SizedBox(height: height, child: scrollable);
    }

    return ScrollbarTheme(
      data: ScrollbarThemeData(
        thumbColor: WidgetStateProperty.all(c.border),
        trackColor: WidgetStateProperty.all(Colors.transparent),
        radius: const Radius.circular(4),
        thickness: WidgetStateProperty.all(4),
      ),
      child: scrollable,
    );
  }
}
