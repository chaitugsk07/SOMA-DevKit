import 'package:flutter/material.dart';
import '../theme/soma_colors.dart';

class SomaChartPoint {
  final String label;
  final double value;
  const SomaChartPoint({required this.label, required this.value});
}

class SomaChartSeries {
  final String name;
  final List<SomaChartPoint> points;
  final Color? color;
  const SomaChartSeries({required this.points, this.name = '', this.color});
}

List<Color> somaChartPalette(SomaColors c) => [
      c.primary,
      HSLColor.fromAHSL(1, 200, 0.70, 0.55).toColor(),
      HSLColor.fromAHSL(1, 150, 0.60, 0.50).toColor(),
      HSLColor.fromAHSL(1, 30, 0.80, 0.55).toColor(),
      c.destructive,
      HSLColor.fromAHSL(1, 262, 0.80, 0.50).toColor(),
    ];

List<Color> somaPieChartPalette(SomaColors c) => [
      c.primary,
      c.destructive,
      HSLColor.fromAHSL(1, 221, 0.83, 0.53).toColor(),
      HSLColor.fromAHSL(1, 262, 0.80, 0.50).toColor(),
      HSLColor.fromAHSL(1, 38, 0.92, 0.50).toColor(),
      HSLColor.fromAHSL(1, 160, 0.60, 0.45).toColor(),
    ];

// Web geometry constants (320×200 viewBox)
const double kChartPadL = 30;
const double kChartPadR = 10;
const double kChartPadT = 10;
const double kChartPadB = 30;

/// Draw 4 horizontal dashed gridlines + bottom axis + left axis border lines.
void drawCartesianAxes(Canvas canvas, Size size, Color borderColor, Color gridColor) {
  final borderPaint = Paint()
    ..color = borderColor
    ..strokeWidth = 1
    ..isAntiAlias = true;

  final gridPaint = Paint()
    ..color = gridColor
    ..strokeWidth = 0.5
    ..isAntiAlias = true;

  final plotH = size.height - kChartPadT - kChartPadB;
  final plotBottom = size.height - kChartPadB;

  // Bottom axis line
  canvas.drawLine(
    Offset(kChartPadL, plotBottom),
    Offset(size.width - kChartPadR, plotBottom),
    borderPaint,
  );
  // Left axis line
  canvas.drawLine(
    Offset(kChartPadL, kChartPadT),
    Offset(kChartPadL, plotBottom),
    borderPaint,
  );

  // 4 horizontal dashed gridlines
  const gridCount = 4;
  for (int i = 1; i <= gridCount; i++) {
    final y = kChartPadT + plotH * (1 - i / gridCount);
    _drawDashedLine(
      canvas,
      Offset(kChartPadL, y),
      Offset(size.width - kChartPadR, y),
      gridPaint,
      4,
      4,
    );
  }
}

/// Draw x-axis labels at evenly spaced positions.
void drawXLabels(Canvas canvas, Size size, List<String> labels, Color color, double Function(int i) xOf) {
  for (int i = 0; i < labels.length; i++) {
    final tp = TextPainter(
      text: TextSpan(
        text: labels[i],
        style: TextStyle(color: color, fontSize: 8, fontFamily: 'Outfit'),
      ),
      textDirection: TextDirection.ltr,
    )..layout();
    tp.paint(canvas, Offset(xOf(i) - tp.width / 2, size.height - kChartPadB + 4));
  }
}

void _drawDashedLine(Canvas canvas, Offset p1, Offset p2, Paint paint, double dash, double gap) {
  final dx = p2.dx - p1.dx;
  final dy = p2.dy - p1.dy;
  final dist = (p2 - p1).distance;
  final count = (dist / (dash + gap)).floor();
  for (int i = 0; i <= count; i++) {
    final t0 = i * (dash + gap) / dist;
    final t1 = (i * (dash + gap) + dash) / dist;
    canvas.drawLine(
      Offset(p1.dx + dx * t0, p1.dy + dy * t0),
      Offset(p1.dx + dx * t1.clamp(0, 1), p1.dy + dy * t1.clamp(0, 1)),
      paint,
    );
  }
}
