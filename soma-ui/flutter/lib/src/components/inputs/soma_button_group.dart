import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

/// Layout wrapper that merges child buttons into one segmented control.
/// Children are laid out in a Row; 1px vertical dividers separate them;
/// the outer container is bordered with radius 6.
class SomaButtonGroup extends StatelessWidget {
  final List<Widget> children;

  const SomaButtonGroup({super.key, required this.children});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final items = <Widget>[];
    for (int i = 0; i < children.length; i++) {
      items.add(Flexible(fit: FlexFit.loose, child: children[i]));
      if (i < children.length - 1) {
        items.add(SizedBox(
          width: 1,
          child: ColoredBox(color: c.border),
        ));
      }
    }

    return Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(6),
        border: Border.all(color: c.border),
        boxShadow: [BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 8, offset: const Offset(0, 2))],
      ),
      clipBehavior: Clip.antiAlias,
      child: IntrinsicHeight(
        child: Row(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: items,
        ),
      ),
    );
  }
}
