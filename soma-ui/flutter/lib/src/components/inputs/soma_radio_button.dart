import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

/// Standalone radio atom — a single circular radio control with an optional label.
///
/// For a group sharing a value, use [SomaRadioGroup].
/// Visual matches exactly how [SomaRadioGroup] renders an individual option.
class SomaRadioButton extends StatefulWidget {
  final bool selected;
  final ValueChanged<bool>? onChanged;
  final bool enabled;
  final Widget? label;

  const SomaRadioButton({
    super.key,
    required this.selected,
    this.onChanged,
    this.enabled = true,
    this.label,
  });

  @override
  State<SomaRadioButton> createState() => _SomaRadioButtonState();
}

class _SomaRadioButtonState extends State<SomaRadioButton> {
  bool _hovered = false;
  bool _focused = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final Color circleFill = (_hovered && !widget.selected)
        ? c.accent.withAlpha(40)
        : Colors.transparent;

    final List<BoxShadow> circleShadows = [
      if (_focused)
        BoxShadow(color: c.ring.withAlpha(55), blurRadius: 0, spreadRadius: 3),
    ];

    return Focus(
      onFocusChange: (focused) => setState(() => _focused = focused),
      child: MouseRegion(
        cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.forbidden,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: () {
            if (widget.enabled) widget.onChanged?.call(true);
          },
          onTapDown: widget.enabled ? (_) => setState(() => _pressed = true) : null,
          onTapUp: widget.enabled ? (_) => setState(() => _pressed = false) : null,
          onTapCancel: widget.enabled ? () => setState(() => _pressed = false) : null,
          child: AnimatedOpacity(
            opacity: widget.enabled ? 1.0 : 0.5,
            duration: const Duration(milliseconds: 150),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                AnimatedScale(
                  scale: _pressed ? 0.97 : 1.0,
                  duration: const Duration(milliseconds: 140),
                  curve: Curves.easeOutCubic,
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 150),
                    width: 20,
                    height: 20,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      border: Border.all(
                        color: widget.selected ? c.primary : c.border,
                        width: widget.selected ? 5 : 1.5,
                      ),
                      color: circleFill,
                      boxShadow: circleShadows.isEmpty ? null : circleShadows,
                    ),
                  ),
                ),
                if (widget.label != null) ...[
                  const SizedBox(width: 8),
                  DefaultTextStyle(
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: c.foreground,
                    ),
                    child: widget.label!,
                  ),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }
}
