import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '../../icons/soma_icons.dart';

class SomaPagination extends StatelessWidget {
  final int page;
  final int totalPages;
  final ValueChanged<int>? onChanged;

  const SomaPagination({
    super.key,
    required this.page,
    required this.totalPages,
    this.onChanged,
  });

  // Returns null for ellipsis slots, int for page number
  List<int?> _pageWindows() {
    if (totalPages <= 7) {
      return List.generate(totalPages, (i) => i + 1);
    }
    final result = <int?>[];
    final start = (page - 1).clamp(2, totalPages - 1);
    final end = (page + 1).clamp(2, totalPages - 1);
    result.add(1);
    if (start > 2) result.add(null); // ellipsis
    for (int p = start; p <= end; p++) {
      result.add(p);
    }
    if (end < totalPages - 1) result.add(null); // ellipsis
    result.add(totalPages);
    return result;
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final windows = _pageWindows();

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        _PaginationButton(
          icon: LucideIcons.chevronLeft,
          enabled: page > 1,
          colors: c,
          onTap: () => onChanged?.call(page - 1),
        ),
        const SizedBox(width: 4),
        for (final p in windows) ...[
          if (p == null)
            SizedBox(
              width: 32,
              height: 32,
              child: Center(
                child: Text('…', style: TextStyle(color: c.mutedForeground, fontSize: 14)),
              ),
            )
          else
            _PageButton(
              pageNum: p,
              isActive: p == page,
              colors: c,
              onTap: () => onChanged?.call(p),
            ),
          const SizedBox(width: 4),
        ],
        _PaginationButton(
          icon: LucideIcons.chevronRight,
          enabled: page < totalPages,
          colors: c,
          onTap: () => onChanged?.call(page + 1),
        ),
      ],
    );
  }
}

class _PaginationButton extends StatefulWidget {
  final IconData icon;
  final bool enabled;
  final SomaColors colors;
  final VoidCallback onTap;

  const _PaginationButton({
    required this.icon,
    required this.enabled,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_PaginationButton> createState() => _PaginationButtonState();
}

class _PaginationButtonState extends State<_PaginationButton> {
  bool _hovered = false;
  bool _focused = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;

    List<BoxShadow>? shadows;
    if (_focused && widget.enabled) {
      shadows = [BoxShadow(color: c.ring.withAlpha(50), blurRadius: 0, spreadRadius: 2)];
    } else if (_hovered && widget.enabled) {
      shadows = [BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 4, offset: const Offset(0, 1))];
    }

    return AnimatedOpacity(
      opacity: widget.enabled ? 1.0 : 0.4,
      duration: const Duration(milliseconds: 150),
      child: Focus(
        onFocusChange: (f) => setState(() => _focused = f),
        child: MouseRegion(
          cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.forbidden,
          onEnter: (_) => setState(() => _hovered = true),
          onExit: (_) => setState(() => _hovered = false),
          child: GestureDetector(
            onTap: widget.enabled ? widget.onTap : null,
            onTapDown: widget.enabled ? (_) => setState(() => _pressed = true) : null,
            onTapUp: widget.enabled ? (_) => setState(() => _pressed = false) : null,
            onTapCancel: widget.enabled ? () => setState(() => _pressed = false) : null,
            child: AnimatedScale(
              scale: _pressed ? 0.97 : 1.0,
              duration: const Duration(milliseconds: 120),
              curve: Curves.easeOutCubic,
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 140),
                curve: Curves.easeOutCubic,
                width: 32,
                height: 32,
                decoration: BoxDecoration(
                  color: _hovered && widget.enabled ? c.accent : Colors.transparent,
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(color: c.border),
                  boxShadow: shadows,
                ),
                child: Icon(widget.icon, size: 16, color: c.foreground),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _PageButton extends StatefulWidget {
  final int pageNum;
  final bool isActive;
  final SomaColors colors;
  final VoidCallback onTap;

  const _PageButton({
    required this.pageNum,
    required this.isActive,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_PageButton> createState() => _PageButtonState();
}

class _PageButtonState extends State<_PageButton> {
  bool _hovered = false;
  bool _focused = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;

    List<BoxShadow>? shadows;
    if (_focused) {
      shadows = [BoxShadow(color: c.ring.withAlpha(50), blurRadius: 0, spreadRadius: 2)];
    } else if (widget.isActive) {
      shadows = [BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 6, offset: const Offset(0, 2))];
    } else if (_hovered) {
      shadows = [BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 4, offset: const Offset(0, 1))];
    }

    return Focus(
      onFocusChange: (f) => setState(() => _focused = f),
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
              duration: const Duration(milliseconds: 140),
              curve: Curves.easeOutCubic,
              width: 32,
              height: 32,
              decoration: BoxDecoration(
                color: widget.isActive
                    ? c.primary
                    : (_hovered ? c.accent : Colors.transparent),
                borderRadius: BorderRadius.circular(6),
                border: widget.isActive ? null : Border.all(color: c.border),
                boxShadow: shadows,
              ),
              child: Center(
                child: Text(
                  '${widget.pageNum}',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 13,
                    fontWeight: FontWeight.w500,
                    color: widget.isActive ? c.primaryForeground : c.foreground,
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
