import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

enum SomaButtonVariant { primary, outline, ghost, destructive, secondary, link }

enum SomaButtonSize { sm, md, lg, icon }

class SomaButton extends StatefulWidget {
  final SomaButtonVariant variant;
  final SomaButtonSize size;
  final bool enabled;
  final VoidCallback? onPressed;
  final Widget child;

  const SomaButton({
    super.key,
    this.variant = SomaButtonVariant.primary,
    this.size = SomaButtonSize.md,
    this.enabled = true,
    this.onPressed,
    required this.child,
  });

  @override
  State<SomaButton> createState() => _SomaButtonState();
}

class _SomaButtonState extends State<SomaButton> {
  bool _hovered = false;
  bool _pressed = false;
  bool _focused = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final isEnabled = widget.enabled && widget.onPressed != null;

    // Resolve colors per variant
    Color bg;
    Color fg;
    Color? borderColor;
    bool underline = false;
    List<BoxShadow>? shadows;

    switch (widget.variant) {
      case SomaButtonVariant.primary:
        bg = _hovered ? Color.fromARGB(
          255,
          (c.primary.r * 0.9).round(),
          (c.primary.g * 0.9).round(),
          (c.primary.b * 0.9).round(),
        ) : c.primary;
        fg = c.primaryForeground;
        if (_focused) {
          shadows = [BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 2)];
        } else if (_hovered) {
          shadows = [BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 8, offset: const Offset(0, 2))];
        }
      case SomaButtonVariant.secondary:
        bg = _hovered
            ? Color.alphaBlend(Colors.white.withAlpha(20), c.secondary)
            : c.secondary;
        fg = c.secondaryForeground;
        if (_focused) {
          shadows = [BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 2)];
        } else if (_hovered) {
          shadows = [BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 8, offset: const Offset(0, 2))];
        }
      case SomaButtonVariant.destructive:
        bg = _hovered ? Color.fromARGB(
          255,
          (c.destructive.r * 0.9).round(),
          (c.destructive.g * 0.9).round(),
          (c.destructive.b * 0.9).round(),
        ) : c.destructive;
        fg = c.destructiveForeground;
        if (_focused) {
          shadows = [BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 2)];
        } else if (_hovered) {
          shadows = [BoxShadow(color: Colors.black.withAlpha(20), blurRadius: 8, offset: const Offset(0, 2))];
        }
      case SomaButtonVariant.outline:
        bg = _hovered ? c.accent : Colors.transparent;
        fg = _hovered ? c.accentForeground : c.foreground;
        borderColor = c.border;
        if (_focused) {
          shadows = [BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 2)];
        } else if (_hovered) {
          shadows = [BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 6, offset: const Offset(0, 1))];
        }
      case SomaButtonVariant.ghost:
        bg = _hovered ? c.accent : Colors.transparent;
        fg = _hovered ? c.accentForeground : c.foreground;
        if (_focused) {
          shadows = [BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 2)];
        } else if (_hovered) {
          shadows = [BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 6, offset: const Offset(0, 1))];
        }
      case SomaButtonVariant.link:
        bg = Colors.transparent;
        fg = c.primary;
        underline = _hovered;
    }

    // Primary variant gets a subtle top-edge gradient highlight
    final bool isPrimary = widget.variant == SomaButtonVariant.primary;

    // Resolve size
    double height;
    double horizontalPadding;
    double fontSize;
    double? fixedWidth;

    switch (widget.size) {
      case SomaButtonSize.sm:
        height = 32;
        horizontalPadding = 12;
        fontSize = 12;
      case SomaButtonSize.md:
        height = 40;
        horizontalPadding = 16;
        fontSize = 14;
      case SomaButtonSize.lg:
        height = 48;
        horizontalPadding = 24;
        fontSize = 16;
      case SomaButtonSize.icon:
        height = 40;
        horizontalPadding = 0;
        fontSize = 14;
        fixedWidth = 40;
    }

    Widget buttonContent = DefaultTextStyle(
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: fontSize,
        fontWeight: FontWeight.w500,
        color: fg,
        decoration: underline ? TextDecoration.underline : TextDecoration.none,
        decorationColor: fg,
      ),
      child: IconTheme(
        data: IconThemeData(color: fg, size: fontSize + 2),
        child: widget.child,
      ),
    );

    Widget button = Focus(
      onFocusChange: (focused) => setState(() => _focused = focused),
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        cursor: isEnabled ? SystemMouseCursors.click : SystemMouseCursors.forbidden,
        child: GestureDetector(
          onTap: isEnabled ? widget.onPressed : null,
          onTapDown: isEnabled ? (_) => setState(() => _pressed = true) : null,
          onTapUp: isEnabled ? (_) => setState(() => _pressed = false) : null,
          onTapCancel: isEnabled ? () => setState(() => _pressed = false) : null,
          child: AnimatedScale(
            scale: _pressed ? 0.97 : 1.0,
            duration: const Duration(milliseconds: 140),
            curve: Curves.easeOutCubic,
            child: AnimatedOpacity(
              opacity: isEnabled ? 1.0 : 0.5,
              duration: const Duration(milliseconds: 140),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 140),
                curve: Curves.easeOutCubic,
                height: height,
                width: fixedWidth,
                padding: widget.size == SomaButtonSize.icon
                    ? EdgeInsets.zero
                    : EdgeInsets.symmetric(horizontal: horizontalPadding),
                decoration: BoxDecoration(
                  color: isPrimary ? null : bg,
                  gradient: isPrimary
                      ? LinearGradient(
                          begin: Alignment.topCenter,
                          end: Alignment.bottomCenter,
                          colors: [
                            Color.alphaBlend(Colors.white.withAlpha(18), bg),
                            bg,
                          ],
                        )
                      : null,
                  borderRadius: BorderRadius.circular(6),
                  border: borderColor != null
                      ? Border.all(color: borderColor)
                      : null,
                  boxShadow: shadows,
                ),
                child: Center(
                  widthFactor: widget.size == SomaButtonSize.icon ? null : 1.0,
                  child: buttonContent,
                ),
              ),
            ),
          ),
        ),
      ),
    );

    return button;
  }
}
