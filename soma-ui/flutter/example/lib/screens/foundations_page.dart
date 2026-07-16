import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class FoundationsPage extends StatelessWidget {
  const FoundationsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return SingleChildScrollView(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Foundations',
              style: Theme.of(context).textTheme.headlineMedium),
          const SizedBox(height: 4),
          Text(
            'Design tokens & typography.',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 32),

          // ── Colors ────────────────────────────────────────────────────────
          Text(
            'COLORS',
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 11,
              fontWeight: FontWeight.w600,
              letterSpacing: 1.2,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 12),
          SomaCard(
            padding: const EdgeInsets.all(16),
            child: LayoutBuilder(builder: (context, constraints) {
              final cols = constraints.maxWidth >= 480 ? 4 : 2;
              final swatches = [
                ('background', c.background, c.foreground),
                ('card', c.card, c.cardForeground),
                ('muted', c.muted, c.mutedForeground),
                ('secondary', c.secondary, c.secondaryForeground),
                ('accent', c.accent, c.accentForeground),
                ('primary', c.primary, c.primaryForeground),
                ('destructive', c.destructive, c.destructiveForeground),
                ('success', c.success, c.successForeground),
                ('border', c.border, c.foreground),
              ];
              return Wrap(
                spacing: 12,
                runSpacing: 12,
                children: swatches.map((s) {
                  final itemWidth =
                      (constraints.maxWidth - 16 - (cols - 1) * 12) / cols;
                  return SizedBox(
                    width: itemWidth.clamp(80, 200),
                    child: _ColorSwatch(
                      name: s.$1,
                      fill: s.$2,
                      textColor: s.$3,
                    ),
                  );
                }).toList(),
              );
            }),
          ),

          const SizedBox(height: 32),

          // ── Typography ────────────────────────────────────────────────────
          Text(
            'TYPOGRAPHY',
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 11,
              fontWeight: FontWeight.w600,
              letterSpacing: 1.2,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 12),
          SomaCard(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _TypographyRow(
                  sample: 'Rajdhani Bold',
                  label: 'Rajdhani · Bold · 28',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontWeight: FontWeight.w700,
                    fontSize: 28,
                    color: c.foreground,
                  ),
                ),
                _divider(c),
                _TypographyRow(
                  sample: 'Rajdhani Semibold',
                  label: 'Rajdhani · Semibold · 22',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontWeight: FontWeight.w600,
                    fontSize: 22,
                    color: c.foreground,
                  ),
                ),
                _divider(c),
                _TypographyRow(
                  sample: 'Rajdhani Medium',
                  label: 'Rajdhani · Medium · 18',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontWeight: FontWeight.w500,
                    fontSize: 18,
                    color: c.foreground,
                  ),
                ),
                _divider(c),
                _TypographyRow(
                  sample: 'Outfit Semibold',
                  label: 'Outfit · Semibold · 18',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.w600,
                    fontSize: 18,
                    color: c.foreground,
                  ),
                ),
                _divider(c),
                _TypographyRow(
                  sample: 'Outfit Regular',
                  label: 'Outfit · Regular · 16',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.w400,
                    fontSize: 16,
                    color: c.foreground,
                  ),
                ),
                _divider(c),
                _TypographyRow(
                  sample: 'Outfit Small',
                  label: 'Outfit · Regular · 13 · muted',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontWeight: FontWeight.w400,
                    fontSize: 13,
                    color: c.mutedForeground,
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 24),
        ],
      ),
    );
  }

  Widget _divider(SomaColors c) => Padding(
        padding: const EdgeInsets.symmetric(vertical: 12),
        child: Divider(color: c.border, height: 1),
      );
}

class _ColorSwatch extends StatelessWidget {
  final String name;
  final Color fill;
  final Color textColor;

  const _ColorSwatch({
    required this.name,
    required this.fill,
    required this.textColor,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Container(
          height: 64,
          decoration: BoxDecoration(
            color: fill,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: c.border.withAlpha(80)),
          ),
          child: Center(
            child: Text(
              'Aa',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontWeight: FontWeight.w600,
                fontSize: 18,
                color: textColor,
              ),
            ),
          ),
        ),
        const SizedBox(height: 6),
        Text(
          name,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 12,
            color: c.mutedForeground,
          ),
        ),
      ],
    );
  }
}

class _TypographyRow extends StatelessWidget {
  final String sample;
  final String label;
  final TextStyle style;

  const _TypographyRow({
    required this.sample,
    required this.label,
    required this.style,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Expanded(child: Text(sample, style: style)),
        const SizedBox(width: 16),
        Text(
          label,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 12,
            color: c.mutedForeground,
          ),
        ),
      ],
    );
  }
}
