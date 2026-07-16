import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class CarouselScreen extends StatefulWidget {
  const CarouselScreen({super.key});

  @override
  State<CarouselScreen> createState() => _CarouselScreenState();
}

class _CarouselScreenState extends State<CarouselScreen> {
  bool _showArrows = true;
  bool _showDots = true;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final slides = [
      _Slide(
        title: 'Palantir Design',
        subtitle: 'Slate + blue, light & dark',
        color: c.primary,
        colors: c,
      ),
      _Slide(
        title: 'Cross-Platform',
        subtitle: 'Web · Mobile · Native',
        color: c.accent,
        colors: c,
      ),
      _Slide(
        title: 'Copy-Paste First',
        subtitle: 'You own the source code',
        color: c.muted,
        colors: c,
      ),
      _Slide(
        title: 'Open System',
        subtitle: 'Built on Leptos + Flutter',
        color: c.secondary,
        colors: c,
      ),
    ];

    return ComponentPage(
      title: 'Carousel',
      subtitle: 'Paginated carousel with arrow controls and dot indicators.',
      preview: SizedBox(
        height: 200,
        child: SomaCarousel(
          showArrows: _showArrows,
          showDots: _showDots,
          items: slides,
        ),
      ),
      controls: Column(
        children: [
          ControlRow(
            label: 'Show arrows',
            child: SomaSwitch(
              value: _showArrows,
              onChanged: (v) => setState(() => _showArrows = v),
            ),
          ),
          ControlRow(
            label: 'Show dots',
            child: SomaSwitch(
              value: _showDots,
              onChanged: (v) => setState(() => _showDots = v),
            ),
          ),
        ],
      ),
    );
  }
}

class _Slide extends StatelessWidget {
  final String title;
  final String subtitle;
  final Color color;
  final dynamic colors;

  const _Slide({
    required this.title,
    required this.subtitle,
    required this.color,
    required this.colors,
  });

  @override
  Widget build(BuildContext context) {
    final c = colors;
    return Container(
      color: c.card,
      child: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              width: 48,
              height: 48,
              decoration: BoxDecoration(
                color: color.withAlpha(51),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(LucideIcons.layers, color: color, size: 24),
            ),
            const SizedBox(height: 12),
            Text(
              title,
              style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 22,
                fontWeight: FontWeight.w600,
                color: c.foreground,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              subtitle,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
