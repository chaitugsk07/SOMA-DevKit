import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class InputPhoneScreen extends StatefulWidget {
  const InputPhoneScreen({super.key});

  @override
  State<InputPhoneScreen> createState() => _InputPhoneScreenState();
}

class _InputPhoneScreenState extends State<InputPhoneScreen> {
  String _value = '';

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Input Phone',
      subtitle: 'Phone number input with a country-code selector.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 360,
            child: SomaInputPhone(
              value: _value,
              onChanged: (v) => setState(() => _value = v),
            ),
          ),
          if (_value.isNotEmpty) ...[
            const SizedBox(height: 12),
            Text(
              _value,
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
            label: 'Current value',
            child: Text(
              _value.isEmpty ? '—' : _value,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.foreground,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
