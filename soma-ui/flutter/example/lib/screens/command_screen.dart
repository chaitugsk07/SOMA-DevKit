import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class CommandScreen extends StatelessWidget {
  const CommandScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final groups = [
      SomaCommandGroup(
        heading: 'Navigation',
        items: [
          SomaCommandItem(
            label: 'Go to Dashboard',
            keywords: 'home dashboard',
            icon: LucideIcons.layoutDashboard,
            onSelect: () {},
          ),
          SomaCommandItem(
            label: 'Go to Settings',
            keywords: 'settings preferences config',
            icon: LucideIcons.settings,
            onSelect: () {},
          ),
          SomaCommandItem(
            label: 'View Profile',
            keywords: 'profile account user',
            icon: LucideIcons.user,
            onSelect: () {},
          ),
        ],
      ),
      SomaCommandGroup(
        heading: 'Actions',
        items: [
          SomaCommandItem(
            label: 'Create new file',
            keywords: 'new create file',
            icon: LucideIcons.filePlus,
            onSelect: () {},
          ),
          SomaCommandItem(
            label: 'Search',
            keywords: 'search find',
            icon: LucideIcons.search,
            onSelect: () {},
          ),
        ],
      ),
    ];

    return ComponentPage(
      title: 'Command',
      subtitle: 'Searchable command palette for quick navigation and actions.',
      preview: SomaButton(
        onPressed: () => showSomaCommand(context, groups: groups),
        child: const Text('Open Command Palette'),
      ),
      controls: Text(
        'Click the button to open the command palette. Type to filter. Press Escape to close.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
