// Guards against the "borderRadius can only be given on borders with uniform
// colors" paint crash in SomaAlert and SomaCallout (and SomaToast variant).
// This crash is invisible to `flutter analyze` / `flutter build` — it only
// surfaces at paint time, so a widget-pump test is the only reliable check.
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:soma_ui/soma_ui.dart';

Widget _wrap(Widget child) => MaterialApp(
      home: SomaThemeProvider(
        themeMode: ThemeMode.dark,
        child: Scaffold(
          body: SizedBox(width: 400, child: child),
        ),
      ),
    );

void main() {
  group('SomaAlert — no paint crash for all variants', () {
    for (final variant in SomaAlertVariant.values) {
      testWidgets('variant: $variant', (tester) async {
        await tester.pumpWidget(_wrap(
          SomaAlert(
            variant: variant,
            title: 'Title',
            child: const Text('Body text'),
          ),
        ));
        expect(tester.takeException(), isNull);
      });
    }
  });

  group('SomaCallout — no paint crash for all variants', () {
    for (final variant in SomaCalloutVariant.values) {
      testWidgets('variant: $variant', (tester) async {
        await tester.pumpWidget(_wrap(
          SomaCallout(
            variant: variant,
            title: 'Title',
            child: const Text('Body text'),
          ),
        ));
        expect(tester.takeException(), isNull);
      });
    }
  });
}
