import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

// ponytail: data-driven PageView carousel — diverges from the web crate's
// composable Carousel/CarouselContent/CarouselItem/Previous/Next/Dots API.
// Flutter's PageView makes a single data-driven widget simpler and idiomatic.
// Ceiling: autoplay, custom slide transitions = upgrade path.

/// A paginated carousel with optional arrow buttons and dot indicators.
///
/// Mirrors the web soma-ui `Carousel` family in
/// `packages/ui/src/components/media/carousel.rs`.
/// **API divergence:** data-driven (`items` list) instead of composable slots,
/// because Flutter's [PageView] makes this the idiomatic choice.
class SomaCarousel extends StatefulWidget {
  final List<Widget> items;
  final bool showArrows;
  final bool showDots;

  const SomaCarousel({
    super.key,
    required this.items,
    this.showArrows = true,
    this.showDots = true,
  });

  @override
  State<SomaCarousel> createState() => _SomaCarouselState();
}

class _SomaCarouselState extends State<SomaCarousel> {
  late final PageController _controller;
  int _current = 0;

  @override
  void initState() {
    super.initState();
    _controller = PageController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _prev() {
    if (_current > 0) {
      _controller.previousPage(
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
  }

  void _next() {
    if (_current < widget.items.length - 1) {
      _controller.nextPage(
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final count = widget.items.length;

    return Stack(
      alignment: Alignment.center,
      children: [
        // Page view
        PageView(
          controller: _controller,
          onPageChanged: (i) => setState(() => _current = i),
          children: widget.items,
        ),

        // Prev arrow
        if (widget.showArrows)
          Positioned(
            left: 8,
            child: _ArrowButton(
              icon: LucideIcons.chevronLeft,
              onPressed: _current > 0 ? _prev : null,
              colors: c,
            ),
          ),

        // Next arrow
        if (widget.showArrows)
          Positioned(
            right: 8,
            child: _ArrowButton(
              icon: LucideIcons.chevronRight,
              onPressed: _current < count - 1 ? _next : null,
              colors: c,
            ),
          ),

        // Dot indicators
        if (widget.showDots)
          Positioned(
            bottom: 8,
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                for (int i = 0; i < count; i++)
                  GestureDetector(
                    onTap: () => _controller.animateToPage(
                      i,
                      duration: const Duration(milliseconds: 300),
                      curve: Curves.easeInOut,
                    ),
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 200),
                      margin: const EdgeInsets.symmetric(horizontal: 3),
                      width: _current == i ? 16 : 6,
                      height: 6,
                      decoration: BoxDecoration(
                        color: _current == i
                            ? c.primary
                            : c.foreground.withAlpha(102),
                        borderRadius: BorderRadius.circular(3),
                      ),
                    ),
                  ),
              ],
            ),
          ),
      ],
    );
  }
}

class _ArrowButton extends StatelessWidget {
  final IconData icon;
  final VoidCallback? onPressed;
  final dynamic colors; // SomaColors

  const _ArrowButton({
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
          width: 32,
          height: 32,
          decoration: BoxDecoration(
            color: c.card,
            border: Border.all(color: c.border),
            shape: BoxShape.circle,
          ),
          child: Icon(icon, size: 16, color: c.foreground),
        ),
      ),
    );
  }
}
