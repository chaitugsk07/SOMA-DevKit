import 'dart:math' as math;
import 'package:flutter/material.dart';
import '../theme/soma_colors.dart';
import '../theme/soma_theme.dart';
import 'chart_common.dart';

enum SomaRadarChartVariant { default_, lines, dots, multiple }

class SomaRadarChart extends StatelessWidget {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaRadarChartVariant variant;

  const SomaRadarChart({
    super.key,
    this.data = const [],
    this.series = const [],
    this.variant = SomaRadarChartVariant.default_,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return AspectRatio(
      aspectRatio: 1.0,
      child: CustomPaint(
        painter: _RadarPainter(data: data, series: series, variant: variant, colors: c),
        size: Size.infinite,
      ),
    );
  }
}

class _RadarPainter extends CustomPainter {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaRadarChartVariant variant;
  final SomaColors colors;

  const _RadarPainter({
    required this.data,
    required this.series,
    required this.variant,
    required this.colors,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final pts = data.isNotEmpty
        ? data
        : (series.isNotEmpty ? series[0].points : const <SomaChartPoint>[]);
    if (pts.isEmpty) return;

    final cx = size.width / 2;
    final cy = size.height / 2;
    final r = (size.width < size.height ? size.width : size.height) * 0.35;
    final n = pts.length;

    // Grid rings (3 concentric polygons)
    final gridPaint = Paint()
      ..color = colors.border.withValues(alpha: 0.5)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 0.5
      ..isAntiAlias = true;

    for (int ring = 1; ring <= 3; ring++) {
      final ringR = r * ring / 3;
      final path = Path();
      for (int i = 0; i < n; i++) {
        final angle = -math.pi / 2 + 2 * math.pi * i / n;
        final x = cx + ringR * math.cos(angle);
        final y = cy + ringR * math.sin(angle);
        if (i == 0) {
          path.moveTo(x, y);
        } else {
          path.lineTo(x, y);
        }
      }
      path.close();
      canvas.drawPath(path, gridPaint);
    }

    // Spokes
    final spokePaint = Paint()
      ..color = colors.border.withValues(alpha: 0.5)
      ..strokeWidth = 0.5
      ..isAntiAlias = true;

    for (int i = 0; i < n; i++) {
      final angle = -math.pi / 2 + 2 * math.pi * i / n;
      canvas.drawLine(
        Offset(cx, cy),
        Offset(cx + r * math.cos(angle), cy + r * math.sin(angle)),
        spokePaint,
      );
    }

    // Axis labels
    for (int i = 0; i < n; i++) {
      final angle = -math.pi / 2 + 2 * math.pi * i / n;
      final lx = cx + (r + 12) * math.cos(angle);
      final ly = cy + (r + 12) * math.sin(angle);
      final tp = TextPainter(
        text: TextSpan(
          text: pts[i].label,
          style: TextStyle(color: colors.mutedForeground, fontSize: 8, fontFamily: 'Outfit'),
        ),
        textDirection: TextDirection.ltr,
      )..layout();
      tp.paint(canvas, Offset(lx - tp.width / 2, ly - tp.height / 2));
    }

    if (variant == SomaRadarChartVariant.multiple) {
      _paintMultiple(canvas, cx, cy, r, n);
    } else {
      _paintSingle(canvas, cx, cy, r, n, pts);
    }
  }

  void _paintSingle(Canvas canvas, double cx, double cy, double r, int n, List<SomaChartPoint> pts) {
    final maxV = pts.map((p) => p.value).reduce((a, b) => a > b ? a : b);
    final polygon = _buildPolygon(cx, cy, r, n, pts, maxV);

    if (variant != SomaRadarChartVariant.lines) {
      final fillAlpha = variant == SomaRadarChartVariant.dots ? 0.20 : 0.25;
      canvas.drawPath(polygon, Paint()..color = colors.primary.withValues(alpha: fillAlpha)..isAntiAlias = true);
    }

    canvas.drawPath(
      polygon,
      Paint()
        ..color = colors.primary
        ..strokeWidth = 1.5
        ..style = PaintingStyle.stroke
        ..isAntiAlias = true,
    );

    if (variant == SomaRadarChartVariant.dots) {
      final dotPaint = Paint()
        ..color = colors.primary
        ..isAntiAlias = true;
      for (int i = 0; i < n; i++) {
        final angle = -math.pi / 2 + 2 * math.pi * i / n;
        final v = i < pts.length ? pts[i].value : 0;
        final vr = maxV == 0 ? 0.0 : r * (v / maxV);
        canvas.drawCircle(Offset(cx + vr * math.cos(angle), cy + vr * math.sin(angle)), 3, dotPaint);
      }
    }
  }

  void _paintMultiple(Canvas canvas, double cx, double cy, double r, int n) {
    final allSeries = series.isNotEmpty ? series : [SomaChartSeries(points: data)];
    final palette = somaChartPalette(colors);

    double maxV = 0;
    for (final s in allSeries) {
      for (final p in s.points) {
        if (p.value > maxV) maxV = p.value;
      }
    }
    if (maxV == 0) return;

    for (int si = 0; si < allSeries.length; si++) {
      final pts = allSeries[si].points;
      if (pts.isEmpty) continue;
      final color = allSeries[si].color ?? palette[si % palette.length];
      final polygon = _buildPolygon(cx, cy, r, n, pts, maxV);
      canvas.drawPath(polygon, Paint()..color = color.withValues(alpha: 0.2)..isAntiAlias = true);
      canvas.drawPath(
        polygon,
        Paint()
          ..color = color
          ..strokeWidth = 1.5
          ..style = PaintingStyle.stroke
          ..isAntiAlias = true,
      );
    }
  }

  Path _buildPolygon(double cx, double cy, double r, int n, List<SomaChartPoint> pts, double maxV) {
    final path = Path();
    for (int i = 0; i < n; i++) {
      final angle = -math.pi / 2 + 2 * math.pi * i / n;
      final v = i < pts.length ? pts[i].value : 0;
      final vr = maxV == 0 ? 0.0 : r * (v / maxV);
      final x = cx + vr * math.cos(angle);
      final y = cy + vr * math.sin(angle);
      if (i == 0) {
        path.moveTo(x, y);
      } else {
        path.lineTo(x, y);
      }
    }
    path.close();
    return path;
  }

  @override
  bool shouldRepaint(_RadarPainter old) =>
      old.variant != variant ||
      old.data != data ||
      old.series != series ||
      old.colors != colors;
}
