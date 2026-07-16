import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SidenavBlockScreen extends StatefulWidget {
  const SidenavBlockScreen({super.key});

  @override
  State<SidenavBlockScreen> createState() => _SidenavBlockScreenState();
}

class _SidenavBlockScreenState extends State<SidenavBlockScreen> {
  static const _items = [
    SomaSidenavItem(icon: LucideIcons.layoutDashboard, label: 'Dashboard', id: 'dashboard'),
    SomaSidenavItem(icon: LucideIcons.barChart2, label: 'Analytics', id: 'analytics'),
    SomaSidenavItem(icon: LucideIcons.fileText, label: 'Reports', id: 'reports'),
    SomaSidenavItem(icon: LucideIcons.users, label: 'Users', id: 'users'),
    SomaSidenavItem(icon: LucideIcons.folder, label: 'Projects', id: 'projects'),
    SomaSidenavItem(icon: LucideIcons.settings, label: 'Settings', id: 'settings'),
  ];

  String _activeId = 'dashboard';

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Sidenav Block',
      subtitle: 'Data-driven 256px sidebar with brand, nav items, and user footer.',
      preview: SizedBox(
        height: 600,
        child: SomaSidenavBlock(
          items: _items,
          activeId: _activeId,
          onItemTap: (id) => setState(() => _activeId = id),
        ),
      ),
      controls: ControlRow(
        label: 'Active Item',
        child: Text(_activeId),
      ),
    );
  }
}
