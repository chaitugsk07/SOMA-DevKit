import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SliderScreen extends StatefulWidget {
  const SliderScreen({super.key});

  @override
  State<SliderScreen> createState() => _SliderScreenState();
}

class _SliderScreenState extends State<SliderScreen> {
  double _value = 50;
  bool _enabled = true;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Slider',
      subtitle: 'Range slider for numeric input.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaSlider(
            value: _value,
            min: 0,
            max: 100,
            enabled: _enabled,
            onChanged: (v) => setState(() => _value = v),
          ),
          const SizedBox(height: 8),
          Text(
            _value.toInt().toString(),
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Enabled', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            SomaSwitch(value: _enabled, onChanged: (v) => setState(() => _enabled = v)),
          ]),
        ],
      ),
    );
  }
}
