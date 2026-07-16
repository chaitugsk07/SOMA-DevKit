import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaInput extends StatefulWidget {
  final String? placeholder;
  final TextEditingController? controller;
  final bool enabled;
  final ValueChanged<String>? onChanged;
  final bool obscureText;
  final TextInputType? keyboardType;

  const SomaInput({
    super.key,
    this.placeholder,
    this.controller,
    this.enabled = true,
    this.onChanged,
    this.obscureText = false,
    this.keyboardType,
  });

  @override
  State<SomaInput> createState() => _SomaInputState();
}

class _SomaInputState extends State<SomaInput> {
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

    final Color borderColor;
    if (_focused) {
      borderColor = c.ring;
    } else if (_hovered) {
      borderColor = c.ring.withAlpha(120);
    } else {
      borderColor = c.input;
    }

    final List<BoxShadow> shadows;
    if (_focused) {
      shadows = [
        BoxShadow(color: c.ring.withAlpha(50), blurRadius: 6, spreadRadius: 1),
        BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
      ];
    } else {
      shadows = [
        BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
      ];
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedOpacity(
        opacity: widget.enabled ? 1.0 : 0.5,
        duration: const Duration(milliseconds: 140),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 140),
          curve: Curves.easeOutCubic,
          height: 40,
          decoration: BoxDecoration(
            color: Colors.transparent,
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
            obscureText: widget.obscureText,
            keyboardType: widget.keyboardType,
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
