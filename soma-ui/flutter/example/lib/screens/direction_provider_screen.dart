import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DirectionProviderScreen extends StatefulWidget {
  const DirectionProviderScreen({super.key});

  @override
  State<DirectionProviderScreen> createState() => _DirectionProviderScreenState();
}

class _DirectionProviderScreenState extends State<DirectionProviderScreen> {
  TextDirection _direction = TextDirection.ltr;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Direction Provider',
      subtitle: 'Wraps content in a Directionality widget for LTR/RTL layout.',
      preview: SomaDirectionProvider(
        direction: _direction,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(LucideIcons.alignLeft, size: 16, color: c.mutedForeground),
                const SizedBox(width: 8),
                Text(
                  'Direction: ${_direction == TextDirection.ltr ? "LTR" : "RTL"}',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.foreground,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: c.muted,
                borderRadius: BorderRadius.circular(6),
                border: Border.all(color: c.border),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Hello, world! This text follows the direction.',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: c.foreground,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Row(
                    children: [
                      Icon(LucideIcons.chevronRight, size: 14, color: c.mutedForeground),
                      const SizedBox(width: 4),
                      Text(
                        'Arrow direction mirrors text flow',
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 12,
                          color: c.mutedForeground,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
      controls: ControlRow(
        label: 'Direction',
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            SomaButton(
              variant: _direction == TextDirection.ltr
                  ? SomaButtonVariant.primary
                  : SomaButtonVariant.outline,
              size: SomaButtonSize.sm,
              onPressed: () => setState(() => _direction = TextDirection.ltr),
              child: const Text('LTR'),
            ),
            const SizedBox(width: 8),
            SomaButton(
              variant: _direction == TextDirection.rtl
                  ? SomaButtonVariant.primary
                  : SomaButtonVariant.outline,
              size: SomaButtonSize.sm,
              onPressed: () => setState(() => _direction = TextDirection.rtl),
              child: const Text('RTL'),
            ),
          ],
        ),
      ),
    );
  }
}
