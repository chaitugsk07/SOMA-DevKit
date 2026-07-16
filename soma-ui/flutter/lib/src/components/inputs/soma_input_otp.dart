import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../theme/soma_theme.dart';

// ponytail: numeric-only OTP, auto-advance forward, backspace moves back.
// length boxes, each with its own FocusNode + TextEditingController.
// Emits concatenated string via onChanged. Ceiling: alpha/mixed OTP
// (remove digitsOnly formatter + numeric keyboard type).

class SomaInputOtp extends StatefulWidget {
  final String value;
  final ValueChanged<String>? onChanged;
  final int length;

  const SomaInputOtp({
    super.key,
    required this.value,
    this.onChanged,
    this.length = 6,
  });

  @override
  State<SomaInputOtp> createState() => _SomaInputOtpState();
}

class _SomaInputOtpState extends State<SomaInputOtp> {
  late List<FocusNode> _focusNodes;
  late List<TextEditingController> _controllers;

  @override
  void initState() {
    super.initState();
    _focusNodes = List.generate(widget.length, (_) => FocusNode());
    _controllers = List.generate(widget.length, (i) {
      final ch = i < widget.value.length ? widget.value[i] : '';
      return TextEditingController(text: ch);
    });
    for (int i = 0; i < widget.length; i++) {
      _focusNodes[i].addListener(() => setState(() {}));
    }
  }

  @override
  void didUpdateWidget(SomaInputOtp old) {
    super.didUpdateWidget(old);
    if (widget.length != old.length) {
      // Dispose old resources before rebuilding to the new length.
      for (final f in _focusNodes) {
        f.dispose();
      }
      for (final c in _controllers) {
        c.dispose();
      }
      _focusNodes = List.generate(widget.length, (_) => FocusNode());
      _controllers = List.generate(widget.length, (i) {
        final ch = i < widget.value.length ? widget.value[i] : '';
        return TextEditingController(text: ch);
      });
      for (int i = 0; i < widget.length; i++) {
        _focusNodes[i].addListener(() => setState(() {}));
      }
      return; // lists rebuilt from widget.value; no further sync needed
    }
    // Sync controllers if value changes externally (same length).
    // Bound by _controllers.length (not widget.length) to guard any rebuild race.
    for (int i = 0; i < _controllers.length; i++) {
      final ch = i < widget.value.length ? widget.value[i] : '';
      if (_controllers[i].text != ch) {
        _controllers[i].text = ch;
      }
    }
  }

  @override
  void dispose() {
    for (final f in _focusNodes) {
      f.dispose();
    }
    for (final c in _controllers) {
      c.dispose();
    }
    super.dispose();
  }

  void _onChanged(int i, String text) {
    if (text.length > 1) {
      // Paste handling: distribute digits across boxes.
      final digits = text.replaceAll(RegExp(r'[^0-9]'), '');
      for (int j = 0; j < widget.length && j < digits.length; j++) {
        _controllers[j].text = digits[j];
      }
      final combined = _controllers.map((c) => c.text).join();
      widget.onChanged?.call(combined);
      final nextFocus = (i + digits.length).clamp(0, widget.length - 1);
      _focusNodes[nextFocus].requestFocus();
      return;
    }

    final combined = _controllers.map((c) => c.text).join();
    widget.onChanged?.call(combined);

    if (text.isNotEmpty && i + 1 < widget.length) {
      _focusNodes[i + 1].requestFocus();
    }
  }

  void _onKeyEvent(int i, KeyEvent event) {
    if (event is KeyDownEvent &&
        event.logicalKey == LogicalKeyboardKey.backspace) {
      if (_controllers[i].text.isEmpty && i > 0) {
        _controllers[i - 1].text = '';
        _focusNodes[i - 1].requestFocus();
        final combined = _controllers.map((c) => c.text).join();
        widget.onChanged?.call(combined);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: List.generate(widget.length, (i) {
        final focused = _focusNodes[i].hasFocus;
        final filled = _controllers[i].text.isNotEmpty;
        return Padding(
          padding: EdgeInsets.only(left: i == 0 ? 0 : 8),
          child: KeyboardListener(
            focusNode: FocusNode(skipTraversal: true),
            onKeyEvent: (e) => _onKeyEvent(i, e),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 150),
              width: 40,
              height: 48,
              decoration: BoxDecoration(
                color: filled ? c.muted : Colors.transparent,
                borderRadius: BorderRadius.circular(6),
                border: Border.all(
                  color: focused ? c.ring : c.border,
                  width: focused ? 2 : 1,
                ),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withAlpha(10),
                    blurRadius: 4,
                    offset: const Offset(0, 1),
                  ),
                  if (focused)
                    BoxShadow(
                      color: c.ring.withAlpha(50),
                      blurRadius: 6,
                      spreadRadius: 1,
                    ),
                ],
              ),
              child: TextField(
                controller: _controllers[i],
                focusNode: _focusNodes[i],
                textAlign: TextAlign.center,
                keyboardType: TextInputType.number,
                inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                maxLength: 1,
                onChanged: (t) => _onChanged(i, t),
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 18,
                  fontWeight: FontWeight.w600,
                  color: c.foreground,
                ),
                decoration: const InputDecoration(
                  border: InputBorder.none,
                  counterText: '',
                  contentPadding: EdgeInsets.zero,
                  isDense: true,
                ),
                cursorColor: c.ring,
              ),
            ),
          ),
        );
      }),
    );
  }
}
