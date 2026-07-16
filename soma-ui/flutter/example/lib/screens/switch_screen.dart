import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SwitchScreen extends StatefulWidget {
  const SwitchScreen({super.key});

  @override
  State<SwitchScreen> createState() => _SwitchScreenState();
}

class _SwitchScreenState extends State<SwitchScreen> {
  bool _value = false;
  bool _enabled = true;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Switch',
      subtitle: 'Toggle switch for boolean values.',
      preview: SomaSwitch(
        value: _value,
        enabled: _enabled,
        onChanged: (v) => setState(() => _value = v),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Value', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            SomaSwitch(value: _value, onChanged: (v) => setState(() => _value = v)),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(width: 120, child: Text('Enabled', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            SomaSwitch(value: _enabled, onChanged: (v) => setState(() => _enabled = v)),
          ]),
        ],
      ),
    );
  }
}
