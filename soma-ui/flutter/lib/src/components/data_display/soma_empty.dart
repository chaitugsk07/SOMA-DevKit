import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaEmpty extends StatelessWidget {
  final Widget? icon;
  final String title;
  final String? description;
  final Widget? child;

  const SomaEmpty({
    super.key,
    this.icon,
    required this.title,
    this.description,
    this.child,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Padding(
      padding: const EdgeInsets.all(32),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (icon != null) ...[
            IconTheme(
              data: IconThemeData(color: c.mutedForeground, size: 40),
              child: icon!,
            ),
            const SizedBox(height: 12),
          ],
          Text(
            title,
            textAlign: TextAlign.center,
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 18,
              fontWeight: FontWeight.w600,
              color: c.foreground,
            ),
          ),
          if (description != null) ...[
            const SizedBox(height: 4),
            Text(
              description!,
              textAlign: TextAlign.center,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 14,
                color: c.mutedForeground,
              ),
            ),
          ],
          if (child != null) ...[
            const SizedBox(height: 16),
            child!,
          ],
        ],
      ),
    );
  }
}
