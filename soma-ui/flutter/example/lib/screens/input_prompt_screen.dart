import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class InputPromptScreen extends StatefulWidget {
  const InputPromptScreen({super.key});

  @override
  State<InputPromptScreen> createState() => _InputPromptScreenState();
}

class _InputPromptScreenState extends State<InputPromptScreen> {
  String _current = '';
  String _lastSubmitted = '';

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Input Prompt',
      subtitle: 'AI-prompt-style composer: multiline input with a send button.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 400,
            child: SomaInputPrompt(
              value: _current,
              onChanged: (v) => setState(() => _current = v),
              onSubmit: () {
                setState(() {
                  _lastSubmitted = _current;
                  _current = '';
                });
              },
            ),
          ),
          if (_lastSubmitted.isNotEmpty) ...[
            const SizedBox(height: 12),
            Text(
              'Submitted: $_lastSubmitted',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ],
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Last submitted',
            child: Flexible(
              child: Text(
                _lastSubmitted.isEmpty ? '—' : _lastSubmitted,
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 13,
                  color: c.foreground,
                ),
                overflow: TextOverflow.ellipsis,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
