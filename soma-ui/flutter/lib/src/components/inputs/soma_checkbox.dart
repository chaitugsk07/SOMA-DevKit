import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

class SomaCheckbox extends StatefulWidget {
  final bool value;
  final ValueChanged<bool>? onChanged;
  final bool enabled;

  const SomaCheckbox({
    super.key,
    required this.value,
    this.onChanged,
    this.enabled = true,
  });

  @override
  State<SomaCheckbox> createState() => _SomaCheckboxState();
}

class _SomaCheckboxState extends State<SomaCheckbox> {
  bool _focused = false;
  bool _hovered = false;
  bool _pressed = false;

  void _handleTap() {
    if (widget.enabled && widget.onChanged != null) {
      widget.onChanged!(!widget.value);
    }
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final Color borderColor;
    if (widget.value) {
      borderColor = c.primary;
    } else if (_hovered) {
      borderColor = c.ring.withAlpha(180);
    } else {
      borderColor = c.border;
    }

    final Color bgColor;
    if (widget.value && _hovered) {
      bgColor = Color.alphaBlend(Colors.white.withAlpha(15), c.primary);
    } else if (widget.value) {
      bgColor = c.primary;
    } else {
      bgColor = Colors.transparent;
    }

    return MouseRegion(
      cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.forbidden,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: Focus(
        onFocusChange: (focused) => setState(() => _focused = focused),
        child: GestureDetector(
          onTap: _handleTap,
          onTapDown: widget.enabled ? (_) => setState(() => _pressed = true) : null,
          onTapUp: widget.enabled ? (_) => setState(() => _pressed = false) : null,
          onTapCancel: widget.enabled ? () => setState(() => _pressed = false) : null,
          child: AnimatedOpacity(
            opacity: widget.enabled ? 1.0 : 0.5,
            duration: const Duration(milliseconds: 150),
            child: Transform.scale(
              scale: _pressed ? 0.93 : 1.0,
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 140),
                curve: Curves.easeOutCubic,
                width: 20,
                height: 20,
                decoration: BoxDecoration(
                  color: bgColor,
                  borderRadius: BorderRadius.circular(4),
                  border: Border.all(
                    color: borderColor,
                    width: 1.5,
                  ),
                  boxShadow: _focused
                      ? [BoxShadow(color: c.ring.withAlpha(60), blurRadius: 2, spreadRadius: 3)]
                      : null,
                ),
                child: AnimatedScale(
                  scale: widget.value ? 1.0 : 0.0,
                  duration: const Duration(milliseconds: 120),
                  curve: Curves.easeOutCubic,
                  child: Icon(LucideIcons.check, size: 14, color: c.primaryForeground),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
