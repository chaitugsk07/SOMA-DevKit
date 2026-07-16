import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ThemeToggleScreen extends StatefulWidget {
  const ThemeToggleScreen({super.key});

  @override
  State<ThemeToggleScreen> createState() => _ThemeToggleScreenState();
}

class _ThemeToggleScreenState extends State<ThemeToggleScreen> {
  bool _dark = true;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'ThemeToggle',
      subtitle: 'Controlled light/dark toggle. The live app toggle lives in the sidebar header.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaThemeToggle(
            isDark: _dark,
            onChanged: (v) => setState(() => _dark = v),
          ),
          const SizedBox(height: 12),
          Text(
            'Current: ${_dark ? 'dark' : 'light'}',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'isDark',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            SomaSwitch(value: _dark, onChanged: (v) => setState(() => _dark = v)),
          ]),
        ],
      ),
    );
  }
}
