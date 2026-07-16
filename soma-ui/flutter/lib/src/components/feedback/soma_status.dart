import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

enum SomaStatusKind { online, offline, away, busy }

class SomaStatus extends StatelessWidget {
  final SomaStatusKind kind;
  final String? label;

  const SomaStatus({
    super.key,
    this.kind = SomaStatusKind.online,
    this.label,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final Color dotColor;
    switch (kind) {
      case SomaStatusKind.online:
        dotColor = c.success;
      case SomaStatusKind.offline:
        dotColor = c.mutedForeground;
      case SomaStatusKind.away:
        dotColor = const Color(0xFFEAB308);
      case SomaStatusKind.busy:
        dotColor = c.destructive;
    }

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 8,
          height: 8,
          decoration: BoxDecoration(
            color: dotColor,
            shape: BoxShape.circle,
            boxShadow: [
              BoxShadow(
                color: dotColor.withAlpha(100),
                blurRadius: 4,
                spreadRadius: 1,
              ),
            ],
          ),
        ),
        if (label != null) ...[
          const SizedBox(width: 8),
          Text(
            label!,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              fontWeight: FontWeight.w500,
              color: c.mutedForeground,
            ),
          ),
        ],
      ],
    );
  }
}
