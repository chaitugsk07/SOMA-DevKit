import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class RadialChartScreen extends StatefulWidget {
  const RadialChartScreen({super.key});

  @override
  State<RadialChartScreen> createState() => _RadialChartScreenState();
}

class _RadialChartScreenState extends State<RadialChartScreen> {
  SomaRadialChartVariant _variant = SomaRadialChartVariant.default_;

  static const _data = [
    SomaChartPoint(label: 'Progress', value: 75),
    SomaChartPoint(label: 'Target', value: 50),
    SomaChartPoint(label: 'Bonus', value: 35),
  ];

  static const _seriesA = SomaChartSeries(name: 'Series A', points: _data);
  static const _seriesB = SomaChartSeries(
    name: 'Series B',
    points: [
      SomaChartPoint(label: 'Progress', value: 40),
      SomaChartPoint(label: 'Target', value: 60),
      SomaChartPoint(label: 'Bonus', value: 20),
    ],
  );

  @override
  Widget build(BuildContext context) {
    final isStacked = _variant == SomaRadialChartVariant.stacked;
    return ComponentPage(
      title: 'Radial Chart',
      subtitle: 'Radial chart with concentric arcs, stacked, labeled, and track variants.',
      preview: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 300),
        child: SomaRadialChart(
          data: isStacked ? const [] : _data,
          series: isStacked ? const [_seriesA, _seriesB] : const [],
          variant: _variant,
        ),
      ),
      controls: ControlRow(
        label: 'Variant',
        child: SomaSelect<SomaRadialChartVariant>(
          items: SomaRadialChartVariant.values
              .map((v) => SomaSelectItem(value: v, label: v.name.replaceAll('_', '')))
              .toList(),
          value: _variant,
          onChanged: (v) => setState(() => _variant = v),
        ),
      ),
    );
  }
}
