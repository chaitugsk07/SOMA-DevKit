import 'package:flutter_test/flutter_test.dart';
import 'package:soma_ui/soma_ui.dart';

void main() {
  test('SomaColors light and dark are distinct', () {
    expect(SomaColors.light.background, isNot(equals(SomaColors.dark.background)));
  });

  test('SomaColors dark primary matches expected hue range', () {
    // primary: hsl(214, 82%, 56%) — should be a blue-ish color
    final p = SomaColors.dark.primary;
    // Blue channel should dominate
    expect(p.b, greaterThan(p.r));
  });
}
