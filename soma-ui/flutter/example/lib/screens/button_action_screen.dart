import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ButtonActionScreen extends StatefulWidget {
  const ButtonActionScreen({super.key});

  @override
  State<ButtonActionScreen> createState() => _ButtonActionScreenState();
}

class _ButtonActionScreenState extends State<ButtonActionScreen> {
  double _duration = 1000;
  String _status = 'Hold the button to trigger the action.';

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'ButtonAction',
      subtitle: 'Press-and-hold button that fires after a set duration.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaButtonAction(
            durationMs: _duration.round(),
            onAction: () {
              setState(() => _status = 'Action fired!');
              Future.delayed(const Duration(seconds: 2),
                  () => setState(() => _status = 'Hold the button to trigger the action.'));
            },
            child: const Text('Hold me'),
          ),
          const SizedBox(height: 16),
          Text(
            _status,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text('Duration (ms)',
                  style: TextStyle(
                      color: c.mutedForeground,
                      fontFamily: 'Outfit',
                      fontSize: 13)),
            ),
            Expanded(
              child: SomaSlider(
                value: _duration,
                min: 500,
                max: 3000,
                onChanged: (v) => setState(() => _duration = v),
              ),
            ),
            SizedBox(
              width: 48,
              child: Text(
                '${_duration.round()}',
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
