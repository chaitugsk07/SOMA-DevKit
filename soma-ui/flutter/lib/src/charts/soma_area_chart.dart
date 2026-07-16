import 'package:flutter/material.dart';
import '../theme/soma_colors.dart';
import '../theme/soma_theme.dart';
import 'chart_common.dart';

enum SomaAreaChartVariant { default_, linear, step, gradient, stacked }

class SomaAreaChart extends StatelessWidget {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaAreaChartVariant variant;

  const SomaAreaChart({
    super.key,
    this.data = const [],
    this.series = const [],
    this.variant = SomaAreaChartVariant.default_,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return AspectRatio(
      aspectRatio: 320 / 200,
      child: CustomPaint(
        painter: _AreaPainter(
          data: data,
          series: series,
          variant: variant,
          colors: c,
        ),
        size: Size.infinite,
      ),
    );
  }
}

class _AreaPainter extends CustomPainter {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaAreaChartVariant variant;
  final SomaColors colors;

  const _AreaPainter({
    required this.data,
    required this.series,
    required this.variant,
    required this.colors,
  });

  @override
  void paint(Canvas canvas, Size size) {
    drawCartesianAxes(canvas, size, colors.border, colors.border.withValues(alpha: 0.4));

    if (variant == SomaAreaChartVariant.stacked) {
      _paintStacked(canvas, size);
      return;
    }

    final pts = data.isNotEmpty
        ? data
        : (series.isNotEmpty ? series[0].points : const <SomaChartPoint>[]);
    if (pts.isEmpty) return;
    _paintSingle(canvas, size, pts, colors.primary);
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

  void _paintSingle(Canvas canvas, Size size, List<SomaChartPoint> pts, Color color) {
    final values = pts.map((p) => p.value).toList();
    final minV = values.reduce((a, b) => a < b ? a : b);
    final maxV = values.reduce((a, b) => a > b ? a : b);
    final plotBottom = size.height - kChartPadB;

    final linePath = _buildLinePath(size, pts, minV, maxV);

    // Area fill
    final areaPath = Path.from(linePath);
    areaPath.lineTo(_xOf(size, pts.length, pts.length - 1), plotBottom);
    areaPath.lineTo(_xOf(size, pts.length, 0), plotBottom);
    areaPath.close();

    if (variant == SomaAreaChartVariant.gradient) {
      final shader = LinearGradient(
        begin: Alignment.topCenter,
        end: Alignment.bottomCenter,
        colors: [color.withValues(alpha: 0.4), color.withValues(alpha: 0)],
      ).createShader(Rect.fromLTWH(0, kChartPadT, size.width, size.height - kChartPadT - kChartPadB));
      canvas.drawPath(areaPath, Paint()..shader = shader..isAntiAlias = true);
    } else {
      final fillAlpha = variant == SomaAreaChartVariant.linear ? 0.15 : 0.2;
      canvas.drawPath(areaPath, Paint()..color = color.withValues(alpha: fillAlpha)..isAntiAlias = true);
    }

    // Line stroke
    canvas.drawPath(
      linePath,
      Paint()
        ..color = color
        ..strokeWidth = 1.5
        ..style = PaintingStyle.stroke
        ..isAntiAlias = true,
    );
  }

  Path _buildLinePath(Size size, List<SomaChartPoint> pts, double minV, double maxV) {
    final path = Path();
    if (pts.isEmpty) return path;
    path.moveTo(_xOf(size, pts.length, 0), _yOf(size, pts[0].value, minV, maxV));

    if (variant == SomaAreaChartVariant.step) {
      for (int i = 1; i < pts.length; i++) {
        final xi = _xOf(size, pts.length, i);
        final yi = _yOf(size, pts[i].value, minV, maxV);
        path.lineTo(xi, _yOf(size, pts[i - 1].value, minV, maxV));
        path.lineTo(xi, yi);
      }
    } else if (variant == SomaAreaChartVariant.linear) {
      for (int i = 1; i < pts.length; i++) {
        path.lineTo(_xOf(size, pts.length, i), _yOf(size, pts[i].value, minV, maxV));
      }
    } else {
      // Smooth cubic bezier (default/gradient)
      for (int i = 1; i < pts.length; i++) {
        final x0 = _xOf(size, pts.length, i - 1);
        final y0 = _yOf(size, pts[i - 1].value, minV, maxV);
        final x1 = _xOf(size, pts.length, i);
        final y1 = _yOf(size, pts[i].value, minV, maxV);
        final cx = (x0 + x1) / 2;
        path.cubicTo(cx, y0, cx, y1, x1, y1);
      }
    }
    return path;
  }

  void _paintStacked(Canvas canvas, Size size) {
    final allSeries = series.isNotEmpty
        ? series
        : [SomaChartSeries(points: data, name: 'Series A')];
    if (allSeries.isEmpty || allSeries[0].points.isEmpty) return;

    final count = allSeries[0].points.length;
    final palette = somaChartPalette(colors);

    // Compute stacked tops and bottoms per series
    final List<double> stackedTops = List.filled(count, 0.0);
    final List<List<double>> seriesBottoms = [];
    final List<List<double>> seriesTops = [];

    for (final s in allSeries) {
      seriesBottoms.add(List<double>.from(stackedTops));
      for (int i = 0; i < count; i++) {
        stackedTops[i] += (i < s.points.length ? s.points[i].value : 0);
      }
      seriesTops.add(List<double>.from(stackedTops));
    }

    final maxV = stackedTops.reduce((a, b) => a > b ? a : b);
    const minV = 0.0;
    final plotBottom = size.height - kChartPadB;

    for (int si = 0; si < allSeries.length; si++) {
      final color = palette[si % palette.length];
      final topVals = seriesTops[si];
      final botVals = seriesBottoms[si];

      // Forward: top bezier
      final path = Path();
      path.moveTo(_xOf(size, count, 0), _yOf(size, topVals[0], minV, maxV));
      for (int i = 1; i < count; i++) {
        final x0 = _xOf(size, count, i - 1);
        final y0 = _yOf(size, topVals[i - 1], minV, maxV);
        final x1 = _xOf(size, count, i);
        final y1 = _yOf(size, topVals[i], minV, maxV);
        final cx = (x0 + x1) / 2;
        path.cubicTo(cx, y0, cx, y1, x1, y1);
      }
      // Backward: bottom line
      for (int i = count - 1; i >= 0; i--) {
        final xi = _xOf(size, count, i);
        final yi = si == 0 ? plotBottom : _yOf(size, botVals[i], minV, maxV);
        path.lineTo(xi, yi);
      }
      path.close();

      canvas.drawPath(path, Paint()..color = color.withValues(alpha: 0.25)..isAntiAlias = true);

      // Top stroke
      final strokePath = Path();
      strokePath.moveTo(_xOf(size, count, 0), _yOf(size, topVals[0], minV, maxV));
      for (int i = 1; i < count; i++) {
        final x0 = _xOf(size, count, i - 1);
        final y0 = _yOf(size, topVals[i - 1], minV, maxV);
        final x1 = _xOf(size, count, i);
        final y1 = _yOf(size, topVals[i], minV, maxV);
        final cx = (x0 + x1) / 2;
        strokePath.cubicTo(cx, y0, cx, y1, x1, y1);
      }
      canvas.drawPath(
        strokePath,
        Paint()
          ..color = color
          ..strokeWidth = 1.5
          ..style = PaintingStyle.stroke
          ..isAntiAlias = true,
      );
    }

    drawXLabels(canvas, size, allSeries[0].points.map((p) => p.label).toList(),
        colors.mutedForeground, (i) => _xOf(size, count, i));
  }

  @override
  bool shouldRepaint(_AreaPainter old) =>
      old.variant != variant ||
      old.data != data ||
      old.series != series ||
      old.colors != colors;
}
