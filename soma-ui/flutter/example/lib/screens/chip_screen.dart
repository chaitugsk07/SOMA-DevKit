import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ChipScreen extends StatefulWidget {
  const ChipScreen({super.key});

  @override
  State<ChipScreen> createState() => _ChipScreenState();
}

class _ChipScreenState extends State<ChipScreen> {
  bool _removable = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Chip',
      subtitle: 'Compact tag/label chip with variants and optional remove.',
      preview: Wrap(
        spacing: 8,
        runSpacing: 8,
        children: [
          SomaChip(
            variant: SomaChipVariant.primary,
            removable: _removable,
            onRemove: _removable ? () {} : null,
            child: const Text('Primary'),
          ),
          SomaChip(
            variant: SomaChipVariant.secondary,
            removable: _removable,
            onRemove: _removable ? () {} : null,
            child: const Text('Secondary'),
          ),
          SomaChip(
            variant: SomaChipVariant.outline,
            removable: _removable,
            onRemove: _removable ? () {} : null,
            child: const Text('Outline'),
          ),
        ],
      ),
      controls: Row(children: [
        SizedBox(
          width: 120,
          child: Text(
            'Removable',
            style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13),
          ),
        ),
        SomaSwitch(value: _removable, onChanged: (v) => setState(() => _removable = v)),
      ]),
    );
  }
}
