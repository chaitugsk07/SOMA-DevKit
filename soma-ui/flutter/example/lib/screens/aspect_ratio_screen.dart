import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AspectRatioScreen extends StatefulWidget {
  const AspectRatioScreen({super.key});

  @override
  State<AspectRatioScreen> createState() => _AspectRatioScreenState();
}

class _AspectRatioScreenState extends State<AspectRatioScreen> {
  double _ratio = 16 / 9;

  static const _presets = [
    ('16:9', 16 / 9),
    ('4:3', 4 / 3),
    ('1:1', 1.0),
    ('9:16', 9 / 16),
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'AspectRatio',
      subtitle: 'Constrains a child widget to a fixed width-to-height ratio.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 280,
            child: SomaAspectRatio(
              ratio: _ratio,
              child: Container(
                decoration: BoxDecoration(
                  color: c.muted,
                  borderRadius: BorderRadius.circular(6),
                ),
                child: Center(
                  child: Text(
                    _ratio == 1.0 ? '1:1' : (_presets.where((p) => p.$2 == _ratio).map((p) => p.$1).firstOrNull ?? _ratio.toStringAsFixed(2)),
                    style: TextStyle(
                      fontFamily: 'Rajdhani',
                      fontSize: 22,
                      fontWeight: FontWeight.w600,
                      color: c.mutedForeground,
                    ),
                  ),
                ),
              ),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            'A 1:1 box alongside a 16:9 box',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 8),
          Row(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              SizedBox(
                width: 60,
                child: SomaAspectRatio(
                  ratio: 1,
                  child: Container(
                    decoration: BoxDecoration(
                      color: c.primary.withAlpha(60),
                      borderRadius: BorderRadius.circular(4),
                    ),
                    child: Center(
                      child: Text('1:1', style: TextStyle(fontFamily: 'Outfit', fontSize: 11, color: c.primary)),
                    ),
                  ),
                ),
              ),
              const SizedBox(width: 8),
              SizedBox(
                width: 160,
                child: SomaAspectRatio(
                  ratio: 16 / 9,
                  child: Container(
                    decoration: BoxDecoration(
                      color: c.primary.withAlpha(120),
                      borderRadius: BorderRadius.circular(4),
                    ),
                    child: Center(
                      child: Text('16:9', style: TextStyle(fontFamily: 'Outfit', fontSize: 11, color: c.primaryForeground)),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Ratio',
            child: SomaSelect<double>(
              items: _presets.map((p) => SomaSelectItem(value: p.$2, label: p.$1)).toList(),
              value: _ratio,
              onChanged: (v) => setState(() => _ratio = v),
            ),
          ),
        ],
      ),
    );
  }
}
