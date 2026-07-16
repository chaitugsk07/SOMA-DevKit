import 'dart:math' as math;
import 'package:flutter/material.dart';
import '../theme/soma_colors.dart';
import '../theme/soma_theme.dart';
import 'chart_common.dart';

enum SomaPieChartVariant { pie, donut, labeled, half }

class SomaPieChart extends StatelessWidget {
  final List<SomaChartPoint> data;
  final SomaPieChartVariant variant;

  const SomaPieChart({
    super.key,
    this.data = const [],
    this.variant = SomaPieChartVariant.pie,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    // half variant crops to upper half → 200/120 aspect ratio
    final aspectRatio = variant == SomaPieChartVariant.half ? 200 / 120 : 1.0;
    return AspectRatio(
      aspectRatio: aspectRatio,
      child: CustomPaint(
        painter: _PiePainter(data: data, variant: variant, colors: c),
        size: Size.infinite,
      ),
    );
  }
}

class _PiePainter extends CustomPainter {
  final List<SomaChartPoint> data;
  final SomaPieChartVariant variant;
  final SomaColors colors;

  const _PiePainter({
    required this.data,
    required this.variant,
    required this.colors,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (data.isEmpty) return;

    final palette = somaPieChartPalette(colors);
    final total = data.fold(0.0, (s, p) => s + p.value);
    if (total == 0) return;

    final cx = size.width / 2;
    // For half variant, center is at bottom of canvas so upper half shows
    final cy = variant == SomaPieChartVariant.half ? size.height : size.height / 2;
    final outerR = variant == SomaPieChartVariant.half
        ? size.height * 0.85
        : (size.width < size.height ? size.width : size.height) * 0.42;
    final isDonut = variant == SomaPieChartVariant.donut ||
        variant == SomaPieChartVariant.labeled ||
        variant == SomaPieChartVariant.half;
    final innerR = isDonut ? outerR * 0.56 : 0.0;

    final separatorPaint = Paint()
      ..color = colors.background
      ..strokeWidth = 1.5
      ..style = PaintingStyle.stroke
      ..isAntiAlias = true;

    final sweepStart = variant == SomaPieChartVariant.half ? math.pi : -math.pi / 2;
    final fullSweep = variant == SomaPieChartVariant.half ? math.pi : 2 * math.pi;

    double angle = sweepStart;
    final slices = <({double startAngle, double sweep, Color color, SomaChartPoint point})>[];

    for (int i = 0; i < data.length; i++) {
      final sweep = fullSweep * (data[i].value / total);
      slices.add((startAngle: angle, sweep: sweep, color: palette[i % palette.length], point: data[i]));
      angle += sweep;
    }

    // Draw slices
    for (final s in slices) {
      final paint = Paint()
        ..color = s.color
        ..isAntiAlias = true;

      if (isDonut) {
        _drawAnnularSector(canvas, cx, cy, outerR, innerR, s.startAngle, s.sweep, paint);
      } else {
        final path = Path()
          ..moveTo(cx, cy)
          ..arcTo(Rect.fromCircle(center: Offset(cx, cy), radius: outerR), s.startAngle, s.sweep, false)
          ..close();
        canvas.drawPath(path, paint);
      }

      // Separator line at start of each slice
      final sx = cx + outerR * math.cos(s.startAngle);
      final sy = cy + outerR * math.sin(s.startAngle);
      final ex = cx + (isDonut ? innerR : 0) * math.cos(s.startAngle);
      final ey = cy + (isDonut ? innerR : 0) * math.sin(s.startAngle);
      canvas.drawLine(Offset(sx, sy), Offset(ex, ey), separatorPaint);
    }

    // Labeled: percentage labels + center total
    if (variant == SomaPieChartVariant.labeled) {
      for (final s in slices) {
        final midAngle = s.startAngle + s.sweep / 2;
        final labelR = (outerR + innerR) / 2;
        final lx = cx + labelR * math.cos(midAngle);
        final ly = cy + labelR * math.sin(midAngle);
        final pct = '${(s.point.value / total * 100).round()}%';
        final tp = TextPainter(
          text: TextSpan(
            text: pct,
            style: const TextStyle(color: Colors.white, fontSize: 9, fontWeight: FontWeight.bold),
          ),
          textDirection: TextDirection.ltr,
        )..layout();
        tp.paint(canvas, Offset(lx - tp.width / 2, ly - tp.height / 2));
      }

      // Center total
      final totalStr = total.toStringAsFixed(0);
      final totalTp = TextPainter(
        text: TextSpan(
          text: totalStr,
          style: TextStyle(
            color: colors.foreground,
            fontSize: 18,
            fontWeight: FontWeight.bold,
            fontFamily: 'Rajdhani',
          ),
        ),
        textDirection: TextDirection.ltr,
      )..layout();
      totalTp.paint(canvas, Offset(cx - totalTp.width / 2, cy - totalTp.height / 2));
    }
  }

  void _drawAnnularSector(Canvas canvas, double cx, double cy, double outerR, double innerR,
      double startAngle, double sweep, Paint paint) {
    final path = Path();
    final outerRect = Rect.fromCircle(center: Offset(cx, cy), radius: outerR);
    final innerRect = Rect.fromCircle(center: Offset(cx, cy), radius: innerR);

    path.moveTo(cx + outerR * math.cos(startAngle), cy + outerR * math.sin(startAngle));
    path.arcTo(outerRect, startAngle, sweep, false);
    path.arcTo(innerRect, startAngle + sweep, -sweep, false);
    path.close();
    canvas.drawPath(path, paint);
  }

  @override
  bool shouldRepaint(_PiePainter old) =>
      old.variant != variant || old.data != data || old.colors != colors;
}
