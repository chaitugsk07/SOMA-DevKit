import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AlertScreen extends StatefulWidget {
  const AlertScreen({super.key});

  @override
  State<AlertScreen> createState() => _AlertScreenState();
}

class _AlertScreenState extends State<AlertScreen> {
  SomaAlertVariant _variant = SomaAlertVariant.primary;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Alert',
      subtitle: 'Alert message with semantic variants.',
      preview: SomaAlert(
        variant: _variant,
        title: 'Alert title',
        child: const Text('This is the alert description.'),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Variant', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaSelect<SomaAlertVariant>(
                items: SomaAlertVariant.values.map((v) => SomaSelectItem(value: v, label: v.name)).toList(),
                value: _variant,
                onChanged: (v) => setState(() => _variant = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
