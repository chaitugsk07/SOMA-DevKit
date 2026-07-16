import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class PopoverScreen extends StatelessWidget {
  const PopoverScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Popover',
      subtitle: 'Anchored panel toggled by clicking a trigger.',
      preview: SomaPopover(
        trigger: SomaButton(
          onPressed: () {},
          child: const Text('Open Popover'),
        ),
        content: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              'Popover Content',
              style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 16,
                fontWeight: FontWeight.w600,
                color: c.cardForeground,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              'This is an anchored popover. Click outside to dismiss.',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ],
        ),
      ),
      controls: Text(
        'Click the button to toggle the popover. Tap outside to dismiss.',
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
