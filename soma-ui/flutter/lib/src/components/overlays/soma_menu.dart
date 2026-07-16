import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '_panel_parts.dart';

// ── Shared model ──────────────────────────────────────────────────────────────

class SomaMenuItem {
  final String label;
  final VoidCallback? onSelected;
  final IconData? icon;
  final bool isLabel;
  final bool isSeparator;

  const SomaMenuItem({
    this.label = '',
    this.onSelected,
    this.icon,
    this.isLabel = false,
    this.isSeparator = false,
  });

  static SomaMenuItem separator() => const SomaMenuItem(isSeparator: true);
  static SomaMenuItem labelItem(String text) => SomaMenuItem(label: text, isLabel: true);
}

// ── Shared anchored-overlay factory ───────────────────────────────────────────

/// Creates an [OverlayEntry] with a tap-outside-dismiss barrier + a
/// [CompositedTransformFollower] panel anchored to [link].
OverlayEntry buildAnchoredEntry({
  required LayerLink link,
  required Widget panel,
  required VoidCallback onClose,
  Offset offset = const Offset(0, 8),
}) {
  return OverlayEntry(
    builder: (_) => Stack(
      children: [
        Positioned.fill(
          child: GestureDetector(
            onTap: onClose,
            behavior: HitTestBehavior.translucent,
            child: const SizedBox.expand(),
          ),
        ),
        CompositedTransformFollower(
          link: link,
          showWhenUnlinked: false,
          offset: offset,
          child: Material(color: Colors.transparent, child: FadeScaleIn(child: panel)),
        ),
      ],
    ),
  );
}

// ── Shared menu panel ─────────────────────────────────────────────────────────

class MenuPanel extends StatelessWidget {
  final List<SomaMenuItem> items;
  final VoidCallback onClose;
  final double? minWidth;

  const MenuPanel({
    super.key,
    required this.items,
    required this.onClose,
    this.minWidth,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      constraints: BoxConstraints(maxHeight: 280, minWidth: minWidth ?? 180),
      decoration: BoxDecoration(
        color: c.card,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: c.border),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(12),
            blurRadius: 6,
          ),
          BoxShadow(
            color: Colors.black.withAlpha(22),
            blurRadius: 24,
            offset: const Offset(0, 10),
          ),
        ],
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(8),
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(4),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: items.map((item) {
              if (item.isSeparator) {
                return Container(
                  height: 1,
                  margin: const EdgeInsets.symmetric(vertical: 4, horizontal: 4),
                  color: c.border,
                );
              }
              if (item.isLabel) {
                return Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  child: Text(
                    item.label.toUpperCase(),
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 11,
                      fontWeight: FontWeight.w600,
                      color: c.mutedForeground,
                      letterSpacing: 0.5,
                    ),
                  ),
                );
              }
              return MenuItemRow(item: item, onClose: onClose, colors: c);
            }).toList(),
          ),
        ),
      ),
    );
  }
}

class MenuItemRow extends StatefulWidget {
  final SomaMenuItem item;
  final VoidCallback onClose;
  final SomaColors colors;

  const MenuItemRow({
    super.key,
    required this.item,
    required this.onClose,
    required this.colors,
  });

  @override
  State<MenuItemRow> createState() => _MenuItemRowState();
}

class _MenuItemRowState extends State<MenuItemRow> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: () {
          widget.onClose();
          widget.item.onSelected?.call();
        },
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 100),
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
          decoration: BoxDecoration(
            color: _hovered ? c.accent : Colors.transparent,
            borderRadius: BorderRadius.circular(4),
          ),
          child: Row(
            children: [
              if (widget.item.icon != null) ...[
                Icon(
                  widget.item.icon,
                  size: 16,
                  color: _hovered ? c.accentForeground : c.foreground,
                ),
                const SizedBox(width: 10),
              ],
              Expanded(
                child: Text(
                  widget.item.label,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: _hovered ? c.accentForeground : c.foreground,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
