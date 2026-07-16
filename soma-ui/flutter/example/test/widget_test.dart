import 'package:flutter_test/flutter_test.dart';
import 'package:soma_ui_example/main.dart';

void main() {
  testWidgets('Playground app smoke test', (WidgetTester tester) async {
    await tester.pumpWidget(const PlaygroundApp());
    expect(find.text('soma_ui playground'), findsOneWidget);
  });
}
