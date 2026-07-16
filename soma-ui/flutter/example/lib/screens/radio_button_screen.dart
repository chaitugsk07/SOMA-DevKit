import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class RadioButtonScreen extends StatefulWidget {
  const RadioButtonScreen({super.key});

  @override
  State<RadioButtonScreen> createState() => _RadioButtonScreenState();
}

class _RadioButtonScreenState extends State<RadioButtonScreen> {
  int _selected = 0;
  bool _enabled = true;

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Radio Button',
      subtitle: 'Standalone radio atom — circular control with optional label.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          for (int i = 0; i < 3; i++) ...[
            SomaRadioButton(
              selected: _selected == i,
              enabled: _enabled,
              label: Text('Option ${i + 1}'),
              onChanged: (_) => setState(() => _selected = i),
            ),
            if (i < 2) const SizedBox(height: 8),
          ],
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Enabled',
            child: SomaSwitch(
              value: _enabled,
              onChanged: (v) => setState(() => _enabled = v),
            ),
          ),
          ControlRow(
            label: 'Selected index',
            child: SomaSelect<int>(
              items: [0, 1, 2]
                  .map((i) => SomaSelectItem(value: i, label: 'Option ${i + 1}'))
                  .toList(),
              value: _selected,
              onChanged: (v) => setState(() => _selected = v),
            ),
          ),
        ],
      ),
    );
  }
}
