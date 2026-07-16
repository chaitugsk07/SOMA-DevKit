import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class PieChartScreen extends StatefulWidget {
  const PieChartScreen({super.key});

  @override
  State<PieChartScreen> createState() => _PieChartScreenState();
}

class _PieChartScreenState extends State<PieChartScreen> {
  SomaPieChartVariant _variant = SomaPieChartVariant.pie;

  static const _data = [
    SomaChartPoint(label: 'Q1', value: 35),
    SomaChartPoint(label: 'Q2', value: 25),
    SomaChartPoint(label: 'Q3', value: 20),
    SomaChartPoint(label: 'Q4', value: 20),
  ];

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Pie Chart',
      subtitle: 'Pie chart with pie, donut, labeled, and half variants.',
      preview: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 300),
        child: SomaPieChart(data: _data, variant: _variant),
      ),
      controls: ControlRow(
        label: 'Variant',
        child: SomaSelect<SomaPieChartVariant>(
          items: SomaPieChartVariant.values
              .map((v) => SomaSelectItem(value: v, label: v.name))
              .toList(),
          value: _variant,
          onChanged: (v) => setState(() => _variant = v),
        ),
      ),
    );
  }
}
