import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class SettingsScreenDemo extends StatelessWidget {
  const SettingsScreenDemo({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Settings Screen',
      subtitle: 'Profile, notifications, security, and API key management',
      preview: const SizedBox(height: 600, child: SomaSettingsScreen()),
      controls: const SizedBox.shrink(),
    );
  }
}
