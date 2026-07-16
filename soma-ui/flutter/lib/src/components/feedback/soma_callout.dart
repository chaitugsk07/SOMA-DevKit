import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

enum SomaCalloutVariant { info, success, warning, destructive }

class SomaCallout extends StatelessWidget {
  final SomaCalloutVariant variant;
  final Widget? icon;
  final String? title;
  final Widget child;

  const SomaCallout({
    super.key,
    this.variant = SomaCalloutVariant.info,
    this.icon,
    this.title,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final Color borderColor;
    final Color accentColor;
    final Color bgColor;
    final IconData defaultIcon;

    switch (variant) {
      case SomaCalloutVariant.info:
        borderColor = c.primary;
        accentColor = c.primary;
        bgColor = c.primary.withAlpha(18);
        defaultIcon = LucideIcons.info;
      case SomaCalloutVariant.success:
        borderColor = c.success;
        accentColor = c.success;
        bgColor = c.success.withAlpha(18);
        defaultIcon = LucideIcons.circleCheck;
      case SomaCalloutVariant.warning:
        const amber = Color(0xFFEAB308);
        borderColor = amber;
        accentColor = amber;
        bgColor = amber.withAlpha(18);
        defaultIcon = LucideIcons.triangleAlert;
      case SomaCalloutVariant.destructive:
        borderColor = c.destructive;
        accentColor = c.destructive;
        bgColor = c.destructive.withAlpha(18);
        defaultIcon = LucideIcons.circleX;
    }

    const radius = BorderRadius.all(Radius.circular(6));
    final hairline = borderColor.withAlpha(80);

    // Uniform hairline border + ClipRRect so the left accent strip respects
    // rounded corners. Flutter forbids borderRadius with non-uniform Border
    // colors, so the accent strip is a Positioned left-edge overlay.
    return Container(
      decoration: BoxDecoration(
        borderRadius: radius,
        border: Border.fromBorderSide(BorderSide(color: hairline)),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 8, offset: const Offset(0, 2)),
          BoxShadow(color: Colors.black.withAlpha(5), blurRadius: 2, offset: const Offset(0, 1)),
        ],
      ),
      child: ClipRRect(
        borderRadius: radius,
        child: Stack(
          children: [
            // Background + content
            Container(
              color: bgColor,
              padding: const EdgeInsets.fromLTRB(16 + 4, 16, 16, 16),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Padding(
                    padding: const EdgeInsets.only(top: 1),
                    child: IconTheme(
                      data: IconThemeData(color: accentColor, size: 16),
                      child: icon ?? Icon(defaultIcon),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: DefaultTextStyle(
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: c.foreground,
                      ),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          if (title != null) ...[
                            Text(
                              title!,
                              style: TextStyle(
                                fontFamily: 'Rajdhani',
                                fontSize: 15,
                                fontWeight: FontWeight.w600,
                                color: accentColor,
                                letterSpacing: 0.1,
                              ),
                            ),
                            const SizedBox(height: 4),
                          ],
                          child,
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
            // Left accent strip (4 px wide, matching original design)
            Positioned(
              left: 0,
              top: 0,
              bottom: 0,
              child: Container(width: 4, color: borderColor),
            ),
          ],
        ),
      ),
    );
  }
}
