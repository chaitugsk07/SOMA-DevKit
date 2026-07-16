import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class BottomNavScreen extends StatefulWidget {
  const BottomNavScreen({super.key});

  @override
  State<BottomNavScreen> createState() => _BottomNavScreenState();
}

class _BottomNavScreenState extends State<BottomNavScreen> {
  int _index = 0;

  static const _items = [
    SomaBottomNavItem(icon: Icons.home, label: 'Home'),
    SomaBottomNavItem(icon: Icons.search, label: 'Search'),
    SomaBottomNavItem(icon: Icons.notifications, label: 'Notifications'),
    SomaBottomNavItem(icon: Icons.person, label: 'Profile'),
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'BottomNav',
      subtitle: 'Bottom navigation bar for primary app navigation.',
      preview: SomaBottomNav(
        items: _items,
        index: _index,
        onTap: (i) => setState(() => _index = i),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Active: ${_items[_index].label}',
            style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.foreground),
          ),
        ],
      ),
    );
  }
}
