import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ScrollAreaScreen extends StatefulWidget {
  const ScrollAreaScreen({super.key});

  @override
  State<ScrollAreaScreen> createState() => _ScrollAreaScreenState();
}

class _ScrollAreaScreenState extends State<ScrollAreaScreen> {
  double _height = 160;
  Axis _direction = Axis.vertical;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'ScrollArea',
      subtitle: 'Styled scrollable container with a themed thin scrollbar.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 300,
            decoration: BoxDecoration(
              border: Border.all(color: c.border),
              borderRadius: BorderRadius.circular(6),
            ),
            child: SomaScrollArea(
              height: _height,
              scrollDirection: _direction,
              child: _direction == Axis.vertical
                  ? _verticalContent(c)
                  : _horizontalContent(c),
            ),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Direction',
            child: SomaSelect<Axis>(
              items: const [
                SomaSelectItem(value: Axis.vertical, label: 'vertical'),
                SomaSelectItem(value: Axis.horizontal, label: 'horizontal'),
              ],
              value: _direction,
              onChanged: (v) => setState(() => _direction = v),
            ),
          ),
          ControlRow(
            label: 'Height',
            child: SomaSelect<double>(
              items: const [
                SomaSelectItem(value: 120, label: '120'),
                SomaSelectItem(value: 160, label: '160'),
                SomaSelectItem(value: 240, label: '240'),
              ],
              value: _height,
              onChanged: (v) => setState(() => _height = v),
            ),
          ),
        ],
      ),
    );
  }

  Widget _verticalContent(SomaColors c) {
    return Padding(
      padding: const EdgeInsets.all(12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: List.generate(
          16,
          (i) => Padding(
            padding: const EdgeInsets.symmetric(vertical: 4),
            child: Text(
              'Item ${i + 1} — scrollable content line',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.foreground,
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _horizontalContent(SomaColors c) {
    return Padding(
      padding: const EdgeInsets.all(12),
      child: Row(
        children: List.generate(
          12,
          (i) => Container(
            margin: const EdgeInsets.only(right: 8),
            width: 80,
            height: 80,
            decoration: BoxDecoration(
              color: c.muted,
              borderRadius: BorderRadius.circular(6),
            ),
            child: Center(
              child: Text(
                '${i + 1}',
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 18,
                  fontWeight: FontWeight.w600,
                  color: c.mutedForeground,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
