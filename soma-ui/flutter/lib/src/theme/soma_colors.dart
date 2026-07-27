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

  /// Light theme — mirrors :root in tokens.css (neutral console)
  static final SomaColors light = SomaColors(
    background: _hsl(0, 0, 100),
    foreground: _hsl(240, 10, 4),
    card: _hsl(0, 0, 100),
    cardForeground: _hsl(240, 10, 4),
    muted: _hsl(240, 5, 96),
    mutedForeground: _hsl(240, 4, 46),
    primary: _hsl(240, 6, 10),
    primaryForeground: _hsl(0, 0, 98),
    secondary: _hsl(240, 5, 96),
    secondaryForeground: _hsl(240, 6, 10),
    accent: _hsl(240, 5, 96),
    accentForeground: _hsl(240, 6, 10),
    destructive: _hsl(6, 66, 44),
    destructiveForeground: _hsl(0, 0, 100),
    success: _hsl(160, 60, 29),
    successForeground: _hsl(0, 0, 100),
    warning: _hsl(30, 80, 32),
    warningForeground: _hsl(0, 0, 100),
    info: _hsl(214, 74, 44),
    infoForeground: _hsl(0, 0, 100),
    contrast: _hsl(240, 6, 10),
    contrastForeground: _hsl(0, 0, 100),
    border: _hsl(240, 6, 90),
    input: _hsl(240, 6, 90),
    ring: _hsl(240, 5, 65),
  );

  /// Dark theme — mirrors .dark in tokens.css (neutral near-black)
  static final SomaColors dark = SomaColors(
    background: _hsl(240, 10, 4),
    foreground: _hsl(0, 0, 98),
    card: _hsl(240, 6, 7),
    cardForeground: _hsl(0, 0, 98),
    muted: _hsl(240, 4, 16),
    mutedForeground: _hsl(240, 5, 65),
    primary: _hsl(0, 0, 98),
    primaryForeground: _hsl(240, 6, 10),
    secondary: _hsl(240, 4, 16),
    secondaryForeground: _hsl(0, 0, 98),
    accent: _hsl(240, 4, 16),
    accentForeground: _hsl(0, 0, 98),
    destructive: _hsl(6, 75, 68),
    destructiveForeground: _hsl(240, 6, 10),
    success: _hsl(160, 52, 58),
    successForeground: _hsl(240, 6, 10),
    warning: _hsl(38, 85, 62),
    warningForeground: _hsl(240, 6, 10),
    info: _hsl(214, 80, 70),
    infoForeground: _hsl(240, 6, 10),
    contrast: _hsl(0, 0, 98),
    contrastForeground: _hsl(240, 6, 10),
    border: _hsl(240, 4, 16),
    input: _hsl(240, 4, 16),
    ring: _hsl(240, 5, 45),
  );
}
