import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaSlider extends StatefulWidget {
  final double value;
  final double min;
  final double max;
  final ValueChanged<double>? onChanged;
  final bool enabled;

  const SomaSlider({
    super.key,
    required this.value,
    this.min = 0,
    this.max = 1,
    this.onChanged,
    this.enabled = true,
  });

  @override
  State<SomaSlider> createState() => _SomaSliderState();
}

class _SomaSliderState extends State<SomaSlider> {
  bool _hovered = false;
  bool _focused = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return MouseRegion(
      cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.forbidden,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedOpacity(
        opacity: widget.enabled ? 1.0 : 0.5,
        duration: const Duration(milliseconds: 150),
        child: Focus(
          onFocusChange: (focused) => setState(() => _focused = focused),
          child: SliderTheme(
            data: SliderThemeData(
              trackHeight: 4,
              activeTrackColor: c.primary,
              inactiveTrackColor: c.muted,
              thumbColor: c.primary,
              overlayColor: c.ring.withAlpha(_hovered ? 40 : 0),
              thumbShape: _RefinedThumb(
                primaryColor: c.primary,
                ringColor: c.ring,
                focused: _focused,
              ),
              overlayShape: const RoundSliderOverlayShape(overlayRadius: 20),
            ),
            child: Slider(
              value: widget.value.clamp(widget.min, widget.max),
              min: widget.min,
              max: widget.max,
              onChanged: widget.enabled ? widget.onChanged : null,
            ),
          ),
        ),
      ),
    );
  }
}

class _RefinedThumb extends SliderComponentShape {
  final Color primaryColor;
  final Color ringColor;
  final bool focused;

  const _RefinedThumb({
    required this.primaryColor,
    required this.ringColor,
    required this.focused,
  });

  @override
  Size getPreferredSize(bool isEnabled, bool isDiscrete) => const Size.fromRadius(12);

  @override
  void paint(
    PaintingContext context,
    Offset center, {
    required Animation<double> activationAnimation,
    required Animation<double> enableAnimation,
    required bool isDiscrete,
    required TextPainter labelPainter,
    required RenderBox parentBox,
    required SliderThemeData sliderTheme,
    required TextDirection textDirection,
    required double value,
    required double textScaleFactor,
    required Size sizeWithOverflow,
  }) {
    final canvas = context.canvas;
    const radius = 10.0;

    // 1. Drop shadow
    final shadowPaint = Paint()
      ..color = Colors.black.withAlpha(18)
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 8);
    canvas.drawCircle(center.translate(0, 2), radius, shadowPaint);

    // 2. Focus ring glow (drawn behind the thumb)
    if (focused) {
      final glowPaint = Paint()
        ..color = ringColor.withAlpha(40)
        ..style = PaintingStyle.fill;
      canvas.drawCircle(center, radius + 5, glowPaint);
    }

    // 3. White fill
    final fillPaint = Paint()
      ..color = Colors.white
      ..style = PaintingStyle.fill;
    canvas.drawCircle(center, radius, fillPaint);

    // 4. Primary border 1.5px stroke
    final borderPaint = Paint()
      ..color = primaryColor
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.5;
    canvas.drawCircle(center, radius, borderPaint);
  }
}
