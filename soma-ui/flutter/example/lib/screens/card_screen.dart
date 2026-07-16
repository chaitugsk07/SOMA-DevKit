import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class CardScreen extends StatelessWidget {
  const CardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Card',
      subtitle: 'Container with header, content, and footer sections.',
      preview: SomaCard(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SomaCardHeader(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: const [
                  SomaCardTitle(child: Text('Create project')),
                  SizedBox(height: 6),
                  SomaCardDescription(child: Text('Deploy your new project in one click.')),
                ],
              ),
            ),
            const SomaCardContent(
              child: Text('Configure your project settings before deploying.'),
            ),
            SomaCardFooter(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  SomaButton(
                    variant: SomaButtonVariant.outline,
                    size: SomaButtonSize.sm,
                    onPressed: () {},
                    child: const Text('Cancel'),
                  ),
                  const SizedBox(width: 8),
                  SomaButton(
                    size: SomaButtonSize.sm,
                    onPressed: () {},
                    child: const Text('Deploy'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
      controls: Text(
        'SomaCard with header, content, and footer sections.',
        style: TextStyle(
          color: c.mutedForeground,
          fontFamily: 'Outfit',
          fontSize: 14,
        ),
      ),
    );
  }
}
