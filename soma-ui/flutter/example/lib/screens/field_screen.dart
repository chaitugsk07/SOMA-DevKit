import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class FieldScreen extends StatefulWidget {
  const FieldScreen({super.key});

  @override
  State<FieldScreen> createState() => _FieldScreenState();
}

class _FieldScreenState extends State<FieldScreen> {
  bool _showError = false;

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Field',
      subtitle: 'Form field wrapper with label, description, and error support.',
      preview: SizedBox(
        width: 320,
        child: SomaField(
          label: 'Email address',
          description: _showError ? null : 'We will never share your email.',
          error: _showError ? 'Please enter a valid email address.' : null,
          child: const SomaInput(placeholder: 'you@example.com'),
        ),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Show error',
            child: SomaSwitch(
              value: _showError,
              onChanged: (v) => setState(() => _showError = v),
            ),
          ),
        ],
      ),
    );
  }
}
