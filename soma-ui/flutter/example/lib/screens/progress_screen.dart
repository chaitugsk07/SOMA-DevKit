import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ProgressScreen extends StatefulWidget {
  const ProgressScreen({super.key});

  @override
  State<ProgressScreen> createState() => _ProgressScreenState();
}

class _ProgressScreenState extends State<ProgressScreen> {
  double _value = 50;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Progress',
      subtitle: 'Linear progress indicator.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaProgress(value: _value / 100),
          const SizedBox(height: 8),
          Text(
            '${_value.toInt()}%',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Value', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaSlider(
                value: _value,
                min: 0,
                max: 100,
                onChanged: (v) => setState(() => _value = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
