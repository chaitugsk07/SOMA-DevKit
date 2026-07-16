import 'dart:math' as math;
import 'package:flutter/material.dart';
import '../theme/soma_colors.dart';
import '../theme/soma_theme.dart';
import 'chart_common.dart';

enum SomaRadialChartVariant { default_, stacked, labeled, track }

class SomaRadialChart extends StatelessWidget {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaRadialChartVariant variant;

  const SomaRadialChart({
    super.key,
    this.data = const [],
    this.series = const [],
    this.variant = SomaRadialChartVariant.default_,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return AspectRatio(
      aspectRatio: 1.0,
      child: CustomPaint(
        painter: _RadialPainter(data: data, series: series, variant: variant, colors: c),
        size: Size.infinite,
      ),
    );
  }
}

class _RadialPainter extends CustomPainter {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaRadialChartVariant variant;
  final SomaColors colors;

  const _RadialPainter({
    required this.data,
    required this.series,
    required this.variant,
    required this.colors,
  });

  static const double _trackWidth = 10;
  static const double _gap = 4;

  @override
  void paint(Canvas canvas, Size size) {
    if (variant == SomaRadialChartVariant.stacked) {
      _paintStacked(canvas, size);
    } else {
      _paintConcentric(canvas, size);
    }
  }

  void _paintConcentric(Canvas canvas, Size size) {
    final pts = data.isNotEmpty
        ? data
        : (series.isNotEmpty ? series[0].points : const <SomaChartPoint>[]);
    if (pts.isEmpty) return;

    final cx = size.width / 2;
    final cy = size.height / 2;
    final palette = somaChartPalette(colors);
    final outerR = (size.width < size.height ? size.width : size.height) / 2 - 10;

    final isTrack = variant == SomaRadialChartVariant.track;
    final trackBgAlpha = isTrack ? 0.5 : 0.3;
    final lineCap = isTrack ? StrokeCap.butt : StrokeCap.round;

    for (int i = 0; i < pts.length; i++) {
      final r = outerR - i * (_trackWidth + _gap);
      if (r <= 0) break;

      final rect = Rect.fromCircle(center: Offset(cx, cy), radius: r);
      canvas.drawArc(
        rect,
        -math.pi / 2,
        2 * math.pi,
        false,
        Paint()
          ..color = colors.muted.withValues(alpha: trackBgAlpha)
          ..strokeWidth = _trackWidth
          ..style = PaintingStyle.stroke
          ..isAntiAlias = true,
      );

      final sweep = 2 * math.pi * (pts[i].value / 100).clamp(0.0, 1.0);
      canvas.drawArc(
        rect,
        -math.pi / 2,
        sweep,
        false,
        Paint()
          ..color = palette[i % palette.length]
          ..strokeWidth = _trackWidth
          ..style = PaintingStyle.stroke
          ..strokeCap = lineCap
          ..isAntiAlias = true,
      );
    }

    if (variant == SomaRadialChartVariant.labeled && pts.isNotEmpty) {
      final total = pts.fold(0.0, (s, p) => s + p.value);
      final totalStr = total.toStringAsFixed(0);

      final totalTp = TextPainter(
        text: TextSpan(
          text: totalStr,
          style: TextStyle(
            color: colors.foreground,
            fontSize: 22,
            fontWeight: FontWeight.bold,
            fontFamily: 'Rajdhani',
          ),
        ),
        textDirection: TextDirection.ltr,
      )..layout();

      final labelTp = TextPainter(
        text: TextSpan(
          text: 'total',
          style: TextStyle(color: colors.mutedForeground, fontSize: 10, fontFamily: 'Outfit'),
        ),
        textDirection: TextDirection.ltr,
      )..layout();

      totalTp.paint(canvas, Offset(cx - totalTp.width / 2, cy - totalTp.height / 2 - 6));
      labelTp.paint(canvas, Offset(cx - labelTp.width / 2, cy + totalTp.height / 2 - 4));
    }
  }

  void _paintStacked(Canvas canvas, Size size) {
    final allSeries = series.isNotEmpty ? series : [SomaChartSeries(points: data)];
    if (allSeries.isEmpty) return;
    final count = allSeries[0].points.length;
    if (count == 0) return;

    final cx = size.width / 2;
    final cy = size.height / 2;
    final palette = somaChartPalette(colors);
    final outerR = (size.width < size.height ? size.width : size.height) / 2 - 10;

    for (int i = 0; i < count; i++) {
      final r = outerR - i * (_trackWidth + _gap);
      if (r <= 0) break;

      final rect = Rect.fromCircle(center: Offset(cx, cy), radius: r);

      // Track background
      canvas.drawArc(
        rect,
        -math.pi / 2,
        2 * math.pi,
        false,
        Paint()
          ..color = colors.muted.withValues(alpha: 0.3)
          ..strokeWidth = _trackWidth
          ..style = PaintingStyle.stroke
          ..isAntiAlias = true,
      );

      // Each series stacked as arc segments within this ring
      double angleOffset = -math.pi / 2;
      for (int si = 0; si < allSeries.length; si++) {
        final v = i < allSeries[si].points.length ? allSeries[si].points[i].value : 0.0;
        final sweep = 2 * math.pi * (v / 100).clamp(0.0, 1.0);
        if (sweep <= 0) continue;
        canvas.drawArc(
          rect,
          angleOffset,
          sweep,
          false,
          Paint()
            ..color = palette[si % palette.length]
            ..strokeWidth = _trackWidth
            ..style = PaintingStyle.stroke
            ..strokeCap = StrokeCap.round
            ..isAntiAlias = true,
        );
        angleOffset += sweep;
      }
    }
  }

  @override
  bool shouldRepaint(_RadialPainter old) =>
      old.variant != variant ||
      old.data != data ||
      old.series != series ||
      old.colors != colors;
}
