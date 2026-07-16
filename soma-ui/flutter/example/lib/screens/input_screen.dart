import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class InputScreen extends StatefulWidget {
  const InputScreen({super.key});

  @override
  State<InputScreen> createState() => _InputScreenState();
}

class _InputScreenState extends State<InputScreen> {
  bool _enabled = true;
  String _placeholder = 'Enter text...';
  late final TextEditingController _controller;
  late final TextEditingController _placeholderController;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
    _placeholderController = TextEditingController(text: _placeholder);
  }

  @override
  void dispose() {
    _controller.dispose();
    _placeholderController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Input',
      subtitle: 'Text input field with placeholder and enabled state.',
      preview: SomaInput(
        controller: _controller,
        placeholder: _placeholder,
        enabled: _enabled,
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
            SizedBox(width: 120, child: Text('Placeholder', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaInput(
                controller: _placeholderController,
                placeholder: 'Placeholder text',
                onChanged: (v) => setState(() => _placeholder = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
