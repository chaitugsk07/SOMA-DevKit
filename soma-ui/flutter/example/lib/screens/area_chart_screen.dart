import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AreaChartScreen extends StatefulWidget {
  const AreaChartScreen({super.key});

  @override
  State<AreaChartScreen> createState() => _AreaChartScreenState();
}

class _AreaChartScreenState extends State<AreaChartScreen> {
  SomaAreaChartVariant _variant = SomaAreaChartVariant.default_;

  static const _data = [
    SomaChartPoint(label: 'Jan', value: 40),
    SomaChartPoint(label: 'Feb', value: 65),
    SomaChartPoint(label: 'Mar', value: 55),
    SomaChartPoint(label: 'Apr', value: 80),
    SomaChartPoint(label: 'May', value: 70),
    SomaChartPoint(label: 'Jun', value: 90),
  ];

  static const _seriesA = SomaChartSeries(
    name: 'Series A',
    points: _data,
  );
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
    final isStacked = _variant == SomaAreaChartVariant.stacked;
    return ComponentPage(
      title: 'Area Chart',
      subtitle: 'Area chart with smooth bezier, linear, step, gradient, and stacked variants.',
      preview: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 400),
        child: SomaAreaChart(
          data: isStacked ? const [] : _data,
          series: isStacked ? const [_seriesA, _seriesB] : const [],
          variant: _variant,
        ),
      ),
      controls: ControlRow(
        label: 'Variant',
        child: SomaSelect<SomaAreaChartVariant>(
          items: SomaAreaChartVariant.values
              .map((v) => SomaSelectItem(value: v, label: v.name.replaceAll('_', '')))
              .toList(),
          value: _variant,
          onChanged: (v) => setState(() => _variant = v),
        ),
      ),
    );
  }
}
