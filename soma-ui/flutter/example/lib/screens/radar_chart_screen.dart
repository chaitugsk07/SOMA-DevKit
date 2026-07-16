import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class RadarChartScreen extends StatefulWidget {
  const RadarChartScreen({super.key});

  @override
  State<RadarChartScreen> createState() => _RadarChartScreenState();
}

class _RadarChartScreenState extends State<RadarChartScreen> {
  SomaRadarChartVariant _variant = SomaRadarChartVariant.default_;

  static const _data = [
    SomaChartPoint(label: 'Speed', value: 80),
    SomaChartPoint(label: 'Power', value: 65),
    SomaChartPoint(label: 'Defense', value: 70),
    SomaChartPoint(label: 'Agility', value: 90),
    SomaChartPoint(label: 'Stamina', value: 75),
  ];

  static const _seriesA = SomaChartSeries(name: 'Hero A', points: _data);
  static const _seriesB = SomaChartSeries(
    name: 'Hero B',
    points: [
      SomaChartPoint(label: 'Speed', value: 60),
      SomaChartPoint(label: 'Power', value: 80),
      SomaChartPoint(label: 'Defense', value: 55),
      SomaChartPoint(label: 'Agility', value: 70),
      SomaChartPoint(label: 'Stamina', value: 85),
    ],
  );

  @override
  Widget build(BuildContext context) {
    final isMultiple = _variant == SomaRadarChartVariant.multiple;
    return ComponentPage(
      title: 'Radar Chart',
      subtitle: 'Radar chart with filled, lines, dots, and multiple series variants.',
      preview: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 320),
        child: SomaRadarChart(
          data: isMultiple ? const [] : _data,
          series: isMultiple ? const [_seriesA, _seriesB] : const [],
          variant: _variant,
        ),
      ),
      controls: ControlRow(
        label: 'Variant',
        child: SomaSelect<SomaRadarChartVariant>(
          items: SomaRadarChartVariant.values
              .map((v) => SomaSelectItem(value: v, label: v.name.replaceAll('_', '')))
              .toList(),
          value: _variant,
          onChanged: (v) => setState(() => _variant = v),
        ),
      ),
    );
  }
}
