import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class FooterBlockScreen extends StatelessWidget {
  const FooterBlockScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Footer Block',
      subtitle: 'Multi-column marketing footer with brand, links, and social icons.',
      preview: SizedBox(
        width: double.infinity,
        child: SomaFooterBlock(
          columns: const [
            SomaFooterLinkColumn(
              heading: 'Product',
              links: ['Features', 'Pricing', 'Changelog'],
            ),
            SomaFooterLinkColumn(
              heading: 'Docs',
              links: ['Getting Started', 'Components', 'API Reference'],
            ),
            SomaFooterLinkColumn(
              heading: 'Company',
              links: ['About', 'Blog', 'Contact'],
            ),
          ],
        ),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
