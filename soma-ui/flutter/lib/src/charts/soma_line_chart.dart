import 'package:flutter/material.dart';
import '../theme/soma_colors.dart';
import '../theme/soma_theme.dart';
import 'chart_common.dart';

enum SomaLineChartVariant { default_, linear, step, dots, multiple }

class SomaLineChart extends StatelessWidget {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaLineChartVariant variant;

  const SomaLineChart({
    super.key,
    this.data = const [],
    this.series = const [],
    this.variant = SomaLineChartVariant.default_,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return AspectRatio(
      aspectRatio: 320 / 200,
      child: CustomPaint(
        painter: _LinePainter(data: data, series: series, variant: variant, colors: c),
        size: Size.infinite,
      ),
    );
  }
}

class _LinePainter extends CustomPainter {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaLineChartVariant variant;
  final SomaColors colors;

  const _LinePainter({
    required this.data,
    required this.series,
    required this.variant,
    required this.colors,
  });

  @override
  void paint(Canvas canvas, Size size) {
    drawCartesianAxes(canvas, size, colors.border, colors.border.withValues(alpha: 0.4));

    if (variant == SomaLineChartVariant.multiple) {
      _paintMultiple(canvas, size);
      return;
    }

    final pts = data.isNotEmpty
        ? data
        : (series.isNotEmpty ? series[0].points : const <SomaChartPoint>[]);
    if (pts.isEmpty) return;

    _paintSeries(canvas, size, pts, colors.primary);
    drawXLabels(canvas, size, pts.map((p) => p.label).toList(), colors.mutedForeground,
        (i) => _xOf(size, pts.length, i));
  }

  double _xOf(Size size, int count, int i) {
    if (count <= 1) return kChartPadL + (size.width - kChartPadL - kChartPadR) / 2;
    return kChartPadL + i * (size.width - kChartPadL - kChartPadR) / (count - 1);
  }

  double _yOf(Size size, double value, double minV, double maxV) {
    final plotH = size.height - kChartPadT - kChartPadB;
    if (maxV == minV) return kChartPadT + plotH / 2;
    return kChartPadT + plotH * (1 - (value - minV) / (maxV - minV));
  }

  void _paintSeries(Canvas canvas, Size size, List<SomaChartPoint> pts, Color color) {
    final values = pts.map((p) => p.value).toList();
    final minV = values.reduce((a, b) => a < b ? a : b);
    final maxV = values.reduce((a, b) => a > b ? a : b);

    final path = Path();
    path.moveTo(_xOf(size, pts.length, 0), _yOf(size, pts[0].value, minV, maxV));

    if (variant == SomaLineChartVariant.step) {
      for (int i = 1; i < pts.length; i++) {
        final xi = _xOf(size, pts.length, i);
        final yi = _yOf(size, pts[i].value, minV, maxV);
        path.lineTo(xi, _yOf(size, pts[i - 1].value, minV, maxV));
        path.lineTo(xi, yi);
      }
    } else if (variant == SomaLineChartVariant.linear || variant == SomaLineChartVariant.dots) {
      for (int i = 1; i < pts.length; i++) {
        path.lineTo(_xOf(size, pts.length, i), _yOf(size, pts[i].value, minV, maxV));
      }
    } else {
      // Smooth bezier (default)
      for (int i = 1; i < pts.length; i++) {
        final x0 = _xOf(size, pts.length, i - 1);
        final y0 = _yOf(size, pts[i - 1].value, minV, maxV);
        final x1 = _xOf(size, pts.length, i);
        final y1 = _yOf(size, pts[i].value, minV, maxV);
        final cx = (x0 + x1) / 2;
        path.cubicTo(cx, y0, cx, y1, x1, y1);
      }
    }

    canvas.drawPath(
      path,
      Paint()
        ..color = color
        ..strokeWidth = 1.5
        ..style = PaintingStyle.stroke
        ..isAntiAlias = true,
    );

    if (variant == SomaLineChartVariant.dots) {
      final dotPaint = Paint()
        ..color = color
        ..isAntiAlias = true;
      for (int i = 0; i < pts.length; i++) {
        canvas.drawCircle(
          Offset(_xOf(size, pts.length, i), _yOf(size, pts[i].value, minV, maxV)),
          3.5,
          dotPaint,
        );
      }
    }
  }

  void _paintMultiple(Canvas canvas, Size size) {
    final allSeries = series.isNotEmpty ? series : [SomaChartSeries(points: data)];
    if (allSeries.isEmpty) return;

    final palette = somaChartPalette(colors);
    double minV = double.infinity, maxV = double.negativeInfinity;
    for (final s in allSeries) {
      for (final p in s.points) {
        if (p.value < minV) minV = p.value;
        if (p.value > maxV) maxV = p.value;
      }
    }
    if (minV == double.infinity) return;

    for (int si = 0; si < allSeries.length; si++) {
      final pts = allSeries[si].points;
      if (pts.isEmpty) continue;
      final color = allSeries[si].color ?? palette[si % palette.length];
      final path = Path();
      path.moveTo(_xOf(size, pts.length, 0), _yOf(size, pts[0].value, minV, maxV));
      for (int i = 1; i < pts.length; i++) {
        final x0 = _xOf(size, pts.length, i - 1);
        final y0 = _yOf(size, pts[i - 1].value, minV, maxV);
        final x1 = _xOf(size, pts.length, i);
        final y1 = _yOf(size, pts[i].value, minV, maxV);
        final cx = (x0 + x1) / 2;
        path.cubicTo(cx, y0, cx, y1, x1, y1);
      }
      canvas.drawPath(
        path,
        Paint()
          ..color = color
          ..strokeWidth = 1.5
          ..style = PaintingStyle.stroke
          ..isAntiAlias = true,
      );
    }

    final firstPts = allSeries[0].points;
    drawXLabels(canvas, size, firstPts.map((p) => p.label).toList(), colors.mutedForeground,
        (i) => _xOf(size, firstPts.length, i));
  }

  @override
  bool shouldRepaint(_LinePainter old) =>
      old.variant != variant ||
      old.data != data ||
      old.series != series ||
      old.colors != colors;
}
