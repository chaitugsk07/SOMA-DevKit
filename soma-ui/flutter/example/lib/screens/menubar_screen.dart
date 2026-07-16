import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class MenubarScreen extends StatelessWidget {
  const MenubarScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Menubar',
      subtitle: 'Horizontal bar of menus, like an application menu.',
      preview: SomaMenubar(
        menus: [
          SomaMenu(
            label: 'File',
            items: [
              SomaMenuItem(label: 'New', icon: LucideIcons.filePlus, onSelected: () {}),
              SomaMenuItem(label: 'Open…', icon: LucideIcons.folderOpen, onSelected: () {}),
              SomaMenuItem.separator(),
              SomaMenuItem(label: 'Save', icon: LucideIcons.save, onSelected: () {}),
              SomaMenuItem(label: 'Save As…', onSelected: () {}),
              SomaMenuItem.separator(),
              SomaMenuItem(label: 'Quit', icon: LucideIcons.logOut, onSelected: () {}),
            ],
          ),
          SomaMenu(
            label: 'Edit',
            items: [
              SomaMenuItem(label: 'Undo', icon: LucideIcons.undo2, onSelected: () {}),
              SomaMenuItem(label: 'Redo', icon: LucideIcons.redo2, onSelected: () {}),
              SomaMenuItem.separator(),
              SomaMenuItem(label: 'Cut', onSelected: () {}),
              SomaMenuItem(label: 'Copy', icon: LucideIcons.copy, onSelected: () {}),
              SomaMenuItem(label: 'Paste', onSelected: () {}),
            ],
          ),
          SomaMenu(
            label: 'View',
            items: [
              SomaMenuItem(label: 'Zoom In', icon: LucideIcons.zoomIn, onSelected: () {}),
              SomaMenuItem(label: 'Zoom Out', icon: LucideIcons.zoomOut, onSelected: () {}),
              SomaMenuItem.separator(),
              SomaMenuItem(label: 'Full Screen', icon: LucideIcons.maximize, onSelected: () {}),
            ],
          ),
        ],
      ),
      controls: Text(
        'Click a menu label to open its dropdown. Hover between menus while one is open to switch.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
