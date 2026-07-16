import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class ComponentPage extends StatefulWidget {
  final String title;
  final String subtitle;
  final Widget preview;
  final Widget controls;
  final Widget? variants;
  final List<Widget>? states;

  const ComponentPage({
    super.key,
    required this.title,
    required this.subtitle,
    required this.preview,
    required this.controls,
    this.variants,
    this.states,
  });

  @override
  State<ComponentPage> createState() => _ComponentPageState();
}

class _ComponentPageState extends State<ComponentPage> {
  int _tabIndex = 0;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final isDark = Theme.of(context).brightness == Brightness.dark;

    // Build tab list: always "Preview", then conditionally "Variants" / "States".
    final tabLabels = <String>['Preview'];
    if (widget.variants != null) tabLabels.add('Variants');
    if (widget.states != null && widget.states!.isNotEmpty) tabLabels.add('States');
    final multiTab = tabLabels.length > 1;

    // Clamp index in case optional slots change.
    final safeIndex = _tabIndex.clamp(0, tabLabels.length - 1);

    return LayoutBuilder(
      builder: (context, constraints) {
        final isWide = constraints.maxWidth >= 560;
        final previewPadding = isWide ? 48.0 : 24.0;

        return SingleChildScrollView(
          padding: const EdgeInsets.fromLTRB(24, 24, 24, 40),
          child: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 768),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // ── Eyebrow rule ──────────────────────────────────────────
                  Container(
                    width: 32,
                    height: 2,
                    decoration: BoxDecoration(
                      color: c.primary,
                      borderRadius: BorderRadius.circular(1),
                    ),
                  ),
                  const SizedBox(height: 10),

                  // ── Title ─────────────────────────────────────────────────
                  Text(
                    widget.title,
                    style: TextStyle(
                      fontFamily: 'Rajdhani',
                      fontSize: 34,
                      fontWeight: FontWeight.w700,
                      letterSpacing: -0.8,
                      color: c.foreground,
                      height: 1.0,
                    ),
                  ),
                  const SizedBox(height: 6),
                  Text(
                    widget.subtitle,
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: c.mutedForeground,
                      height: 1.5,
                    ),
                  ),
                  const SizedBox(height: 28),

                  // ── Tab bar (only when >1 tab) ─────────────────────────────
                  if (multiTab) ...[
                    SomaTabs(
                      tabs: tabLabels.map((l) => SomaTab(label: l)).toList(),
                      index: safeIndex,
                      onChanged: (i) => setState(() => _tabIndex = i),
                    ),
                    const SizedBox(height: 20),
                  ],

                  // ── Tab content ───────────────────────────────────────────
                  AnimatedSwitcher(
                    duration: const Duration(milliseconds: 140),
                    child: KeyedSubtree(
                      key: ValueKey(safeIndex),
                      child: _buildTabContent(
                        safeIndex,
                        tabLabels,
                        c,
                        isDark,
                        previewPadding,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildTabContent(
    int index,
    List<String> tabLabels,
    dynamic c,
    bool isDark,
    double previewPadding,
  ) {
    final label = tabLabels[index];

    if (label == 'Variants' && widget.variants != null) {
      return _ContentCard(child: widget.variants!);
    }

    if (label == 'States' && widget.states != null) {
      return _ContentCard(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: widget.states!,
        ),
      );
    }

    // Default: Preview tab
    return _PreviewAndControls(
      preview: widget.preview,
      controls: widget.controls,
      previewPadding: previewPadding,
      isDark: isDark,
    );
  }
}

// ── Preview + Controls panel ─────────────────────────────────────────────────

class _PreviewAndControls extends StatelessWidget {
  final Widget preview;
  final Widget controls;
  final double previewPadding;
  final bool isDark;

  const _PreviewAndControls({
    required this.preview,
    required this.controls,
    required this.previewPadding,
    required this.isDark,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    // Subtle layered shadow — light: soft warm-black lift; dark: deep cool-black.
    final shadowColor = isDark
        ? Colors.black.withAlpha(90)
        : Colors.black.withAlpha(15);
    final shadowColor2 = isDark
        ? Colors.black.withAlpha(50)
        : Colors.black.withAlpha(10);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Preview panel
        Container(
          decoration: BoxDecoration(
            color: c.card,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: c.border),
            boxShadow: [
              BoxShadow(
                color: shadowColor,
                blurRadius: 12,
                offset: const Offset(0, 2),
              ),
              BoxShadow(
                color: shadowColor2,
                blurRadius: 4,
                offset: const Offset(0, 1),
              ),
            ],
          ),
          padding: EdgeInsets.all(previewPadding),
          child: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(minHeight: 208),
              child: Center(child: preview),
            ),
          ),
        ),
        const SizedBox(height: 16),

        // Controls panel
        Container(
          decoration: BoxDecoration(
            color: c.card,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: c.border),
          ),
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'CONTROLS',
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 11,
                  fontWeight: FontWeight.w700,
                  letterSpacing: 1.4,
                  color: c.mutedForeground,
                ),
              ),
              const SizedBox(height: 16),
              controls,
            ],
          ),
        ),
      ],
    );
  }
}

// ── Generic content card (for Variants / States tabs) ────────────────────────

class _ContentCard extends StatelessWidget {
  final Widget child;
  const _ContentCard({required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      decoration: BoxDecoration(
        color: c.card,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: c.border),
      ),
      padding: const EdgeInsets.all(24),
      child: child,
    );
  }
}

// ── ControlRow ───────────────────────────────────────────────────────────────

/// A single labeled control row, mirroring the Rust playground's ControlRow.
///
/// Wide (≥560px): label left, control right, space-between, border-bottom divider.
/// Narrow: label above control, 8px gap.
///
/// FIX (unbounded-width): In wide mode the child is wrapped in `Expanded` so
/// any internal `Expanded`/`Flexible` (e.g. SomaSelect's inner Row) receives
/// a bounded width instead of panicking with "incoming width is unbounded".
class ControlRow extends StatelessWidget {
  final String label;
  final Widget child;

  const ControlRow({super.key, required this.label, required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final labelText = Text(
      label,
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 13,
        color: c.mutedForeground,
      ),
    );

    return LayoutBuilder(
      builder: (context, constraints) {
        final isWide = constraints.maxWidth >= 560;
        if (isWide) {
          return Container(
            decoration: BoxDecoration(
              border: Border(bottom: BorderSide(color: c.border)),
            ),
            padding: const EdgeInsets.symmetric(vertical: 12),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                labelText,
                const SizedBox(width: 16),
                // BUG FIX: wrap in Expanded so SomaSelect (and any widget with
                // an internal Expanded/Flexible) gets a bounded width.
                Expanded(child: Align(alignment: Alignment.centerRight, child: child)),
              ],
            ),
          );
        }
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 10),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              labelText,
              const SizedBox(height: 8),
              child,
            ],
          ),
        );
      },
    );
  }
}
