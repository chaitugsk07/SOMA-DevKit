import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaTextarea extends StatefulWidget {
  final TextEditingController? controller;
  final String? placeholder;
  final bool enabled;
  final int minLines;
  final ValueChanged<String>? onChanged;

  const SomaTextarea({
    super.key,
    this.controller,
    this.placeholder,
    this.enabled = true,
    this.minLines = 3,
    this.onChanged,
  });

  @override
  State<SomaTextarea> createState() => _SomaTextareaState();
}

class _SomaTextareaState extends State<SomaTextarea> {
  final FocusNode _focus = FocusNode();
  bool _focused = false;
  bool _hovered = false;

  @override
  void initState() {
    super.initState();
    _focus.addListener(() => setState(() => _focused = _focus.hasFocus));
  }

  @override
  void dispose() {
    _focus.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final borderColor = _focused
        ? c.ring
        : (_hovered ? c.ring.withAlpha(120) : c.input);

    final shadows = [
      BoxShadow(
        color: Colors.black.withAlpha(12),
        blurRadius: 4,
        offset: const Offset(0, 1),
      ),
      if (_focused)
        BoxShadow(
          color: c.ring.withAlpha(50),
          blurRadius: 6,
          spreadRadius: 1,
        ),
    ];

    return AnimatedOpacity(
      opacity: widget.enabled ? 1.0 : 0.5,
      duration: const Duration(milliseconds: 150),
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(6),
            border: Border.all(
              color: borderColor,
              width: _focused ? 2 : 1,
            ),
            boxShadow: shadows,
          ),
          child: TextField(
            controller: widget.controller,
            focusNode: _focus,
            enabled: widget.enabled,
            onChanged: widget.onChanged,
            minLines: widget.minLines,
            maxLines: null,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: c.foreground,
            ),
            decoration: InputDecoration(
              hintText: widget.placeholder,
              hintStyle: TextStyle(color: c.mutedForeground),
              border: InputBorder.none,
              contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
              isDense: true,
            ),
            cursorColor: c.ring,
          ),
        ),
      ),
    );
  }
}
