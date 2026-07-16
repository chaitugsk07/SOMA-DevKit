import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaItem extends StatelessWidget {
  final Widget? leading;
  final Widget child;
  final Widget? trailing;

  const SomaItem({
    super.key,
    this.leading,
    required this.child,
    this.trailing,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: c.card,
        border: Border.all(color: c.border),
        borderRadius: BorderRadius.circular(6),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(8), blurRadius: 6, offset: const Offset(0, 1)),
          BoxShadow(color: Colors.black.withAlpha(4), blurRadius: 2, offset: const Offset(0, 1)),
        ],
      ),
      child: Row(
        children: [
          if (leading != null) ...[
            IconTheme(
              data: IconThemeData(color: c.mutedForeground, size: 18),
              child: leading!,
            ),
            const SizedBox(width: 12),
          ],
          Expanded(
            child: DefaultTextStyle(
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 14,
                color: c.foreground,
              ),
              child: child,
            ),
          ),
          if (trailing != null) ...[
            const SizedBox(width: 12),
            IconTheme(
              data: IconThemeData(color: c.mutedForeground, size: 18),
              child: trailing!,
            ),
          ],
        ],
      ),
    );
  }
}
