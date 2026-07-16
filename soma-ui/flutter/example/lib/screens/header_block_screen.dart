import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class HeaderBlockScreen extends StatelessWidget {
  const HeaderBlockScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Header Block',
      subtitle: 'App header bar with brand, nav links, search, notifications, and avatar.',
      preview: const SizedBox(
        width: double.infinity,
        child: SomaHeaderBlock(
          navLinks: ['Products', 'Docs', 'Blog', 'Pricing'],
        ),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
