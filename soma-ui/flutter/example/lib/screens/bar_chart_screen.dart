import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class BarChartScreen extends StatefulWidget {
  const BarChartScreen({super.key});

  @override
  State<BarChartScreen> createState() => _BarChartScreenState();
}

class _BarChartScreenState extends State<BarChartScreen> {
  SomaBarChartVariant _variant = SomaBarChartVariant.default_;

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
    final isMultiSeries = _variant == SomaBarChartVariant.stacked || _variant == SomaBarChartVariant.grouped;
    return ComponentPage(
      title: 'Bar Chart',
      subtitle: 'Bar chart with vertical, horizontal, stacked, and grouped variants.',
      preview: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 400),
        child: SomaBarChart(
          data: isMultiSeries ? const [] : _data,
          series: isMultiSeries ? const [_seriesA, _seriesB] : const [],
          variant: _variant,
        ),
      ),
      controls: ControlRow(
        label: 'Variant',
        child: SomaSelect<SomaBarChartVariant>(
          items: SomaBarChartVariant.values
              .map((v) => SomaSelectItem(value: v, label: v.name.replaceAll('_', '')))
              .toList(),
          value: _variant,
          onChanged: (v) => setState(() => _variant = v),
        ),
      ),
    );
  }
}
