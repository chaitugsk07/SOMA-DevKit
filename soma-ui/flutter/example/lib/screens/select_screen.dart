import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SelectScreen extends StatefulWidget {
  const SelectScreen({super.key});

  @override
  State<SelectScreen> createState() => _SelectScreenState();
}

class _SelectScreenState extends State<SelectScreen> {
  String? _value;
  bool _enabled = true;

  static const _items = [
    SomaSelectItem(value: 'apple', label: 'Apple'),
    SomaSelectItem(value: 'banana', label: 'Banana'),
    SomaSelectItem(value: 'cherry', label: 'Cherry'),
    SomaSelectItem(value: 'date', label: 'Date'),
    SomaSelectItem(value: 'elderberry', label: 'Elderberry'),
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Select',
      subtitle: 'Dropdown selector for single value from a list.',
      preview: SomaSelect<String>(
        items: _items,
        value: _value,
        placeholder: 'Select a fruit...',
        enabled: _enabled,
        onChanged: (v) => setState(() => _value = v),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Enabled', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            SomaSwitch(value: _enabled, onChanged: (v) => setState(() => _enabled = v)),
          ]),
          const SizedBox(height: 12),
          Text(
            'Selected: ${_value ?? 'none'}',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.foreground),
          ),
        ],
      ),
    );
  }
}
