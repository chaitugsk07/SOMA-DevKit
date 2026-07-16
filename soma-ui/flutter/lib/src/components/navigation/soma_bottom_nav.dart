import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaBottomNavItem {
  final IconData icon;
  final String label;

  const SomaBottomNavItem({required this.icon, required this.label});
}

class SomaBottomNav extends StatelessWidget {
  final List<SomaBottomNavItem> items;
  final int index;
  final ValueChanged<int>? onTap;

  const SomaBottomNav({
    super.key,
    required this.items,
    required this.index,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Container(
      decoration: BoxDecoration(
        color: c.card,
        border: Border(top: BorderSide(color: c.border)),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 12, offset: const Offset(0, -4)),
          BoxShadow(color: Colors.black.withAlpha(8), blurRadius: 3, offset: const Offset(0, -1)),
        ],
      ),
      child: SafeArea(
        top: false,
        child: Row(
          children: List.generate(items.length, (i) {
            final active = i == index;
            return Expanded(
              child: _BottomNavItemWidget(
                item: items[i],
                active: active,
                onTap: () => onTap?.call(i),
              ),
            );
          }),
        ),
      ),
    );
  }
}

class _BottomNavItemWidget extends StatefulWidget {
  final SomaBottomNavItem item;
  final bool active;
  final VoidCallback onTap;

  const _BottomNavItemWidget({
    required this.item,
    required this.active,
    required this.onTap,
  });

  @override
  State<_BottomNavItemWidget> createState() => _BottomNavItemWidgetState();
}

class _BottomNavItemWidgetState extends State<_BottomNavItemWidget> {
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final color = widget.active ? c.primary : c.mutedForeground;

    return GestureDetector(
      onTap: widget.onTap,
      behavior: HitTestBehavior.opaque,
      onTapDown: (_) => setState(() => _pressed = true),
      onTapUp: (_) => setState(() => _pressed = false),
      onTapCancel: () => setState(() => _pressed = false),
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        child: AnimatedScale(
          scale: _pressed ? 0.93 : 1.0,
          duration: const Duration(milliseconds: 120),
          curve: Curves.easeOutCubic,
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 8),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                // Active indicator pill above the icon
                AnimatedContainer(
                  duration: const Duration(milliseconds: 160),
                  curve: Curves.easeOutCubic,
                  height: 2,
                  width: widget.active ? 24 : 0,
                  margin: const EdgeInsets.only(bottom: 4),
                  decoration: BoxDecoration(
                    color: c.primary,
                    borderRadius: BorderRadius.circular(99),
                  ),
                ),
                AnimatedContainer(
                  duration: const Duration(milliseconds: 140),
                  curve: Curves.easeOutCubic,
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 2),
                  decoration: BoxDecoration(
                    color: widget.active ? c.primary.withAlpha(20) : Colors.transparent,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Icon(widget.item.icon, size: 22, color: color),
                ),
                const SizedBox(height: 2),
                AnimatedDefaultTextStyle(
                  duration: const Duration(milliseconds: 140),
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 11,
                    fontWeight: widget.active ? FontWeight.w600 : FontWeight.w400,
                    color: color,
                  ),
                  child: Text(widget.item.label),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
