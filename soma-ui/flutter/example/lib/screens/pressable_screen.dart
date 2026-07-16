import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class PressableScreen extends StatelessWidget {
  const PressableScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Pressable',
      subtitle: 'Press-down scale animation wrapper for any widget.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaPressable(
            onTap: () {},
            child: SomaCard(
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 20),
              child: Text(
                'Press me',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 15,
                  fontWeight: FontWeight.w500,
                  color: c.foreground,
                ),
              ),
            ),
          ),
          const SizedBox(height: 16),
          SomaPressable(
            onTap: () {},
            child: SomaButton(
              onPressed: null,
              child: const Text('Also pressable'),
            ),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Wrap any widget with SomaPressable to add a 0.95 scale press animation.',
            style: TextStyle(
              color: c.mutedForeground,
              fontFamily: 'Outfit',
              fontSize: 13,
            ),
          ),
        ],
      ),
    );
  }
}
