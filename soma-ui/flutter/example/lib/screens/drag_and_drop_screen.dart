import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DragAndDropScreen extends StatefulWidget {
  const DragAndDropScreen({super.key});

  @override
  State<DragAndDropScreen> createState() => _DragAndDropScreenState();
}

class _DragAndDropScreenState extends State<DragAndDropScreen> {
  List<String> _items = [
    'Design system tokens',
    'Component library',
    'Playground app',
    'Documentation',
    'CI pipeline',
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Drag and Drop',
      subtitle: 'Reorderable list using Flutter\'s native ReorderableListView.',
      preview: SizedBox(
        width: double.infinity,
        child: SomaDragAndDrop(
          children: _items
              .map((item) => Padding(
                    padding: const EdgeInsets.symmetric(vertical: 2),
                    child: Text(
                      item,
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: c.foreground,
                      ),
                    ),
                  ))
              .toList(),
          onReorder: (oldIndex, newIndex) {
            setState(() {
              if (newIndex > oldIndex) newIndex--;
              final item = _items.removeAt(oldIndex);
              _items.insert(newIndex, item);
            });
          },
        ),
      ),
      controls: ControlRow(
        label: 'Reset order',
        child: SomaButton(
          variant: SomaButtonVariant.outline,
          size: SomaButtonSize.sm,
          onPressed: () => setState(() => _items = [
                'Design system tokens',
                'Component library',
                'Playground app',
                'Documentation',
                'CI pipeline',
              ]),
          child: const Text('Reset'),
        ),
      ),
    );
  }
}
