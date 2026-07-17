import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:soma_ui/soma_ui.dart';

Widget _wrap(Widget child) => SomaThemeProvider(
      themeMode: ThemeMode.light,
      child: Builder(
        builder: (context) => MaterialApp(
          theme: SomaTheme.buildThemeData(context),
          home: Scaffold(body: child),
        ),
      ),
    );

void main() {
  testWidgets('console shell opens mobile navigation', (tester) async {
    await tester.binding.setSurfaceSize(const Size(420, 700));
    addTearDown(() => tester.binding.setSurfaceSize(null));

    await tester.pumpWidget(
      _wrap(
        SomaConsoleShell(
          brand: 'SOMA Console',
          navigationBuilder: (_) => const Text('Agents'),
          header: const Text('Production'),
          body: const Text('Overview'),
        ),
      ),
    );

    expect(find.text('Agents'), findsNothing);
    await tester.tap(find.byTooltip('Open navigation'));
    await tester.pump();
    expect(find.text('Agents'), findsOneWidget);
    expect(find.text('SOMA Console'), findsOneWidget);
  });

  testWidgets('data table exposes loading and empty states', (tester) async {
    const columns = [
      SomaDataColumn(key: 'name', header: 'Name'),
    ];

    await tester.pumpWidget(
      _wrap(
        const SomaDataTable(
          columns: columns,
          rows: [],
          loading: true,
        ),
      ),
    );
    expect(find.text('Loading resources…'), findsOneWidget);

    await tester.pumpWidget(
      _wrap(
        const SomaDataTable(
          columns: columns,
          rows: [],
          emptyMessage: 'No agents configured.',
        ),
      ),
    );
    expect(find.text('No agents configured.'), findsOneWidget);
  });

  testWidgets('command palette supports arrow and Enter selection',
      (tester) async {
    String? selected;
    await tester.pumpWidget(
      _wrap(
        Builder(
          builder: (context) => TextButton(
            onPressed: () => showSomaCommand(
              context,
              groups: [
                SomaCommandGroup(
                  heading: 'Actions',
                  items: [
                    SomaCommandItem(
                      label: 'First action',
                      onSelect: () => selected = 'first',
                    ),
                    SomaCommandItem(
                      label: 'Second action',
                      onSelect: () => selected = 'second',
                    ),
                  ],
                ),
              ],
            ),
            child: const Text('Open'),
          ),
        ),
      ),
    );

    await tester.tap(find.text('Open'));
    await tester.pumpAndSettle();
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowDown);
    await tester.sendKeyEvent(LogicalKeyboardKey.enter);
    await tester.pumpAndSettle();

    expect(selected, 'second');
    expect(find.text('Second action'), findsNothing);
  });
}
