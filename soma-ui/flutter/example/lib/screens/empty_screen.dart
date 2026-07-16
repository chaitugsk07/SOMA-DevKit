import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class EmptyScreen extends StatelessWidget {
  const EmptyScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Empty',
      subtitle: 'Empty state placeholder with icon, title, description, and action.',
      preview: SomaEmpty(
        icon: const Icon(Icons.inbox_outlined),
        title: 'No results found',
        description: 'Try adjusting your search or filters to find what you\'re looking for.',
        child: SomaButton(
          onPressed: () {},
          child: const Text('Clear filters'),
        ),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
