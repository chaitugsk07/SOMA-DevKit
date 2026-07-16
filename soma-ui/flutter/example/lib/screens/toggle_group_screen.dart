import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ToggleGroupScreen extends StatefulWidget {
  const ToggleGroupScreen({super.key});

  @override
  State<ToggleGroupScreen> createState() => _ToggleGroupScreenState();
}

class _ToggleGroupScreenState extends State<ToggleGroupScreen> {
  String _value = 'left';

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'ToggleGroup',
      subtitle: 'Segmented control for mutually exclusive selection.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaToggleGroup<String>(
            items: const [
              SomaToggleItem(value: 'left', child: Text('Left')),
              SomaToggleItem(value: 'center', child: Text('Center')),
              SomaToggleItem(value: 'right', child: Text('Right')),
            ],
            value: _value,
            onChanged: (v) => setState(() => _value = v),
          ),
          const SizedBox(height: 8),
          Text(
            'Selected: $_value',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Text(
        'Interact with the toggle group above.',
        style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground),
      ),
    );
  }
}
