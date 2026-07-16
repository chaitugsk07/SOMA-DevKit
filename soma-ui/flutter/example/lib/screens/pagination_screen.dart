import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class PaginationScreen extends StatefulWidget {
  const PaginationScreen({super.key});

  @override
  State<PaginationScreen> createState() => _PaginationScreenState();
}

class _PaginationScreenState extends State<PaginationScreen> {
  int _page = 1;
  double _totalPages = 10;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final total = _totalPages.toInt().clamp(1, 20);
    return ComponentPage(
      title: 'Pagination',
      subtitle: 'Page navigation with ellipsis windowing.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaPagination(
            page: _page.clamp(1, total),
            totalPages: total,
            onChanged: (p) => setState(() => _page = p),
          ),
          const SizedBox(height: 12),
          Text(
            'Page $_page of $total',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text('Total pages: ${_totalPages.toInt()}',
                  style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13)),
            ),
            Expanded(
              child: SomaSlider(
                value: _totalPages,
                min: 1,
                max: 20,
                onChanged: (v) => setState(() {
                  _totalPages = v.roundToDouble();
                  _page = _page.clamp(1, _totalPages.toInt());
                }),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
