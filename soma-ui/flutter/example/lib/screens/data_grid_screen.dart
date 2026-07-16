import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

// Generate 50 sample rows to demonstrate virtualization.
List<Map<String, String>> _generateRows() {
  const statuses = ['Active', 'Inactive', 'Pending', 'Archived'];
  const regions = ['APAC', 'EMEA', 'NA', 'LATAM'];
  return List.generate(50, (i) {
    final n = i + 1;
    return {
      'id': '$n',
      'name': 'Record $n',
      'status': statuses[i % statuses.length],
      'region': regions[i % regions.length],
      'score': '${(n * 17) % 100}',
    };
  });
}

final _columns = [
  const SomaDataColumn(key: 'id', header: 'ID', sortable: true),
  const SomaDataColumn(key: 'name', header: 'Name', sortable: true),
  const SomaDataColumn(key: 'status', header: 'Status', sortable: true),
  const SomaDataColumn(key: 'region', header: 'Region', sortable: true),
  const SomaDataColumn(key: 'score', header: 'Score', sortable: true),
];

final _rows = _generateRows();

class DataGridScreen extends StatefulWidget {
  const DataGridScreen({super.key});

  @override
  State<DataGridScreen> createState() => _DataGridScreenState();
}

class _DataGridScreenState extends State<DataGridScreen> {
  double _height = 300;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'DataGrid',
      subtitle: 'Virtualized, sortable data grid with sticky header (50 rows).',
      preview: SomaDataGrid(
        columns: _columns,
        rows: _rows,
        height: _height,
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Height',
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  '${_height.round()} px',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 13,
                    color: c.mutedForeground,
                  ),
                ),
                const SizedBox(width: 12),
                SomaButton(
                  variant: SomaButtonVariant.outline,
                  size: SomaButtonSize.sm,
                  onPressed: _height > 150
                      ? () => setState(() => _height -= 50)
                      : null,
                  child: const Text('−'),
                ),
                const SizedBox(width: 4),
                SomaButton(
                  variant: SomaButtonVariant.outline,
                  size: SomaButtonSize.sm,
                  onPressed: _height < 600
                      ? () => setState(() => _height += 50)
                      : null,
                  child: const Text('+'),
                ),
              ],
            ),
          ),
          ControlRow(
            label: 'Rows',
            child: Text(
              '${_rows.length} rows — tap column header to sort',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
