import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ContextMenuScreen extends StatelessWidget {
  const ContextMenuScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Context Menu',
      subtitle: 'Menu triggered by right-click or long-press.',
      preview: SomaContextMenu(
        items: [
          SomaMenuItem(
            label: 'Open',
            icon: LucideIcons.folderOpen,
            onSelected: () {},
          ),
          SomaMenuItem(
            label: 'Copy path',
            icon: LucideIcons.copy,
            onSelected: () {},
          ),
          SomaMenuItem.separator(),
          SomaMenuItem(
            label: 'Rename',
            icon: LucideIcons.pencil,
            onSelected: () {},
          ),
          SomaMenuItem(
            label: 'Delete',
            icon: LucideIcons.trash2,
            onSelected: () {},
          ),
        ],
        child: Container(
          width: 240,
          height: 120,
          decoration: BoxDecoration(
            color: c.muted,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: c.border),
          ),
          child: Center(
            child: Text(
              'Right-click or long-press here',
              textAlign: TextAlign.center,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ),
        ),
      ),
      controls: Text(
        'Right-click (desktop) or long-press (mobile) the area above.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
