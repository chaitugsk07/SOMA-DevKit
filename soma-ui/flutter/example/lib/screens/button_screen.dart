import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ButtonScreen extends StatefulWidget {
  const ButtonScreen({super.key});

  @override
  State<ButtonScreen> createState() => _ButtonScreenState();
}

class _ButtonScreenState extends State<ButtonScreen> {
  SomaButtonVariant _variant = SomaButtonVariant.primary;
  SomaButtonSize _size = SomaButtonSize.md;
  bool _enabled = true;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Button',
      subtitle: 'Interactive button component with variants and sizes.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaButton(
            variant: _variant,
            size: _size,
            enabled: _enabled,
            onPressed: () {},
            child: const Text('Button'),
          ),
          const SizedBox(height: 16),
          Text(
            'All Variants',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 8),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: SomaButtonVariant.values
                .map((v) => SomaButton(
                      variant: v,
                      onPressed: () {},
                      child: Text(v.name),
                    ))
                .toList(),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Variant', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaSelect<SomaButtonVariant>(
                items: SomaButtonVariant.values.map((v) => SomaSelectItem(value: v, label: v.name)).toList(),
                value: _variant,
                onChanged: (v) => setState(() => _variant = v),
              ),
            ),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(width: 120, child: Text('Size', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaSelect<SomaButtonSize>(
                items: SomaButtonSize.values.map((v) => SomaSelectItem(value: v, label: v.name)).toList(),
                value: _size,
                onChanged: (v) => setState(() => _size = v),
              ),
            ),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(width: 120, child: Text('Enabled', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            SomaSwitch(value: _enabled, onChanged: (v) => setState(() => _enabled = v)),
          ]),
        ],
      ),
    );
  }
}
