import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class NavigationMenuScreen extends StatelessWidget {
  const NavigationMenuScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'NavigationMenu',
      subtitle: 'Horizontal menu bar with anchored content panels per item.',
      preview: SomaNavigationMenu(
        items: [
          SomaNavigationMenuItem(
            label: 'Products',
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  'Products',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontSize: 15,
                    fontWeight: FontWeight.w600,
                    color: c.foreground,
                  ),
                ),
                const SizedBox(height: 8),
                _NavLink('Analytics Dashboard'),
                _NavLink('Data Explorer'),
                _NavLink('Report Builder'),
              ],
            ),
          ),
          SomaNavigationMenuItem(
            label: 'Resources',
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  'Resources',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontSize: 15,
                    fontWeight: FontWeight.w600,
                    color: c.foreground,
                  ),
                ),
                const SizedBox(height: 8),
                _NavLink('Documentation'),
                _NavLink('API Reference'),
                _NavLink('Changelog'),
              ],
            ),
          ),
          SomaNavigationMenuItem(
            label: 'Company',
            content: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  'Company',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontSize: 15,
                    fontWeight: FontWeight.w600,
                    color: c.foreground,
                  ),
                ),
                const SizedBox(height: 8),
                _NavLink('About'),
                _NavLink('Blog'),
                _NavLink('Careers'),
              ],
            ),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Interaction',
            child: Text(
              'Tap a trigger to open its panel; tap outside or tap again to close.',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _NavLink extends StatefulWidget {
  final String label;
  const _NavLink(this.label);

  @override
  State<_NavLink> createState() => _NavLinkState();
}

class _NavLinkState extends State<_NavLink> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: Text(
          widget.label,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 14,
            color: _hovered ? c.primary : c.foreground,
          ),
        ),
      ),
    );
  }
}
