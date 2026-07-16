import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class LineChartScreen extends StatefulWidget {
  const LineChartScreen({super.key});

  @override
  State<LineChartScreen> createState() => _LineChartScreenState();
}

class _LineChartScreenState extends State<LineChartScreen> {
  SomaLineChartVariant _variant = SomaLineChartVariant.default_;

  static const _data = [
    SomaChartPoint(label: 'Jan', value: 40),
    SomaChartPoint(label: 'Feb', value: 65),
    SomaChartPoint(label: 'Mar', value: 55),
    SomaChartPoint(label: 'Apr', value: 80),
    SomaChartPoint(label: 'May', value: 70),
    SomaChartPoint(label: 'Jun', value: 90),
  ];

  static const _seriesA = SomaChartSeries(name: 'Series A', points: _data);
  static const _seriesB = SomaChartSeries(
    name: 'Series B',
    points: [
      SomaChartPoint(label: 'Jan', value: 20),
      SomaChartPoint(label: 'Feb', value: 35),
      SomaChartPoint(label: 'Mar', value: 45),
      SomaChartPoint(label: 'Apr', value: 30),
      SomaChartPoint(label: 'May', value: 50),
      SomaChartPoint(label: 'Jun', value: 40),
    ],
  );

  @override
  Widget build(BuildContext context) {
    final isMultiple = _variant == SomaLineChartVariant.multiple;
    return ComponentPage(
      title: 'Line Chart',
      subtitle: 'Line chart with smooth, linear, step, dots, and multiple series variants.',
      preview: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 400),
        child: SomaLineChart(
          data: isMultiple ? const [] : _data,
          series: isMultiple ? const [_seriesA, _seriesB] : const [],
          variant: _variant,
        ),
      ),
      controls: ControlRow(
        label: 'Variant',
        child: SomaSelect<SomaLineChartVariant>(
          items: SomaLineChartVariant.values
              .map((v) => SomaSelectItem(value: v, label: v.name.replaceAll('_', '')))
              .toList(),
          value: _variant,
          onChanged: (v) => setState(() => _variant = v),
        ),
      ),
    );
  }
}
