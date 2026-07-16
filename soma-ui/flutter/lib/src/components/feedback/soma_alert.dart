import 'package:flutter/material.dart';
import '../../theme/soma_colors.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

enum SomaAlertVariant { primary, destructive, success, warning, info }

class SomaAlert extends StatelessWidget {
  final SomaAlertVariant variant;
  final String? title;
  final Widget child;

  const SomaAlert({
    super.key,
    this.variant = SomaAlertVariant.primary,
    this.title,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final Color accentColor;
    final Color bgColor;
    final Color borderColor;
    final IconData iconData;

    switch (variant) {
      case SomaAlertVariant.primary:
        accentColor = c.primary;
        bgColor = c.card;
        borderColor = c.border;
        iconData = LucideIcons.info;
      case SomaAlertVariant.destructive:
        accentColor = c.destructive;
        bgColor = c.destructive.withAlpha(18);
        borderColor = c.destructive.withAlpha(100);
        iconData = LucideIcons.circleX;
      case SomaAlertVariant.success:
        accentColor = c.success;
        bgColor = c.success.withAlpha(18);
        borderColor = c.success.withAlpha(100);
        iconData = LucideIcons.circleCheck;
      case SomaAlertVariant.warning:
        const amber = Color(0xFFEAB308);
        accentColor = amber;
        bgColor = amber.withAlpha(18);
        borderColor = amber.withAlpha(100);
        iconData = LucideIcons.triangleAlert;
      case SomaAlertVariant.info:
        accentColor = c.primary;
        bgColor = c.primary.withAlpha(18);
        borderColor = c.primary.withAlpha(100);
        iconData = LucideIcons.info;
    }

    const radius = BorderRadius.all(Radius.circular(6));

    // ClipRRect clips the accent strip to rounded corners. The outer
    // Container carries the uniform border + shadow; the accent strip is a
    // Positioned left-edge overlay inside a Stack.
    return Container(
      decoration: BoxDecoration(
        borderRadius: radius,
        border: Border.fromBorderSide(BorderSide(color: borderColor)),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 8, offset: const Offset(0, 2)),
          BoxShadow(color: Colors.black.withAlpha(5), blurRadius: 2, offset: const Offset(0, 1)),
        ],
      ),
      child: ClipRRect(
        borderRadius: radius,
        child: Stack(
          children: [
            // Tinted background + content
            Container(
              color: bgColor,
              padding: const EdgeInsets.fromLTRB(16 + 3, 16, 16, 16),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Padding(
                    padding: const EdgeInsets.only(top: 1),
                    child: Icon(iconData, size: 16, color: accentColor),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: DefaultTextStyle(
                      style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.foreground),
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
                          DefaultTextStyle(
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 14,
                              color: _descriptionColor(c, accentColor),
                            ),
                            child: child,
                          ),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
            // Left accent strip — clipped to rounded corners by ClipRRect above
            Positioned(
              left: 0,
              top: 0,
              bottom: 0,
              child: Container(width: 3, color: accentColor),
            ),
          ],
        ),
      ),
    );
  }

  Color _descriptionColor(SomaColors c, Color accentColor) {
    return variant == SomaAlertVariant.primary ? c.mutedForeground : c.foreground;
  }
}
