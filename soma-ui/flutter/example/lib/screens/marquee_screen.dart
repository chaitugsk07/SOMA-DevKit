import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class MarqueeScreen extends StatefulWidget {
  const MarqueeScreen({super.key});

  @override
  State<MarqueeScreen> createState() => _MarqueeScreenState();
}

class _MarqueeScreenState extends State<MarqueeScreen> {
  bool _reverse = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Marquee',
      subtitle: 'Infinite horizontal scroll for banners and tickers.',
      preview: SomaMarquee(
        reverse: _reverse,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            SomaBadge(child: const Text('Flutter')),
            const SizedBox(width: 12),
            SomaBadge(
              variant: SomaBadgeVariant.secondary,
              child: const Text('Motion'),
            ),
            const SizedBox(width: 12),
            Text(
              'soma_ui',
              style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 16,
                fontWeight: FontWeight.w600,
                color: c.foreground,
              ),
            ),
            const SizedBox(width: 12),
            SomaBadge(
              variant: SomaBadgeVariant.outline,
              child: const Text('Marquee'),
            ),
            const SizedBox(width: 48),
          ],
        ),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Reverse',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            SomaSwitch(
              value: _reverse,
              onChanged: (v) => setState(() => _reverse = v),
            ),
          ]),
        ],
      ),
    );
  }
}
