import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class GettingStartedPage extends StatelessWidget {
  const GettingStartedPage({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return SingleChildScrollView(
      padding: const EdgeInsets.all(24),
      child: Align(
        alignment: Alignment.topCenter,
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 768),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Hero
              const SizedBox(height: 16),
              Text(
                'soma_ui',
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 56,
                  fontWeight: FontWeight.w700,
                  color: c.foreground,
                  height: 1.0,
                  letterSpacing: -1,
                ),
              ),
              const SizedBox(height: 12),
              Text(
                'A cross-platform design system delivering the same '
                'components across web (Leptos/WASM) and Flutter. '
                'Palantir-style slate & blue, light + dark out of the box, '
                'typeset in Outfit and Rajdhani. Copy-paste-first — you own '
                'the source.',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 16,
                  color: c.mutedForeground,
                  height: 1.6,
                ),
              ),
              const SizedBox(height: 32),

              // Highlight cards
              LayoutBuilder(builder: (context, constraints) {
                final wide = constraints.maxWidth >= 520;
                final cards = [
                  _HighlightCard(
                    icon: LucideIcons.layoutGrid,
                    title: '50+ Components',
                    body: 'Buttons, inputs, overlays, navigation, data display, '
                        'motion, and more — all styled with Palantir tokens.',
                  ),
                  _HighlightCard(
                    icon: LucideIcons.sunMoon,
                    title: 'Light & Dark',
                    body: 'Semantic CSS-variable tokens port perfectly into '
                        'Flutter\'s InheritedWidget theme. Toggle anytime.',
                  ),
                  _HighlightCard(
                    icon: LucideIcons.monitor,
                    title: 'Responsive',
                    body: 'Every component adapts from narrow mobile to wide '
                        'desktop. LayoutBuilder-driven breakpoints throughout.',
                  ),
                ];
                if (wide) {
                  return IntrinsicHeight(
                    child: Row(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: cards
                          .expand((card) => [
                                Expanded(child: card),
                                if (card != cards.last) const SizedBox(width: 16),
                              ])
                          .toList(),
                    ),
                  );
                }
                return Column(
                  children: cards
                      .expand((card) => [card, const SizedBox(height: 16)])
                      .toList()
                    ..removeLast(),
                );
              }),

              const SizedBox(height: 32),

              // Divider + hint
              Divider(color: c.border),
              const SizedBox(height: 16),
              Row(
                children: [
                  Icon(LucideIcons.chevronRight, size: 16, color: c.mutedForeground),
                  const SizedBox(width: 8),
                  Text(
                    'Pick a component from the sidebar to explore.',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 13,
                      color: c.mutedForeground,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 24),
            ],
          ),
        ),
      ),
    );
  }
}

class _HighlightCard extends StatelessWidget {
  final IconData icon;
  final String title;
  final String body;

  const _HighlightCard({
    required this.icon,
    required this.title,
    required this.body,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return SomaCard(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            width: 40,
            height: 40,
            decoration: BoxDecoration(
              color: c.primary.withAlpha(20),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Center(
              child: Icon(icon, size: 20, color: c.primary),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            title,
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 20,
              fontWeight: FontWeight.w600,
              color: c.cardForeground,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            body,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: c.mutedForeground,
              height: 1.5,
            ),
          ),
        ],
      ),
    );
  }
}
