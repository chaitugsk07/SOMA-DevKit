import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class FormScreen extends StatefulWidget {
  const FormScreen({super.key});

  @override
  State<FormScreen> createState() => _FormScreenState();
}

class _FormScreenState extends State<FormScreen> {
  final _email = TextEditingController();
  final _password = TextEditingController();
  String? _submitted;

  @override
  void dispose() {
    _email.dispose();
    _password.dispose();
    super.dispose();
  }

  void _handleSubmit() {
    setState(() => _submitted = 'email=${_email.text}');
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Form',
      subtitle: 'Form container with consistent vertical spacing and sub-pieces.',
      preview: SizedBox(
        width: 320,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            SomaForm(
              children: [
                SomaFormItem(
                  label: 'Email',
                  child: SomaInput(
                    controller: _email,
                    placeholder: 'you@example.com',
                    keyboardType: TextInputType.emailAddress,
                  ),
                ),
                SomaFormItem(
                  label: 'Password',
                  description: 'At least 8 characters.',
                  child: SomaInput(
                    controller: _password,
                    placeholder: '••••••••',
                    obscureText: true,
                  ),
                ),
                SomaButton(
                  onPressed: _handleSubmit,
                  child: const Text('Sign in'),
                ),
              ],
            ),
            if (_submitted != null) ...[
              const SizedBox(height: 16),
              Text(
                'Submitted: $_submitted',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 13,
                  color: c.mutedForeground,
                ),
              ),
            ],
          ],
        ),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Reset',
            child: SomaButton(
              variant: SomaButtonVariant.outline,
              size: SomaButtonSize.sm,
              onPressed: () => setState(() {
                _email.clear();
                _password.clear();
                _submitted = null;
              }),
              child: const Text('Reset'),
            ),
          ),
        ],
      ),
    );
  }
}
