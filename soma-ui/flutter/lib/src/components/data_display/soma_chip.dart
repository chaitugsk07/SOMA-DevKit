import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

enum SomaChipVariant { primary, secondary, outline }

class SomaChip extends StatefulWidget {
  final SomaChipVariant variant;
  final bool removable;
  final VoidCallback? onRemove;
  final Widget child;

  const SomaChip({
    super.key,
    this.variant = SomaChipVariant.primary,
    this.removable = false,
    this.onRemove,
    required this.child,
  });

  @override
  State<SomaChip> createState() => _SomaChipState();
}

class _SomaChipState extends State<SomaChip> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    Color bg;
    Color fg;
    Color borderColor;
    final bool isFilled = widget.variant != SomaChipVariant.outline;

    switch (widget.variant) {
      case SomaChipVariant.primary:
        bg = c.primary;
        fg = c.primaryForeground;
        borderColor = Colors.transparent;
      case SomaChipVariant.secondary:
        bg = c.secondary;
        fg = c.secondaryForeground;
        borderColor = Colors.transparent;
      case SomaChipVariant.outline:
        bg = Colors.transparent;
        fg = c.foreground;
        borderColor = c.border;
    }

    List<BoxShadow>? shadows;
    Color effectiveBg;

    if (isFilled) {
      shadows = _hovered
          ? [BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 8, offset: const Offset(0, 2))]
          : [BoxShadow(color: Colors.black.withAlpha(14), blurRadius: 4, offset: const Offset(0, 1))];
      effectiveBg = bg;
    } else {
      shadows = null;
      effectiveBg = _hovered ? c.accent.withAlpha(60) : bg;
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 120),
        curve: Curves.easeOutCubic,
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 3),
        decoration: BoxDecoration(
          gradient: isFilled
              ? LinearGradient(
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                  colors: [
                    Color.alphaBlend(Colors.white.withAlpha(18), effectiveBg),
                    effectiveBg,
                  ],
                )
              : null,
          color: isFilled ? null : effectiveBg,
          borderRadius: BorderRadius.circular(999),
          border: Border.all(color: borderColor),
          boxShadow: shadows,
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            DefaultTextStyle(
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 12,
                fontWeight: FontWeight.w500,
                color: fg,
              ),
              child: widget.child,
            ),
            if (widget.removable) ...[
              const SizedBox(width: 4),
              _ChipRemoveButton(onRemove: widget.onRemove, fg: fg),
            ],
          ],
        ),
      ),
    );
  }
}

class _ChipRemoveButton extends StatefulWidget {
  final VoidCallback? onRemove;
  final Color fg;

  const _ChipRemoveButton({required this.onRemove, required this.fg});

  @override
  State<_ChipRemoveButton> createState() => _ChipRemoveButtonState();
}

class _ChipRemoveButtonState extends State<_ChipRemoveButton> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: widget.onRemove,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 120),
          curve: Curves.easeOutCubic,
          width: 14,
          height: 14,
          decoration: BoxDecoration(
            color: _hovered ? c.foreground.withAlpha(20) : Colors.transparent,
            shape: BoxShape.circle,
          ),
          child: Center(
            child: Icon(LucideIcons.x, size: 10, color: widget.fg),
          ),
        ),
      ),
    );
  }
}
