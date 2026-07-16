import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

const _kToppings = [
  SomaMultiSelectOption(value: 'cheese', label: 'Cheese'),
  SomaMultiSelectOption(value: 'mushrooms', label: 'Mushrooms'),
  SomaMultiSelectOption(value: 'pepperoni', label: 'Pepperoni'),
  SomaMultiSelectOption(value: 'olives', label: 'Olives'),
  SomaMultiSelectOption(value: 'basil', label: 'Basil'),
  SomaMultiSelectOption(value: 'anchovies', label: 'Anchovies'),
];

class MultiSelectScreen extends StatefulWidget {
  const MultiSelectScreen({super.key});

  @override
  State<MultiSelectScreen> createState() => _MultiSelectScreenState();
}

class _MultiSelectScreenState extends State<MultiSelectScreen> {
  List<String> _selected = [];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'MultiSelect',
      subtitle: 'Multi-value dropdown with inline removable pills.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 320,
            child: SomaMultiSelect(
              options: _kToppings,
              selected: _selected,
              placeholder: 'Select toppings…',
              onChanged: (v) => setState(() => _selected = v),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            _selected.isEmpty
                ? 'Nothing selected'
                : 'Selected: ${_selected.join(', ')}',
            style: TextStyle(
                fontFamily: 'Outfit', fontSize: 13, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Toggle items in the dropdown — the list stays open.',
            style: TextStyle(
                fontFamily: 'Outfit', fontSize: 13, color: c.mutedForeground),
          ),
        ],
      ),
    );
  }
}
