import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class TableScreen extends StatelessWidget {
  const TableScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Table',
      subtitle: 'Simple data table with header, rows, and optional caption.',
      preview: SomaTable(
        columns: const [Text('Name'), Text('Role'), Text('Status')],
        rows: const [
          [Text('Alice'), Text('Engineer'), Text('Active')],
          [Text('Bob'), Text('Designer'), Text('Active')],
          [Text('Carol'), Text('Manager'), Text('Away')],
          [Text('Dave'), Text('DevOps'), Text('Offline')],
        ],
        caption: 'Team members',
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
