import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaSeparator extends StatelessWidget {
  final Axis orientation;

  const SomaSeparator({
    super.key,
    this.orientation = Axis.horizontal,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    if (orientation == Axis.horizontal) {
      return Container(height: 1, color: c.border);
    } else {
      return Container(width: 1, color: c.border);
    }
  }
}
