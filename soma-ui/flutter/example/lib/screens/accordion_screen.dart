import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AccordionScreen extends StatelessWidget {
  const AccordionScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Accordion',
      subtitle: 'Single-open accordion — expanding one item collapses the rest.',
      preview: SomaAccordion(
        items: const [
          SomaAccordionItem(
            value: 'q1',
            title: 'What is soma-ui?',
            content: Text(
              'soma-ui is a cross-platform design system built with Palantir-style '
              'slate/blue tokens, supporting web (Leptos) and mobile (Flutter) from a single source of truth.',
            ),
            openByDefault: true,
          ),
          SomaAccordionItem(
            value: 'q2',
            title: 'How do I install it?',
            content: Text(
              'Add soma_ui to your pubspec.yaml dependencies, run flutter pub get, '
              'and wrap your app in SomaTheme.',
            ),
          ),
          SomaAccordionItem(
            value: 'q3',
            title: 'Does it support dark mode?',
            content: Text(
              'Yes. SomaTheme provides both light and dark token sets. '
              'Toggle the theme with SomaTheme.of(context).brightness.',
            ),
          ),
        ],
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
