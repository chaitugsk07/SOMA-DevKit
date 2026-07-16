import 'package:flutter/material.dart';
import '../../theme/soma_colors.dart';
import '../../theme/soma_theme.dart';

class SomaToggleItem<T> {
  final T value;
  final Widget child;

  const SomaToggleItem({required this.value, required this.child});
}

class SomaToggleGroup<T> extends StatelessWidget {
  final List<SomaToggleItem<T>> items;
  final T value;
  final ValueChanged<T>? onChanged;

  const SomaToggleGroup({
    super.key,
    required this.items,
    required this.value,
    this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Container(
      decoration: BoxDecoration(
        border: Border.all(color: c.border),
        borderRadius: BorderRadius.circular(6),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(10),
            blurRadius: 8,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(5),
        child: IntrinsicHeight(
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: items.asMap().entries.map((entry) {
              final idx = entry.key;
              final item = entry.value;
              final selected = item.value == value;
              final isLast = idx == items.length - 1;

              return _SomaToggleSegment(
                selected: selected,
                colors: c,
                isLast: isLast,
                onTap: () => onChanged?.call(item.value),
                child: item.child,
              );
            }).toList(),
          ),
        ),
      ),
    );
  }
}

class _SomaToggleSegment extends StatefulWidget {
  final bool selected;
  final SomaColors colors;
  final bool isLast;
  final VoidCallback onTap;
  final Widget child;

  const _SomaToggleSegment({
    required this.selected,
    required this.colors,
    required this.isLast,
    required this.onTap,
    required this.child,
  });

  @override
  State<_SomaToggleSegment> createState() => _SomaToggleSegmentState();
}

class _SomaToggleSegmentState extends State<_SomaToggleSegment> {
  bool _hovered = false;
  bool _focused = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    final fgColor = widget.selected ? c.accentForeground : c.foreground;

    List<BoxShadow> shadows = [];
    if (widget.selected) {
      shadows.add(BoxShadow(
        color: Colors.black.withAlpha(14),
        blurRadius: 6,
        offset: const Offset(0, 1),
      ));
    }
    if (_focused) {
      shadows.add(BoxShadow(
        color: c.ring.withAlpha(55),
        blurRadius: 0,
        spreadRadius: 2,
      ));
    }

    final decoration = widget.selected
        ? BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.topCenter,
              end: Alignment.bottomCenter,
              colors: [
                Color.alphaBlend(Colors.white.withAlpha(12), c.accent),
                c.accent,
              ],
            ),
            boxShadow: shadows.isEmpty ? null : shadows,
          )
        : BoxDecoration(
            color: _hovered ? c.muted : Colors.transparent,
            boxShadow: shadows.isEmpty ? null : shadows,
          );

    return Focus(
      onFocusChange: (focused) => setState(() => _focused = focused),
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: widget.onTap,
          onTapDown: (_) => setState(() => _pressed = true),
          onTapUp: (_) => setState(() => _pressed = false),
          onTapCancel: () => setState(() => _pressed = false),
          child: AnimatedScale(
            scale: _pressed ? 0.97 : 1.0,
            duration: const Duration(milliseconds: 120),
            curve: Curves.easeOutCubic,
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 150),
              decoration: decoration,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              child: DefaultTextStyle(
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 14,
                  fontWeight: FontWeight.w500,
                  color: fgColor,
                ),
                child: IconTheme(
                  data: IconThemeData(color: fgColor, size: 16),
                  child: widget.child,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
