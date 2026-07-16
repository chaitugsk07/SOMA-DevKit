import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DataTableScreen extends StatefulWidget {
  const DataTableScreen({super.key});

  @override
  State<DataTableScreen> createState() => _DataTableScreenState();
}

class _DataTableScreenState extends State<DataTableScreen> {
  bool _selectable = false;
  bool _filterable = false;
  double _pageSize = 0;

  static const _columns = [
    SomaDataColumn(key: 'name', header: 'Name', sortable: true),
    SomaDataColumn(key: 'role', header: 'Role', sortable: true),
    SomaDataColumn(key: 'dept', header: 'Department'),
    SomaDataColumn(key: 'status', header: 'Status'),
  ];

  static const _rows = [
    {'name': 'Alice Johnson', 'role': 'Engineer', 'dept': 'Platform', 'status': 'Active'},
    {'name': 'Bob Smith', 'role': 'Designer', 'dept': 'Product', 'status': 'Active'},
    {'name': 'Carol White', 'role': 'Manager', 'dept': 'Engineering', 'status': 'Away'},
    {'name': 'Dave Lee', 'role': 'DevOps', 'dept': 'Infra', 'status': 'Active'},
    {'name': 'Eve Martinez', 'role': 'Analyst', 'dept': 'Data', 'status': 'Active'},
    {'name': 'Frank Brown', 'role': 'Engineer', 'dept': 'Backend', 'status': 'Offline'},
    {'name': 'Grace Kim', 'role': 'Engineer', 'dept': 'Frontend', 'status': 'Active'},
    {'name': 'Hank Wilson', 'role': 'QA', 'dept': 'Quality', 'status': 'Active'},
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'DataTable',
      subtitle: 'Feature-rich table with sort, filter, select, and pagination.',
      preview: SomaDataTable(
        columns: _columns,
        rows: _rows,
        selectable: _selectable,
        filterable: _filterable,
        pageSize: _pageSize.toInt(),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text('Selectable', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13)),
            ),
            SomaSwitch(value: _selectable, onChanged: (v) => setState(() => _selectable = v)),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(
              width: 120,
              child: Text('Filterable', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13)),
            ),
            SomaSwitch(value: _filterable, onChanged: (v) => setState(() => _filterable = v)),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(
              width: 120,
              child: Text('Page size: ${_pageSize.toInt() == 0 ? "off" : _pageSize.toInt()}',
                  style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13)),
            ),
            Expanded(
              child: SomaSlider(
                value: _pageSize,
                min: 0,
                max: 8,
                onChanged: (v) => setState(() => _pageSize = v.roundToDouble()),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
