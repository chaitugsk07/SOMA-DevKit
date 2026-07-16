import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '../../icons/soma_icons.dart';

// ponytail: divergence from web — web uses Chip (data_display category built
// concurrently). Flutter impl renders inline pills directly to avoid a
// cross-category import that may not be stable yet.

class SomaMultiSelectOption {
  final String value;
  final String label;
  const SomaMultiSelectOption({required this.value, required this.label});
}

class SomaMultiSelect extends StatefulWidget {
  final List<SomaMultiSelectOption> options;
  final List<String> selected;
  final ValueChanged<List<String>>? onChanged;
  final String placeholder;

  const SomaMultiSelect({
    super.key,
    required this.options,
    required this.selected,
    this.onChanged,
    this.placeholder = '',
  });

  @override
  State<SomaMultiSelect> createState() => _SomaMultiSelectState();
}

class _SomaMultiSelectState extends State<SomaMultiSelect> {
  bool _open = false;
  bool _hovered = false;
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlay;

  void _toggle(String value) {
    final next = List<String>.from(widget.selected);
    if (next.contains(value)) {
      next.remove(value);
    } else {
      next.add(value);
    }
    widget.onChanged?.call(next);
  }

  void _openMenu(BuildContext context) {
    final c = SomaTheme.of(context);
    _overlay = OverlayEntry(builder: (_) => _buildOverlay(context, c));
    Overlay.of(context).insert(_overlay!);
    setState(() => _open = true);
  }

  void _closeMenu() {
    _overlay?.remove();
    _overlay = null;
    if (mounted) setState(() => _open = false);
  }

  Widget _buildOverlay(BuildContext outerContext, SomaColors c) {
    return Stack(
      children: [
        Positioned.fill(
          child: GestureDetector(
            onTap: _closeMenu,
            behavior: HitTestBehavior.translucent,
            child: const SizedBox.expand(),
          ),
        ),
        CompositedTransformFollower(
          link: _layerLink,
          showWhenUnlinked: false,
          offset: const Offset(0, 4),
          child: Material(
            color: Colors.transparent,
            child: StatefulBuilder(
              builder: (_, setInnerState) {
                return Container(
                  constraints: const BoxConstraints(maxHeight: 240),
                  decoration: BoxDecoration(
                    color: c.card,
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(color: c.border),
                    boxShadow: [
                      BoxShadow(color: Colors.black.withAlpha(8), blurRadius: 4, offset: const Offset(0, 1)),
                      BoxShadow(color: Colors.black.withAlpha(25), blurRadius: 16, offset: const Offset(0, 8)),
                    ],
                  ),
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(6),
                    child: SingleChildScrollView(
                      padding: const EdgeInsets.all(4),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: widget.options.map((opt) {
                          final isSelected =
                              widget.selected.contains(opt.value);
                          return _MultiSelectRow(
                            label: opt.label,
                            selected: isSelected,
                            colors: c,
                            onTap: () {
                              _toggle(opt.value);
                              // Rebuild overlay to reflect new selection
                              _overlay?.markNeedsBuild();
                              setInnerState(() {});
                            },
                          );
                        }).toList(),
                      ),
                    ),
                  ),
                );
              },
            ),
          ),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _overlay?.remove();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return CompositedTransformTarget(
      link: _layerLink,
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: () => _open ? _closeMenu() : _openMenu(context),
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            constraints: const BoxConstraints(minHeight: 40),
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(6),
              border: Border.all(
                color: _open ? c.ring : (_hovered ? c.ring.withAlpha(120) : c.input),
                width: _open ? 2 : 1,
              ),
              boxShadow: _open
                  ? [
                      BoxShadow(color: c.ring.withAlpha(50), blurRadius: 6, spreadRadius: 1),
                      BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
                    ]
                  : [
                      BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
                    ],
            ),
            child: Row(
              children: [
                Expanded(
                  child: widget.selected.isEmpty
                      ? Text(
                          widget.placeholder,
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 14,
                            color: c.mutedForeground,
                          ),
                        )
                      : Wrap(
                          spacing: 4,
                          runSpacing: 4,
                          children: widget.selected.map((val) {
                            final label = widget.options
                                .where((o) => o.value == val)
                                .map((o) => o.label)
                                .firstOrNull ??
                                val;
                            return _InlinePill(
                              label: label,
                              colors: c,
                              onRemove: () {
                                _toggle(val);
                              },
                            );
                          }).toList(),
                        ),
                ),
                const SizedBox(width: 4),
                AnimatedRotation(
                  turns: _open ? 0.5 : 0,
                  duration: const Duration(milliseconds: 150),
                  child: Icon(LucideIcons.chevronDown,
                      size: 18, color: c.mutedForeground),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

/// Inline removable pill — used only by SomaMultiSelect.
class _InlinePill extends StatelessWidget {
  final String label;
  final SomaColors colors;
  final VoidCallback onRemove;

  const _InlinePill({
    required this.label,
    required this.colors,
    required this.onRemove,
  });

  @override
  Widget build(BuildContext context) {
    final c = colors;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(999),
        border: Border.all(color: c.border),
        color: c.secondary,
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(8), blurRadius: 4, offset: const Offset(0, 1)),
        ],
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            label,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 12,
              color: c.secondaryForeground,
            ),
          ),
          const SizedBox(width: 4),
          GestureDetector(
            onTap: onRemove,
            child: Icon(LucideIcons.x, size: 12, color: c.mutedForeground),
          ),
        ],
      ),
    );
  }
}

class _MultiSelectRow extends StatefulWidget {
  final String label;
  final bool selected;
  final SomaColors colors;
  final VoidCallback onTap;

  const _MultiSelectRow({
    required this.label,
    required this.selected,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_MultiSelectRow> createState() => _MultiSelectRowState();
}

class _MultiSelectRowState extends State<_MultiSelectRow> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 100),
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
          decoration: BoxDecoration(
            color: _hovered ? c.accent : Colors.transparent,
            borderRadius: BorderRadius.circular(4),
          ),
          child: Row(
            children: [
              Expanded(
                child: Text(
                  widget.label,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: _hovered ? c.accentForeground : c.foreground,
                  ),
                ),
              ),
              if (widget.selected)
                Icon(LucideIcons.check, size: 14, color: c.primary),
            ],
          ),
        ),
      ),
    );
  }
}
