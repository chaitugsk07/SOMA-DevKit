import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

const _kFrameworks = [
  SomaComboboxOption(value: 'flutter', label: 'Flutter'),
  SomaComboboxOption(value: 'react', label: 'React'),
  SomaComboboxOption(value: 'vue', label: 'Vue'),
  SomaComboboxOption(value: 'svelte', label: 'Svelte'),
  SomaComboboxOption(value: 'angular', label: 'Angular'),
  SomaComboboxOption(value: 'solid', label: 'SolidJS'),
  SomaComboboxOption(value: 'leptos', label: 'Leptos'),
  SomaComboboxOption(value: 'ember', label: 'Ember'),
];

class ComboboxScreen extends StatefulWidget {
  const ComboboxScreen({super.key});

  @override
  State<ComboboxScreen> createState() => _ComboboxScreenState();
}

class _ComboboxScreenState extends State<ComboboxScreen> {
  String? _selected;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Combobox',
      subtitle: 'Searchable single-select dropdown.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 280,
            child: SomaCombobox(
              options: _kFrameworks,
              value: _selected,
              placeholder: 'Select a framework…',
              onChanged: (v) => setState(() => _selected = v),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            _selected == null ? 'Nothing selected' : 'Selected: $_selected',
            style: TextStyle(
                fontFamily: 'Outfit', fontSize: 13, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text('Value',
                  style: TextStyle(
                      color: c.mutedForeground,
                      fontFamily: 'Outfit',
                      fontSize: 13)),
            ),
            Expanded(
              child: SomaSelect<String?>(
                items: [
                  const SomaSelectItem(value: null, label: '(none)'),
                  ..._kFrameworks.map((o) =>
                      SomaSelectItem(value: o.value, label: o.label)),
                ],
                value: _selected,
                onChanged: (v) => setState(() => _selected = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
