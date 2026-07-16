import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class TextareaScreen extends StatefulWidget {
  const TextareaScreen({super.key});

  @override
  State<TextareaScreen> createState() => _TextareaScreenState();
}

class _TextareaScreenState extends State<TextareaScreen> {
  bool _enabled = true;
  double _rows = 3;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Textarea',
      subtitle: 'Multi-line text input.',
      preview: SomaTextarea(
        placeholder: 'Enter text here...',
        enabled: _enabled,
        minLines: _rows.toInt(),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Enabled', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            SomaSwitch(value: _enabled, onChanged: (v) => setState(() => _enabled = v)),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(width: 120, child: Text('Min rows (${_rows.toInt()})', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaSlider(
                value: _rows,
                min: 1,
                max: 10,
                onChanged: (v) => setState(() => _rows = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
