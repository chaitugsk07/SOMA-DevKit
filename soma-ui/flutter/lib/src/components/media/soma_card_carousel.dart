import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

// ponytail: uses a PageView with viewportFraction so adjacent cards peek.
// The web crate uses CSS scroll-snap; PageView is Flutter's idiomatic equivalent.
// Ceiling: programmatic scroll-to-index, variable card widths = upgrade path
// (use a ScrollController + custom snap logic or the carousel_slider package).

/// A horizontal card-swipe carousel with peeking adjacent cards.
///
/// Mirrors the web soma-ui `CardCarousel` in
/// `packages/ui/src/components/media/card_carousel.rs`.
/// Uses [PageView] with viewportFraction ~0.8 so the next card peeks.
class SomaCardCarousel extends StatefulWidget {
  final List<Widget> cards;

  const SomaCardCarousel({
    super.key,
    required this.cards,
  });

  @override
  State<SomaCardCarousel> createState() => _SomaCardCarouselState();
}

class _SomaCardCarouselState extends State<SomaCardCarousel> {
  late final PageController _controller;
  int _current = 0;

  @override
  void initState() {
    super.initState();
    _controller = PageController(viewportFraction: 0.8);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Stack(
      alignment: Alignment.center,
      children: [
        SizedBox(
          height: 200,
          child: PageView.builder(
            controller: _controller,
            itemCount: widget.cards.length,
            onPageChanged: (i) => setState(() => _current = i),
            itemBuilder: (context, i) => Padding(
              padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 8),
              child: Container(
                decoration: BoxDecoration(
                  color: c.card,
                  border: Border.all(color: c.border),
                  borderRadius: BorderRadius.circular(8),
                ),
                clipBehavior: Clip.antiAlias,
                child: widget.cards[i],
              ),
            ),
          ),
        ),
        // Prev arrow
        Positioned(
          left: 0,
          child: _SmallArrow(
            icon: LucideIcons.chevronLeft,
            onPressed: _current > 0
                ? () => _controller.previousPage(
                      duration: const Duration(milliseconds: 300),
                      curve: Curves.easeInOut,
                    )
                : null,
            colors: c,
          ),
        ),
        // Next arrow
        Positioned(
          right: 0,
          child: _SmallArrow(
            icon: LucideIcons.chevronRight,
            onPressed: _current < widget.cards.length - 1
                ? () => _controller.nextPage(
                      duration: const Duration(milliseconds: 300),
                      curve: Curves.easeInOut,
                    )
                : null,
            colors: c,
          ),
        ),
      ],
    );
  }
}

class _SmallArrow extends StatelessWidget {
  final IconData icon;
  final VoidCallback? onPressed;
  final dynamic colors;

  const _SmallArrow({
    required this.icon,
    required this.onPressed,
    required this.colors,
  });

  @override
  Widget build(BuildContext context) {
    final c = colors;
    final enabled = onPressed != null;
    return GestureDetector(
      onTap: onPressed,
      child: AnimatedOpacity(
        duration: const Duration(milliseconds: 150),
        opacity: enabled ? 1.0 : 0.4,
        child: Container(
          width: 28,
          height: 28,
          decoration: BoxDecoration(
            color: c.card,
            border: Border.all(color: c.border),
            shape: BoxShape.circle,
            boxShadow: [
              BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 4),
            ],
          ),
          child: Icon(icon, size: 14, color: c.foreground),
        ),
      ),
    );
  }
}
