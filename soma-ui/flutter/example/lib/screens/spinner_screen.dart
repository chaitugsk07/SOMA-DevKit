import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SpinnerScreen extends StatefulWidget {
  const SpinnerScreen({super.key});

  @override
  State<SpinnerScreen> createState() => _SpinnerScreenState();
}

class _SpinnerScreenState extends State<SpinnerScreen> {
  SomaSpinnerSize _size = SomaSpinnerSize.md;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Spinner',
      subtitle: 'Loading spinner with size variants.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaSpinner(size: _size),
          const SizedBox(height: 16),
          Row(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: SomaSpinnerSize.values.map((s) => Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  SomaSpinner(size: s),
                  const SizedBox(height: 4),
                  Text(s.name, style: TextStyle(fontFamily: 'Outfit', fontSize: 11, color: c.mutedForeground)),
                ],
              ),
            )).toList(),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Size', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaSelect<SomaSpinnerSize>(
                items: SomaSpinnerSize.values.map((v) => SomaSelectItem(value: v, label: v.name)).toList(),
                value: _size,
                onChanged: (v) => setState(() => _size = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
