import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaTab {
  final String label;
  final Widget? content;

  const SomaTab({required this.label, this.content});
}

class SomaTabs extends StatefulWidget {
  final List<SomaTab> tabs;
  final int index;
  final ValueChanged<int>? onChanged;

  const SomaTabs({
    super.key,
    required this.tabs,
    required this.index,
    this.onChanged,
  });

  @override
  State<SomaTabs> createState() => _SomaTabsState();
}

class _SomaTabsState extends State<SomaTabs> {
  int? _hoveredIndex;
  int? _focusedIndex;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        // Tab bar — scrollable so tabs never overflow on narrow screens
        SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: Container(
            height: 36,
            padding: const EdgeInsets.all(4),
            decoration: BoxDecoration(
              color: c.muted,
              borderRadius: BorderRadius.circular(6),
            ),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: List.generate(widget.tabs.length, (i) {
                final active = i == widget.index;
                final hovered = _hoveredIndex == i && !active;
                final focused = _focusedIndex == i;

                List<BoxShadow>? shadows;
                if (focused) {
                  shadows = [
                    BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 2),
                  ];
                } else if (active) {
                  shadows = [
                    BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 8, offset: const Offset(0, 2)),
                  ];
                }

                return Focus(
                  onFocusChange: (f) => setState(() => _focusedIndex = f ? i : null),
                  child: MouseRegion(
                    cursor: SystemMouseCursors.click,
                    onEnter: (_) => setState(() => _hoveredIndex = i),
                    onExit: (_) => setState(() => _hoveredIndex = null),
                    child: GestureDetector(
                      onTap: () => widget.onChanged?.call(i),
                      child: AnimatedContainer(
                        duration: const Duration(milliseconds: 200),
                        curve: Curves.easeOutCubic,
                        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                        decoration: BoxDecoration(
                          color: active
                              ? c.background
                              : hovered
                                  ? c.background.withAlpha(100)
                                  : Colors.transparent,
                          borderRadius: BorderRadius.circular(4),
                          border: active
                              ? Border.all(color: Colors.white.withAlpha(18))
                              : null,
                          boxShadow: shadows,
                        ),
                        child: Text(
                          widget.tabs[i].label,
                          maxLines: 1,
                          softWrap: false,
                          overflow: TextOverflow.visible,
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 14,
                            fontWeight: FontWeight.w500,
                            color: active
                                ? c.foreground
                                : hovered
                                    ? c.foreground.withAlpha(200)
                                    : c.mutedForeground,
                          ),
                        ),
                      ),
                    ),
                  ),
                );
              }),
            ),
          ),
        ),
        // Tab content (if provided)
        if (widget.index < widget.tabs.length && widget.tabs[widget.index].content != null) ...[
          const SizedBox(height: 8),
          widget.tabs[widget.index].content!,
        ],
      ],
    );
  }
}
