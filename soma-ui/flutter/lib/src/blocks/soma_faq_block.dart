import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaFaqItem {
  final String question;
  final String answer;
  const SomaFaqItem({required this.question, required this.answer});
}

class SomaFaqBlock extends StatelessWidget {
  final String title;
  final String subtitle;
  final List<SomaFaqItem> items;

  const SomaFaqBlock({
    super.key,
    this.title = 'Frequently Asked Questions',
    this.subtitle = 'Everything you need to know about Soma UI.',
    this.items = const [],
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 640),
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 64),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Text(
                title,
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 30,
                  fontWeight: FontWeight.w700,
                  color: c.foreground,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                subtitle,
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 16,
                  color: c.mutedForeground,
                ),
              ),
              const SizedBox(height: 40),
              SomaAccordion(
                items: items
                    .map(
                      (item) => SomaAccordionItem(
                        value: item.question,
                        title: item.question,
                        content: Text(item.answer),
                      ),
                    )
                    .toList(),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
