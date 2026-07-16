import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class HoverCardScreen extends StatelessWidget {
  const HoverCardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Hover Card',
      subtitle: 'Rich preview card shown on hover.',
      preview: SomaHoverCard(
        trigger: Text(
          '@soma_ui',
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 14,
            color: c.primary,
            decoration: TextDecoration.underline,
            decorationColor: c.primary,
          ),
        ),
        content: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              'soma_ui',
              style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 16,
                fontWeight: FontWeight.w600,
                color: c.cardForeground,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              'Cross-platform design system for Leptos + Flutter.',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Text(
                  '128 ',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.w600,
                    fontSize: 13,
                    color: c.cardForeground,
                  ),
                ),
                Text(
                  'components',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 13,
                    color: c.mutedForeground,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
      controls: Text(
        'Hover over the @soma_ui link to see the hover card.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
