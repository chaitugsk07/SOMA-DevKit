import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class TabsScreen extends StatefulWidget {
  const TabsScreen({super.key});

  @override
  State<TabsScreen> createState() => _TabsScreenState();
}

class _TabsScreenState extends State<TabsScreen> {
  int _index = 0;

  static const _tabs = [
    SomaTab(label: 'Account', content: Text('Account settings...')),
    SomaTab(label: 'Password', content: Text('Change password...')),
    SomaTab(label: 'Notifications', content: Text('Notification prefs...')),
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Tabs',
      subtitle: 'Tab navigation for switching between content panels.',
      preview: SomaTabs(
        tabs: _tabs,
        index: _index,
        onChanged: (i) => setState(() => _index = i),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Active tab: ${_tabs[_index].label}',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.foreground),
          ),
        ],
      ),
    );
  }
}
