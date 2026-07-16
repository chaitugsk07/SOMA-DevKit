import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '../../icons/soma_icons.dart';

// ponytail: single-date, single-month view. ISO yyyy-MM-dd string value.
// No intl dependency — format manually. No range/min/max.
// Ceiling: range selection, min/max constraints, keyboard nav in grid.

const _kMonthNames = [
  'January', 'February', 'March', 'April', 'May', 'June',
  'July', 'August', 'September', 'October', 'November', 'December',
];
const _kDayHeaders = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];

String _isoDate(int year, int month, int day) =>
    '${year.toString().padLeft(4, '0')}-'
    '${month.toString().padLeft(2, '0')}-'
    '${day.toString().padLeft(2, '0')}';

class SomaDatePicker extends StatefulWidget {
  final String? value; // ISO yyyy-MM-dd or null
  final ValueChanged<String>? onChanged;
  final String placeholder;

  const SomaDatePicker({
    super.key,
    this.value,
    this.onChanged,
    this.placeholder = 'Pick a date',
  });

  @override
  State<SomaDatePicker> createState() => _SomaDatePickerState();
}

class _SomaDatePickerState extends State<SomaDatePicker> {
  bool _open = false;
  bool _hovered = false;
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlay;

  late int _viewYear;
  late int _viewMonth; // 1-based

  @override
  void initState() {
    super.initState();
    final now = DateTime.now();
    _viewYear = now.year;
    _viewMonth = now.month;
  }

  void _shiftMonth(int delta) {
    final d = DateTime(_viewYear, _viewMonth + delta);
    setState(() {
      _viewYear = d.year;
      _viewMonth = d.month;
    });
    // ponytail: markNeedsBuild() removed — StatefulBuilder's setInner already
    // handles overlay rebuilds; the redundant markNeedsBuild caused a
    // double-layout pass ("Each child must be laid out exactly once").
  }

  void _openMenu(BuildContext context) {
    final c = SomaTheme.of(context);
    _overlay = OverlayEntry(builder: (_) => _buildOverlay(c));
    Overlay.of(context).insert(_overlay!);
    setState(() => _open = true);
  }

  void _closeMenu() {
    _overlay?.remove();
    _overlay = null;
    if (mounted) setState(() => _open = false);
  }

