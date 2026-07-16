import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class FaqBlockScreen extends StatelessWidget {
  const FaqBlockScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'FAQ Block',
      subtitle: 'Centered FAQ section powered by SomaAccordion.',
      preview: const SizedBox(
        width: double.infinity,
        child: SomaFaqBlock(
          items: [
            SomaFaqItem(
              question: 'What is Soma UI?',
              answer:
                  'Soma UI is a cross-platform design system with Palantir-inspired slate/blue tokens, supporting web (Leptos) and mobile (Flutter) from a single source of truth.',
            ),
            SomaFaqItem(
              question: 'Is it free to use?',
              answer:
                  'Yes. Soma UI is open source. You own the source — copy the components directly into your project, no black-box packages.',
            ),
            SomaFaqItem(
              question: 'Does it support dark mode?',
              answer:
                  'Yes. SomaTheme provides both light and dark token sets. Wrap your app with SomaThemeProvider and toggle with SomaThemeToggle.',
            ),
            SomaFaqItem(
              question: 'Which Flutter version is required?',
              answer:
                  'Soma UI targets the latest stable Flutter release. It uses standard widgets and no platform channels, so it runs on any Flutter-supported platform.',
            ),
            SomaFaqItem(
              question: 'How do I add a new component?',
              answer:
                  'Place the widget file in the matching category folder under lib/src/components/<category>/, export it from the category barrel, and re-export from components.dart.',
            ),
          ],
        ),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
