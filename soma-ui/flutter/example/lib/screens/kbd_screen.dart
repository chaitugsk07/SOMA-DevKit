import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class KbdScreen extends StatelessWidget {
  const KbdScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Kbd',
      subtitle: 'Keyboard key display for shortcuts and hotkeys.',
      preview: Wrap(
        spacing: 8,
        runSpacing: 8,
        alignment: WrapAlignment.center,
        children: const [
          SomaKbd(child: Text('⌘')),
          SomaKbd(child: Text('K')),
          SomaKbd(child: Text('Ctrl')),
          SomaKbd(child: Text('Shift')),
          SomaKbd(child: Text('⌘ K')),
          SomaKbd(child: Text('Enter')),
          SomaKbd(child: Text('Esc')),
        ],
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
