import 'package:flutter/material.dart';
import '../theme/soma_colors.dart';
import '../theme/soma_theme.dart';
import 'chart_common.dart';

enum SomaBarChartVariant { default_, horizontal, stacked, grouped }

class SomaBarChart extends StatelessWidget {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaBarChartVariant variant;

  const SomaBarChart({
    super.key,
    this.data = const [],
    this.series = const [],
    this.variant = SomaBarChartVariant.default_,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return AspectRatio(
      aspectRatio: 320 / 200,
      child: CustomPaint(
        painter: _BarPainter(data: data, series: series, variant: variant, colors: c),
        size: Size.infinite,
      ),
    );
  }
}

class _BarPainter extends CustomPainter {
  final List<SomaChartPoint> data;
  final List<SomaChartSeries> series;
  final SomaBarChartVariant variant;
  final SomaColors colors;

  const _BarPainter({
    required this.data,
    required this.series,
    required this.variant,
    required this.colors,
  });

  @override
  void paint(Canvas canvas, Size size) {
    switch (variant) {
      case SomaBarChartVariant.default_:
        _paintVertical(canvas, size);
      case SomaBarChartVariant.horizontal:
        _paintHorizontal(canvas, size);
      case SomaBarChartVariant.stacked:
        _paintStacked(canvas, size);
      case SomaBarChartVariant.grouped:
        _paintGrouped(canvas, size);
    }
  }

  List<SomaChartPoint> get _pts =>
      data.isNotEmpty ? data : (series.isNotEmpty ? series[0].points : const <SomaChartPoint>[]);

  void _paintVertical(Canvas canvas, Size size) {
    drawCartesianAxes(canvas, size, colors.border, colors.border.withValues(alpha: 0.4));
    final pts = _pts;
    if (pts.isEmpty) return;

    final values = pts.map((p) => p.value).toList();
    final maxV = values.reduce((a, b) => a > b ? a : b);
    final plotW = size.width - kChartPadL - kChartPadR;
    final plotH = size.height - kChartPadT - kChartPadB;
    final plotBottom = size.height - kChartPadB;
    final barW = plotW / pts.length * 0.6;
    final gap = plotW / pts.length;

    final paint = Paint()
      ..color = colors.primary
      ..isAntiAlias = true;

    for (int i = 0; i < pts.length; i++) {
      final x = kChartPadL + gap * i + gap / 2 - barW / 2;
      final barH = maxV == 0 ? 0.0 : plotH * (pts[i].value / maxV);
      final top = plotBottom - barH;
      final rect = Rect.fromLTWH(x, top, barW, barH);
      canvas.drawRRect(
        RRect.fromRectAndCorners(rect, topLeft: const Radius.circular(2), topRight: const Radius.circular(2)),
        paint,
      );
    }

    drawXLabels(canvas, size, pts.map((p) => p.label).toList(), colors.mutedForeground,
        (i) => kChartPadL + gap * i + gap / 2);
  }

  void _paintHorizontal(Canvas canvas, Size size) {
    final pts = _pts;
    if (pts.isEmpty) return;

    final borderPaint = Paint()
      ..color = colors.border
      ..strokeWidth = 1
      ..isAntiAlias = true;

    final values = pts.map((p) => p.value).toList();
    final maxV = values.reduce((a, b) => a > b ? a : b);
    final plotW = size.width - kChartPadL - kChartPadR;
    final plotH = size.height - kChartPadT - kChartPadB;
    final barH = plotH / pts.length * 0.6;
    final gap = plotH / pts.length;

    // Axis lines
    canvas.drawLine(Offset(kChartPadL, kChartPadT), Offset(kChartPadL, size.height - kChartPadB), borderPaint);
    canvas.drawLine(Offset(kChartPadL, size.height - kChartPadB), Offset(size.width - kChartPadR, size.height - kChartPadB), borderPaint);

    final paint = Paint()
      ..color = colors.primary
      ..isAntiAlias = true;

    for (int i = 0; i < pts.length; i++) {
      final y = kChartPadT + gap * i + gap / 2 - barH / 2;
      final barW = maxV == 0 ? 0.0 : plotW * (pts[i].value / maxV);
      final rect = Rect.fromLTWH(kChartPadL, y, barW, barH);
      canvas.drawRRect(
        RRect.fromRectAndCorners(rect, topRight: const Radius.circular(2), bottomRight: const Radius.circular(2)),
        paint,
      );

      // Y-label
      final tp = TextPainter(
        text: TextSpan(
          text: pts[i].label,
          style: TextStyle(color: colors.mutedForeground, fontSize: 8, fontFamily: 'Outfit'),
        ),
        textDirection: TextDirection.ltr,
      )..layout();
      tp.paint(canvas, Offset(kChartPadL - tp.width - 4, y + barH / 2 - tp.height / 2));
    }
  }

