import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class ItemScreen extends StatelessWidget {
  const ItemScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Item',
      subtitle: 'List row with leading icon, content, and trailing action.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaItem(
            leading: const Icon(Icons.folder_outlined),
            trailing: const Icon(Icons.chevron_right),
            child: const Text('Documents'),
          ),
          const SizedBox(height: 8),
          SomaItem(
            leading: const Icon(Icons.image_outlined),
            trailing: const Icon(Icons.chevron_right),
            child: const Text('Photos'),
          ),
          const SizedBox(height: 8),
          SomaItem(
            leading: const Icon(Icons.music_note_outlined),
            child: const Text('Music'),
          ),
        ],
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
