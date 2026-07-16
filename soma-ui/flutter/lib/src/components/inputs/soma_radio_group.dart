import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaRadioOption<T> {
  final T value;
  final String label;

  const SomaRadioOption({required this.value, required this.label});
}

class SomaRadioGroup<T> extends StatelessWidget {
  final List<SomaRadioOption<T>> options;
  final T value;
  final ValueChanged<T>? onChanged;
  final bool enabled;

  const SomaRadioGroup({
    super.key,
    required this.options,
    required this.value,
    this.onChanged,
    this.enabled = true,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: options.map((opt) {
        return Padding(
          padding: const EdgeInsets.only(bottom: 8),
          child: _RadioOptionRow<T>(
            option: opt,
            selected: opt.value == value,
            enabled: enabled,
            onTap: () {
              if (enabled && onChanged != null) {
                onChanged!(opt.value);
              }
            },
          ),
        );
      }).toList(),
    );
  }
}

class _RadioOptionRow<T> extends StatefulWidget {
  final SomaRadioOption<T> option;
  final bool selected;
  final bool enabled;
  final VoidCallback onTap;

  const _RadioOptionRow({
    super.key,
    required this.option,
    required this.selected,
    required this.enabled,
    required this.onTap,
  });

  @override
  State<_RadioOptionRow<T>> createState() => _RadioOptionRowState<T>();
}

class _RadioOptionRowState<T> extends State<_RadioOptionRow<T>> {
  bool _hovered = false;
  bool _focused = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final Color circleFill = (_hovered && !widget.selected)
        ? c.accent.withAlpha(30)
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
          onTap: widget.onTap,
          onTapDown: widget.enabled ? (_) => setState(() => _pressed = true) : null,
          onTapUp: widget.enabled ? (_) => setState(() => _pressed = false) : null,
          onTapCancel: widget.enabled ? () => setState(() => _pressed = false) : null,
          child: AnimatedOpacity(
            opacity: widget.enabled ? 1.0 : 0.5,
            duration: const Duration(milliseconds: 150),
            child: AnimatedScale(
              scale: _pressed ? 0.97 : 1.0,
              duration: const Duration(milliseconds: 140),
              curve: Curves.easeOutCubic,
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  AnimatedContainer(
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
                  const SizedBox(width: 8),
                  Text(
                    widget.option.label,
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: c.foreground,
                    ),
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
