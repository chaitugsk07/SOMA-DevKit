import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DropdownMenuScreen extends StatelessWidget {
  const DropdownMenuScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Dropdown Menu',
      subtitle: 'A menu of actions anchored to a trigger.',
      preview: SomaDropdownMenu(
        trigger: SomaButton(
          variant: SomaButtonVariant.outline,
          onPressed: () {},
          child: const Text('Open Menu'),
        ),
        items: [
          SomaMenuItem.labelItem('Actions'),
          SomaMenuItem(
            label: 'Edit',
            icon: LucideIcons.pencil,
            onSelected: () {},
          ),
          SomaMenuItem(
            label: 'Duplicate',
            icon: LucideIcons.copy,
            onSelected: () {},
          ),
          SomaMenuItem.separator(),
          SomaMenuItem(
            label: 'Delete',
            icon: LucideIcons.trash2,
            onSelected: () {},
          ),
        ],
      ),
      controls: Text(
        'Click the button to open the dropdown. Select an item or click outside to close.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
