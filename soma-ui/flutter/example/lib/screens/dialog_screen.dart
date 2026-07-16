import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DialogScreen extends StatefulWidget {
  const DialogScreen({super.key});

  @override
  State<DialogScreen> createState() => _DialogScreenState();
}

class _DialogScreenState extends State<DialogScreen> {
  String _title = 'Are you sure?';
  String _message = 'This action cannot be undone.';
  late final TextEditingController _titleController;
  late final TextEditingController _messageController;

  @override
  void initState() {
    super.initState();
    _titleController = TextEditingController(text: _title);
    _messageController = TextEditingController(text: _message);
  }

  @override
  void dispose() {
    _titleController.dispose();
    _messageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Dialog',
      subtitle: 'Modal dialog for confirmations and forms.',
      preview: SomaButton(
        onPressed: () => showSomaDialog(
          context: context,
          title: _title,
          description: _message,
          actions: [
            SomaButton(
              variant: SomaButtonVariant.outline,
              onPressed: () => Navigator.pop(context),
              child: const Text('Cancel'),
            ),
            SomaButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Confirm'),
            ),
          ],
        ),
        child: const Text('Open Dialog'),
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(width: 120, child: Text('Title', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaInput(
                controller: _titleController,
                placeholder: 'Dialog title',
                onChanged: (v) => setState(() => _title = v),
              ),
            ),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(width: 120, child: Text('Message', style: TextStyle(color: c.mutedForeground, fontFamily: 'Outfit', fontSize: 13))),
            Expanded(
              child: SomaInput(
                controller: _messageController,
                placeholder: 'Dialog message',
                onChanged: (v) => setState(() => _message = v),
              ),
            ),
          ]),
        ],
      ),
    );
  }
}
