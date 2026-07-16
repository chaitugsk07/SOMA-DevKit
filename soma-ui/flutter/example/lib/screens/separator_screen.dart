import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SeparatorScreen extends StatefulWidget {
  const SeparatorScreen({super.key});

  @override
  State<SeparatorScreen> createState() => _SeparatorScreenState();
}

class _SeparatorScreenState extends State<SeparatorScreen> {
  bool _vertical = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Separator',
      subtitle: 'Visual divider between content sections.',
      preview: _vertical
          ? SizedBox(height: 100, child: SomaSeparator(orientation: Axis.vertical))
          : const SizedBox(width: 200, child: SomaSeparator()),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Vertical', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            SomaSwitch(value: _vertical, onChanged: (v) => setState(() => _vertical = v)),
          ]),
        ],
      ),
    );
  }
}