  Widget _buildOverlay(SomaColors c) {
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
          offset: const Offset(0, 44),
          child: Material(
            color: Colors.transparent,
            child: StatefulBuilder(
              builder: (_, setInner) {
                final now = DateTime.now();
                final daysInMonth =
                    DateTime(_viewYear, _viewMonth + 1, 0).day;
                final firstDow =
                    DateTime(_viewYear, _viewMonth, 1).weekday % 7; // 0=Sun

                return Container(
                  width: 280,
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: c.card,
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: c.border),
                    boxShadow: [
                      BoxShadow(color: Colors.black.withAlpha(8), blurRadius: 4, offset: const Offset(0, 1)),
                      BoxShadow(color: Colors.black.withAlpha(25), blurRadius: 16, offset: const Offset(0, 8)),
                    ],
                  ),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      // Month navigation
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          _NavBtn(
                            icon: LucideIcons.chevronLeft,
                            colors: c,
                            onTap: () {
                              _shiftMonth(-1);
                              setInner(() {});
                            },
                          ),
                          Text(
                            '${_kMonthNames[_viewMonth - 1]} $_viewYear',
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 14,
                              fontWeight: FontWeight.w600,
                              color: c.foreground,
                            ),
                          ),
                          _NavBtn(
                            icon: LucideIcons.chevronRight,
                            colors: c,
                            onTap: () {
                              _shiftMonth(1);
                              setInner(() {});
                            },
                          ),
                        ],
                      ),
                      const SizedBox(height: 8),
                      // Weekday headers
                      Row(
                        children: _kDayHeaders
                            .map((h) => Expanded(
                                  child: Center(
                                    child: Text(
                                      h,
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 11,
                                        color: c.mutedForeground,
                                      ),
                                    ),
                                  ),
                                ))
                            .toList(),
                      ),
                      const SizedBox(height: 4),
                      // Day grid
                      GridView.count(
                        crossAxisCount: 7,
                        shrinkWrap: true,
                        physics: const NeverScrollableScrollPhysics(),
                        mainAxisSpacing: 2,
                        crossAxisSpacing: 2,
                        children: [
                          // Leading empty cells
                          for (int i = 0; i < firstDow; i++)
                            const SizedBox.shrink(),
                          // Day cells
                          for (int day = 1; day <= daysInMonth; day++)
                            _buildDay(day, now, c, setInner),
                        ],
                      ),
                    ],
                  ),
                );
              },
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildDay(
      int day, DateTime now, SomaColors c, StateSetter setInner) {
    final iso = _isoDate(_viewYear, _viewMonth, day);
    final isToday =
        _viewYear == now.year && _viewMonth == now.month && day == now.day;
    final isSelected = widget.value == iso;

    Color bg;
    Color fg;
    if (isSelected) {
      bg = c.primary;
      fg = c.primaryForeground;
    } else if (isToday) {
      bg = Colors.transparent;
      fg = c.primary;
    } else {
      bg = Colors.transparent;
      fg = c.foreground;
    }

    return _DayCell(
      day: day,
      bg: bg,
      fg: fg,
      isToday: isToday,
      isSelected: isSelected,
      colors: c,
      onTap: () {
        _closeMenu();
        widget.onChanged?.call(iso);
      },
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
    final hasValue = widget.value != null && widget.value!.isNotEmpty;

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
            height: 40,
            padding: const EdgeInsets.symmetric(horizontal: 12),
            decoration: BoxDecoration(
              color: _hovered ? c.accent.withAlpha(30) : Colors.transparent,
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
                Icon(LucideIcons.calendar,
                    size: 16, color: c.mutedForeground),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    widget.value ?? widget.placeholder,
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: hasValue ? c.foreground : c.mutedForeground,
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _NavBtn extends StatefulWidget {
  final IconData icon;
  final SomaColors colors;
  final VoidCallback onTap;

  const _NavBtn(
      {required this.icon, required this.colors, required this.onTap});

  @override
  State<_NavBtn> createState() => _NavBtnState();
}

class _NavBtnState extends State<_NavBtn> {
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
          width: 28,
          height: 28,
          decoration: BoxDecoration(
            color: _hovered ? c.accent : Colors.transparent,
            borderRadius: BorderRadius.circular(6),
          ),
          child: Center(
            child: Icon(widget.icon, size: 14, color: c.foreground),
          ),
        ),
      ),
    );
  }
}

class _DayCell extends StatefulWidget {
  final int day;
  final Color bg;
  final Color fg;
  final bool isToday;
  final bool isSelected;
  final SomaColors colors;
  final VoidCallback onTap;

  const _DayCell({
    required this.day,
    required this.bg,
    required this.fg,
    required this.isToday,
    required this.isSelected,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_DayCell> createState() => _DayCellState();
}

class _DayCellState extends State<_DayCell> {
  bool _hovered = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    final bg = _hovered && !widget.isSelected ? c.accent : widget.bg;
    final fg = _hovered && !widget.isSelected ? c.accentForeground : widget.fg;

    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() { _hovered = false; _pressed = false; }),
      child: GestureDetector(
        onTap: widget.onTap,
        onTapDown: (_) => setState(() => _pressed = true),
        onTapUp: (_) => setState(() => _pressed = false),
        onTapCancel: () => setState(() => _pressed = false),
        child: AnimatedScale(
          scale: _pressed ? 0.95 : 1.0,
          duration: const Duration(milliseconds: 80),
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 100),
            decoration: BoxDecoration(
              color: bg,
              shape: BoxShape.circle,
              border: widget.isToday && !widget.isSelected
                  ? Border.all(color: c.primary, width: 1.5)
                  : null,
            ),
            child: Center(
              child: Text(
                '${widget.day}',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 13,
                  fontWeight: widget.isToday ? FontWeight.w600 : FontWeight.w400,
                  color: fg,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
