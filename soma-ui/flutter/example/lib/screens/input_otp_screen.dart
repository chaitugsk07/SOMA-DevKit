import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class InputOtpScreen extends StatefulWidget {
  const InputOtpScreen({super.key});

  @override
  State<InputOtpScreen> createState() => _InputOtpScreenState();
}

class _InputOtpScreenState extends State<InputOtpScreen> {
  String _value = '';
  double _length = 6;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final length = _length.round();
    // Clamp value to current length.
    final clamped = _value.length > length ? _value.substring(0, length) : _value;

    return ComponentPage(
      title: 'InputOtp',
      subtitle: 'Numeric one-time-password input with auto-advance focus.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaInputOtp(
            value: clamped,
            length: length,
            onChanged: (v) => setState(() => _value = v),
          ),
          const SizedBox(height: 16),
          Text(
            clamped.isEmpty ? 'Enter digits above' : 'Value: $clamped',
            style: TextStyle(
                fontFamily: 'Outfit', fontSize: 13, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 80,
              child: Text('Length',
                  style: TextStyle(
                      color: c.mutedForeground,
                      fontFamily: 'Outfit',
                      fontSize: 13)),
            ),
            Expanded(
              child: SomaSlider(
                value: _length,
                min: 4,
                max: 8,
                onChanged: (v) => setState(() {
                  _length = v.roundToDouble();
                  // Reset value when length changes.
                  _value = '';
                }),
              ),
            ),
            SizedBox(
              width: 24,
              child: Text(
                '$length',
                style: TextStyle(
                    fontFamily: 'Outfit', fontSize: 13, color: c.foreground),
                textAlign: TextAlign.right,
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
