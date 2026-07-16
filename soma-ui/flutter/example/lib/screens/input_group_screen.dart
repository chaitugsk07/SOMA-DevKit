import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class InputGroupScreen extends StatefulWidget {
  const InputGroupScreen({super.key});

  @override
  State<InputGroupScreen> createState() => _InputGroupScreenState();
}

class _InputGroupScreenState extends State<InputGroupScreen> {
  bool _showLeading = true;
  bool _showTrailing = true;

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Input Group',
      subtitle: 'An input with optional leading and trailing addons sharing one border.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            width: 360,
            child: SomaInputGroup(
              leading: _showLeading
                  ? const SomaInputGroupAddon(child: Icon(LucideIcons.search, size: 16))
                  : null,
              trailing: _showTrailing
                  ? SomaInputGroupAddon(
                      child: SomaButton(
                        size: SomaButtonSize.sm,
                        onPressed: () {},
                        child: const Text('Go'),
                      ),
                    )
                  : null,
              child: const SomaInput(placeholder: 'Search...'),
            ),
          ),
          const SizedBox(height: 16),
          SizedBox(
            width: 360,
            child: SomaInputGroup(
              leading: const SomaInputGroupAddon(child: Text('https://')),
              child: const SomaInput(placeholder: 'yoursite.com'),
            ),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Show leading',
            child: SomaSwitch(
              value: _showLeading,
              onChanged: (v) => setState(() => _showLeading = v),
            ),
          ),
          ControlRow(
            label: 'Show trailing',
            child: SomaSwitch(
              value: _showTrailing,
              onChanged: (v) => setState(() => _showTrailing = v),
            ),
          ),
        ],
      ),
    );
  }
}
