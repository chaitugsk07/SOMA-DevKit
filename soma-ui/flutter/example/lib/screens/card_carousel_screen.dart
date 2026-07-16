import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class CardCarouselScreen extends StatelessWidget {
  const CardCarouselScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final cards = [
      _CardItem(label: 'Button', icon: LucideIcons.mousePointerClick, colors: c),
      _CardItem(label: 'Input', icon: LucideIcons.textCursor, colors: c),
      _CardItem(label: 'Select', icon: LucideIcons.chevronsUpDown, colors: c),
      _CardItem(label: 'Carousel', icon: LucideIcons.galleryHorizontal, colors: c),
      _CardItem(label: 'Dialog', icon: LucideIcons.squareDashedBottom, colors: c),
      _CardItem(label: 'Badge', icon: LucideIcons.tag, colors: c),
    ];

    return ComponentPage(
      title: 'Card Carousel',
      subtitle: 'Horizontal card-swipe carousel with peeking adjacent cards.',
      preview: SizedBox(
        width: double.infinity,
        child: SomaCardCarousel(cards: cards),
      ),
      controls: Column(
        children: [
          ControlRow(
            label: 'Cards',
            child: Text(
              '${cards.length} items',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _CardItem extends StatelessWidget {
  final String label;
  final IconData icon;
  final dynamic colors;

  const _CardItem({required this.label, required this.icon, required this.colors});

  @override
  Widget build(BuildContext context) {
    final c = colors;
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(icon, size: 32, color: c.primary),
          const SizedBox(height: 12),
          Text(
            label,
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 18,
              fontWeight: FontWeight.w600,
              color: c.foreground,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            'soma_ui component',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 12,
              color: c.mutedForeground,
            ),
          ),
        ],
      ),
    );
  }
}
