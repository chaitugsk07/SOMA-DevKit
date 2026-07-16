import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class CalloutScreen extends StatefulWidget {
  const CalloutScreen({super.key});

  @override
  State<CalloutScreen> createState() => _CalloutScreenState();
}

class _CalloutScreenState extends State<CalloutScreen> {
  SomaCalloutVariant _variant = SomaCalloutVariant.info;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Callout',
      subtitle: 'Inline callout block with a leading icon and colored left border.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaCallout(
            variant: _variant,
            title: '${_variant.name[0].toUpperCase()}${_variant.name.substring(1)} callout',
            child: const Text('This is the callout body text with supporting detail.'),
          ),
          const SizedBox(height: 24),
          Text(
            'All Variants',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 12),
          Column(
            children: SomaCalloutVariant.values
                .map(
                  (v) => Padding(
                    padding: const EdgeInsets.only(bottom: 8),
                    child: SomaCallout(
                      variant: v,
                      title: v.name,
                      child: const Text('Callout description text.'),
                    ),
                  ),
                )
                .toList(),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Variant',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            Expanded(
              child: SomaSelect<SomaCalloutVariant>(
                items: SomaCalloutVariant.values
                    .map((v) => SomaSelectItem(value: v, label: v.name))
                    .toList(),
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
