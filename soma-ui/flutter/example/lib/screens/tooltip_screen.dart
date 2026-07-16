import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class TooltipScreen extends StatelessWidget {
  const TooltipScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Tooltip',
      subtitle: 'Informational overlay shown on hover.',
      preview: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaTooltip(
            message: 'Add to your list',
            child: SomaButton(
              variant: SomaButtonVariant.outline,
              onPressed: () {},
              child: const Text('Hover me'),
            ),
          ),
          const SizedBox(width: 16),
          SomaTooltip(
            message: 'Delete this item',
            child: SomaButton(
              variant: SomaButtonVariant.destructive,
              onPressed: () {},
              child: const Text('Delete'),
            ),
          ),
        ],
      ),
      controls: Text(
        'Hover over the buttons to see tooltips.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
