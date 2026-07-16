import 'package:flutter/material.dart';
import '../../theme/soma_colors.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

class SomaSelectItem<T> {
  final T value;
  final String label;

  const SomaSelectItem({required this.value, required this.label});
}

class SomaSelect<T> extends StatefulWidget {
  final List<SomaSelectItem<T>> items;
  final T? value;
  final ValueChanged<T>? onChanged;
  final String? placeholder;
  final bool enabled;

  const SomaSelect({
    super.key,
    required this.items,
    this.value,
    this.onChanged,
    this.placeholder,
    this.enabled = true,
  });

  @override
  State<SomaSelect<T>> createState() => _SomaSelectState<T>();
}

class _SomaSelectState<T> extends State<SomaSelect<T>> {
  bool _open = false;
  bool _hovered = false;
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlay;

  void _openMenu(BuildContext context) {
    if (!widget.enabled) return;
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
          offset: const Offset(0, 46),
          child: Material(
            color: Colors.transparent,
            child: Container(
              constraints: const BoxConstraints(maxHeight: 240),
              decoration: BoxDecoration(
                color: c.card,
                borderRadius: BorderRadius.circular(6),
                border: Border.all(color: c.border),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withAlpha(8),
                    blurRadius: 4,
                    offset: const Offset(0, 1),
                  ),
                  BoxShadow(
                    color: Colors.black.withAlpha(25),
                    blurRadius: 16,
                    offset: const Offset(0, 8),
                  ),
                ],
              ),
              child: ClipRRect(
                borderRadius: BorderRadius.circular(6),
                child: SingleChildScrollView(
                  padding: const EdgeInsets.all(4),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: widget.items.map((item) {
                      final selected = item.value == widget.value;
                      return _SelectMenuRow(
                        label: item.label,
                        selected: selected,
                        colors: c,
                        onTap: () {
                          _closeMenu();
                          widget.onChanged?.call(item.value);
                        },
                      );
                    }).toList(),
                  ),
                ),
              ),
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
    final selectedLabel = widget.items
        .where((i) => i.value == widget.value)
        .map((i) => i.label)
        .firstOrNull;
    final displayText = selectedLabel ?? widget.placeholder;
    final isPlaceholder = selectedLabel == null;

    final Color borderColor;
    if (_open) {
      borderColor = c.ring;
    } else if (_hovered) {
      borderColor = c.ring.withAlpha(120);
    } else {
      borderColor = c.input;
    }

    final List<BoxShadow> shadows;
    if (_open) {
      shadows = [
        BoxShadow(color: c.ring.withAlpha(50), blurRadius: 6, spreadRadius: 1),
        BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
      ];
    } else {
      shadows = [
        BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
      ];
    }

    return CompositedTransformTarget(
      link: _layerLink,
      child: MouseRegion(
        cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.forbidden,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: () => _open ? _closeMenu() : _openMenu(context),
          child: AnimatedOpacity(
            opacity: widget.enabled ? 1.0 : 0.5,
            duration: const Duration(milliseconds: 150),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 140),
              curve: Curves.easeOutCubic,
              height: 40,
              padding: const EdgeInsets.symmetric(horizontal: 12),
              decoration: BoxDecoration(
                color: _hovered ? c.accent.withAlpha(30) : Colors.transparent,
                borderRadius: BorderRadius.circular(6),
                border: Border.all(
                  color: borderColor,
                  width: _open ? 2 : 1,
                ),
                boxShadow: shadows,
              ),
              child: Row(
                children: [
                  Expanded(
                    child: Text(
                      displayText ?? '',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: isPlaceholder ? c.mutedForeground : c.foreground,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                  AnimatedRotation(
                    turns: _open ? 0.5 : 0,
                    duration: const Duration(milliseconds: 150),
                    child: Icon(LucideIcons.chevronDown, size: 18, color: c.mutedForeground),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _SelectMenuRow extends StatefulWidget {
  final String label;
  final bool selected;
  final SomaColors colors;
  final VoidCallback onTap;

  const _SelectMenuRow({
    required this.label,
    required this.selected,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_SelectMenuRow> createState() => _SelectMenuRowState();
}

class _SelectMenuRowState extends State<_SelectMenuRow> {
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
          duration: const Duration(milliseconds: 120),
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