  void _paintStacked(Canvas canvas, Size size) {
    drawCartesianAxes(canvas, size, colors.border, colors.border.withValues(alpha: 0.4));
    final allSeries = series.isNotEmpty ? series : [SomaChartSeries(points: data)];
    if (allSeries.isEmpty || allSeries[0].points.isEmpty) return;

    final count = allSeries[0].points.length;
    final palette = somaChartPalette(colors);
    final plotW = size.width - kChartPadL - kChartPadR;
    final plotH = size.height - kChartPadT - kChartPadB;
    final plotBottom = size.height - kChartPadB;
    final barW = plotW / count * 0.6;
    final gap = plotW / count;

    double maxTotal = 0;
    for (int i = 0; i < count; i++) {
      double total = 0;
      for (final s in allSeries) {
        total += i < s.points.length ? s.points[i].value : 0;
      }
      if (total > maxTotal) maxTotal = total;
    }

    for (int i = 0; i < count; i++) {
      double stackBottom = plotBottom;
      for (int si = 0; si < allSeries.length; si++) {
        final v = i < allSeries[si].points.length ? allSeries[si].points[i].value : 0.0;
        final barH = maxTotal == 0 ? 0.0 : plotH * (v / maxTotal);
        final x = kChartPadL + gap * i + gap / 2 - barW / 2;
        final top = stackBottom - barH;
        final rect = Rect.fromLTWH(x, top, barW, barH);
        canvas.drawRect(rect, Paint()..color = palette[si % palette.length]..isAntiAlias = true);
        stackBottom = top;
      }
    }

    drawXLabels(canvas, size, allSeries[0].points.map((p) => p.label).toList(),
        colors.mutedForeground, (i) => kChartPadL + gap * i + gap / 2);
  }

  void _paintGrouped(Canvas canvas, Size size) {
    drawCartesianAxes(canvas, size, colors.border, colors.border.withValues(alpha: 0.4));
    final allSeries = series.isNotEmpty ? series : [SomaChartSeries(points: data)];
    if (allSeries.isEmpty || allSeries[0].points.isEmpty) return;

    final count = allSeries[0].points.length;
    final palette = somaChartPalette(colors);
    final plotW = size.width - kChartPadL - kChartPadR;
    final plotH = size.height - kChartPadT - kChartPadB;
    final plotBottom = size.height - kChartPadB;
    final gap = plotW / count;
    final groupW = gap * 0.7;
    final barW = groupW / allSeries.length;

    double maxV = 0;
    for (final s in allSeries) {
      for (final p in s.points) {
        if (p.value > maxV) maxV = p.value;
      }
    }

    for (int i = 0; i < count; i++) {
      final groupX = kChartPadL + gap * i + gap / 2 - groupW / 2;
      for (int si = 0; si < allSeries.length; si++) {
        final v = i < allSeries[si].points.length ? allSeries[si].points[i].value : 0.0;
        final barH = maxV == 0 ? 0.0 : plotH * (v / maxV);
        final x = groupX + si * barW;
        final top = plotBottom - barH;
        final rect = Rect.fromLTWH(x, top, barW - 1, barH);
        canvas.drawRRect(
          RRect.fromRectAndCorners(rect, topLeft: const Radius.circular(2), topRight: const Radius.circular(2)),
          Paint()..color = palette[si % palette.length]..isAntiAlias = true,
        );
      }
    }

    drawXLabels(canvas, size, allSeries[0].points.map((p) => p.label).toList(),
        colors.mutedForeground, (i) => kChartPadL + gap * i + gap / 2);
  }

  @override
  bool shouldRepaint(_BarPainter old) =>
      old.variant != variant ||
      old.data != data ||
      old.series != series ||
      old.colors != colors;
}
