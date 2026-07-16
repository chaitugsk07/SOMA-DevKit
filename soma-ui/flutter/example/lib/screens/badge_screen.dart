import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class BadgeScreen extends StatefulWidget {
  const BadgeScreen({super.key});

  @override
  State<BadgeScreen> createState() => _BadgeScreenState();
}

class _BadgeScreenState extends State<BadgeScreen> {
  SomaBadgeVariant _variant = SomaBadgeVariant.primary;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Badge',
      subtitle: 'Small status indicator with semantic variants.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaBadge(variant: _variant, child: Text(_variant.name)),
          const SizedBox(height: 16),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: SomaBadgeVariant.values
                .map((v) => SomaBadge(variant: v, child: Text(v.name)))
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
              child: SomaSelect<SomaBadgeVariant>(
                items: SomaBadgeVariant.values.map((v) => SomaSelectItem(value: v, label: v.name)).toList(),
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
