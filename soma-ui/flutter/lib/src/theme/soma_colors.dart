import 'package:flutter/material.dart';

Color _hsl(double h, double s, double l) {
  return HSLColor.fromAHSL(1.0, h, s / 100, l / 100).toColor();
}

class SomaColors {
  final Color background;
  final Color foreground;
  final Color card;
  final Color cardForeground;
  final Color muted;
  final Color mutedForeground;
  final Color primary;
  final Color primaryForeground;
  final Color secondary;
  final Color secondaryForeground;
  final Color accent;
  final Color accentForeground;
  final Color destructive;
  final Color destructiveForeground;
  final Color success;
  final Color successForeground;
  final Color warning;
  final Color warningForeground;
  final Color info;
  final Color infoForeground;
  final Color contrast;
  final Color contrastForeground;
  final Color border;
  final Color input;
  final Color ring;

  const SomaColors({
    required this.background,
    required this.foreground,
    required this.card,
    required this.cardForeground,
    required this.muted,
    required this.mutedForeground,
    required this.primary,
    required this.primaryForeground,
    required this.secondary,
    required this.secondaryForeground,
    required this.accent,
    required this.accentForeground,
    required this.destructive,
    required this.destructiveForeground,
    required this.success,
    required this.successForeground,
    required this.warning,
    required this.warningForeground,
    required this.info,
    required this.infoForeground,
    required this.contrast,
    required this.contrastForeground,
    required this.border,
    required this.input,
    required this.ring,
  });

  /// Light theme — mirrors :root in tokens.css (warm mahogany)
  static final SomaColors light = SomaColors(
    background: _hsl(30, 38, 96),
    foreground: _hsl(19, 24, 13),
    card: _hsl(33, 43, 98),
    cardForeground: _hsl(19, 24, 13),
    muted: _hsl(33, 30, 92),
    mutedForeground: _hsl(24, 12, 43),
    primary: _hsl(0, 17, 36),
    primaryForeground: _hsl(30, 28, 95),
    secondary: _hsl(34, 28, 90),
    secondaryForeground: _hsl(19, 24, 18),
    accent: _hsl(30, 32, 89),
    accentForeground: _hsl(19, 24, 18),
    destructive: _hsl(6, 66, 44),
    destructiveForeground: _hsl(0, 0, 100),
    success: _hsl(160, 60, 29),
    successForeground: _hsl(0, 0, 100),
    warning: _hsl(30, 80, 32),
    warningForeground: _hsl(0, 0, 100),
    info: _hsl(214, 74, 44),
    infoForeground: _hsl(0, 0, 100),
    contrast: _hsl(34, 15, 10),
    contrastForeground: _hsl(0, 0, 100),
    border: _hsl(34, 26, 86),
    input: _hsl(34, 22, 85),
    ring: _hsl(0, 32, 46),
  );

  /// Dark theme — mirrors .dark in tokens.css (warm near-black)
  static final SomaColors dark = SomaColors(
    background: _hsl(22, 24, 7),
    foreground: _hsl(33, 30, 93),
    card: _hsl(24, 18, 13),
    cardForeground: _hsl(33, 30, 93),
    muted: _hsl(24, 16, 22),
    mutedForeground: _hsl(28, 12, 63),
    primary: _hsl(9, 33, 43),
    primaryForeground: _hsl(30, 25, 95),
    secondary: _hsl(24, 14, 19),
    secondaryForeground: _hsl(33, 30, 93),
    accent: _hsl(24, 16, 22),
    accentForeground: _hsl(33, 30, 93),
    destructive: _hsl(6, 75, 68),
    destructiveForeground: _hsl(22, 24, 10),
    success: _hsl(160, 52, 58),
    successForeground: _hsl(22, 24, 10),
    warning: _hsl(38, 85, 62),
    warningForeground: _hsl(22, 24, 10),
    info: _hsl(214, 80, 70),
    infoForeground: _hsl(22, 24, 10),
    contrast: _hsl(33, 30, 93),
    contrastForeground: _hsl(22, 24, 10),
    border: _hsl(26, 15, 26),
    input: _hsl(26, 16, 31),
    ring: _hsl(9, 52, 60),
  );
}
