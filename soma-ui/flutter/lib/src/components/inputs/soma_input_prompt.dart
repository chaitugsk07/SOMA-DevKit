import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

// ponytail: Enter sends, Shift+Enter inserts newline — mirrors web on:keydown behavior.
// Auto-grow via maxLines: null (Flutter expands naturally).
// Ceiling: no attach button (web's show_attach flag omitted — not requested).

class SomaInputPrompt extends StatefulWidget {
  final String? value;
  final ValueChanged<String>? onChanged;
  final VoidCallback? onSubmit;
  final String placeholder;

  const SomaInputPrompt({
    super.key,
    this.value,
    this.onChanged,
    this.onSubmit,
    this.placeholder = 'Ask anything...',
  });

  @override
  State<SomaInputPrompt> createState() => _SomaInputPromptState();
}

class _SomaInputPromptState extends State<SomaInputPrompt> {
  late final TextEditingController _ctrl;
  final FocusNode _focus = FocusNode();
  bool _focused = false;
  bool _hovered = false;
  bool _sendHovered = false;
  bool _sendPressed = false;

  @override
  void initState() {
    super.initState();
    _ctrl = TextEditingController(text: widget.value ?? '');
    _focus.addListener(() => setState(() => _focused = _focus.hasFocus));
  }

  @override
  void didUpdateWidget(SomaInputPrompt old) {
    super.didUpdateWidget(old);
    if (widget.value != null && widget.value != _ctrl.text) {
      _ctrl.text = widget.value!;
    }
  }

  @override
  void dispose() {
    _ctrl.dispose();
    _focus.dispose();
    super.dispose();
  }

  void _submit() {
    final text = _ctrl.text.trim();
    if (text.isEmpty) return;
    widget.onSubmit?.call();
    _ctrl.clear();
    widget.onChanged?.call('');
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        decoration: BoxDecoration(
          color: c.background,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: _focused ? c.ring : (_hovered ? c.ring.withAlpha(120) : c.input),
            width: _focused ? 2 : 1,
          ),
          boxShadow: _focused
              ? [
                  BoxShadow(color: c.ring.withAlpha(50), blurRadius: 6, spreadRadius: 1),
                  BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 8, offset: const Offset(0, 2)),
                ]
              : [BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 8, offset: const Offset(0, 2))],
        ),
        padding: const EdgeInsets.fromLTRB(12, 12, 12, 8),
        child: Column(
          mainAxisSize: MainAxisSize.min,
        children: [
          ConstrainedBox(
            constraints: const BoxConstraints(minHeight: 40, maxHeight: 192),
            child: TextField(
              controller: _ctrl,
              focusNode: _focus,
              maxLines: null,
              keyboardType: TextInputType.multiline,
              textInputAction: TextInputAction.newline,
              onChanged: widget.onChanged,
              onSubmitted: (_) => _submit(),
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 14,
                color: c.foreground,
              ),
              decoration: InputDecoration(
                hintText: widget.placeholder,
                hintStyle: TextStyle(color: c.mutedForeground),
                border: InputBorder.none,
                isDense: true,
                contentPadding: EdgeInsets.zero,
              ),
              cursorColor: c.ring,
            ),
          ),
          const SizedBox(height: 4),
          Align(
            alignment: Alignment.centerRight,
            child: MouseRegion(
              cursor: SystemMouseCursors.click,
              onEnter: (_) => setState(() => _sendHovered = true),
              onExit: (_) => setState(() => _sendHovered = false),
              child: GestureDetector(
                onTap: _submit,
                onTapDown: (_) => setState(() => _sendPressed = true),
                onTapUp: (_) => setState(() => _sendPressed = false),
                onTapCancel: () => setState(() => _sendPressed = false),
                child: AnimatedScale(
                  scale: _sendPressed ? 0.95 : 1.0,
                  duration: const Duration(milliseconds: 120),
                  curve: Curves.easeOutCubic,
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 150),
                    width: 32,
                    height: 32,
                    decoration: BoxDecoration(
                      gradient: LinearGradient(
                        begin: Alignment.topCenter,
                        end: Alignment.bottomCenter,
                        colors: () {
                          final baseColor = _sendHovered
                              ? Color.fromARGB(
                                  255,
                                  (c.primary.r * 0.9).round(),
                                  (c.primary.g * 0.9).round(),
                                  (c.primary.b * 0.9).round(),
                                )
                              : c.primary;
                          return [Color.alphaBlend(Colors.white.withAlpha(18), baseColor), baseColor];
                        }(),
                      ),
                      borderRadius: BorderRadius.circular(6),
                      boxShadow: [BoxShadow(color: Colors.black.withAlpha(14), blurRadius: 6, offset: const Offset(0, 1))],
                    ),
                    child: Center(
                      child: Icon(LucideIcons.arrowUp, size: 16, color: c.primaryForeground),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    ),
    );
  }
}
