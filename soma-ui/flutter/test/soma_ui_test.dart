import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:soma_ui/soma_ui.dart';

double _contrastRatio(Color a, Color b) {
  final aLuminance = a.computeLuminance();
  final bLuminance = b.computeLuminance();
  final lighter = aLuminance > bLuminance ? aLuminance : bLuminance;
  final darker = aLuminance > bLuminance ? bLuminance : aLuminance;
  return (lighter + 0.05) / (darker + 0.05);
}

void main() {
  test('SomaColors light and dark are distinct', () {
    expect(
        SomaColors.light.background, isNot(equals(SomaColors.dark.background)));
  });

  test('SomaColors dark primary matches the warm mahogany palette', () {
    final p = SomaColors.dark.primary;
    expect(p.r, greaterThan(p.g));
    expect(p.g, greaterThan(p.b));
  });

  test('semantic feedback colors meet AA contrast in both themes', () {
    for (final c in [SomaColors.light, SomaColors.dark]) {
      final roles = {
        'destructive': (c.destructive, c.destructiveForeground),
        'success': (c.success, c.successForeground),
        'warning': (c.warning, c.warningForeground),
        'info': (c.info, c.infoForeground),
      };

      for (final entry in roles.entries) {
        final (color, foreground) = entry.value;
        final tint = Color.alphaBlend(color.withAlpha(18), c.background);
        expect(
          _contrastRatio(color, tint),
          greaterThanOrEqualTo(4.5),
          reason: '${entry.key} must be readable on its tinted surface',
        );
        expect(
          _contrastRatio(foreground, color),
          greaterThanOrEqualTo(4.5),
          reason: '${entry.key} foreground must be readable on its fill',
        );
      }
    }
  });
}
