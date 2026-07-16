import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class RadioGroupScreen extends StatefulWidget {
  const RadioGroupScreen({super.key});

  @override
  State<RadioGroupScreen> createState() => _RadioGroupScreenState();
}

class _RadioGroupScreenState extends State<RadioGroupScreen> {
  String _selected = 'option1';

  static const _options = [
    SomaRadioOption(value: 'option1', label: 'Option 1'),
    SomaRadioOption(value: 'option2', label: 'Option 2'),
    SomaRadioOption(value: 'option3', label: 'Option 3'),
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'RadioGroup',
      subtitle: 'Radio button group for single selection.',
      preview: SomaRadioGroup<String>(
        options: _options,
        value: _selected,
        onChanged: (v) => setState(() => _selected = v),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Selected: $_selected',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.foreground),
          ),
        ],
      ),
    );
  }
}
