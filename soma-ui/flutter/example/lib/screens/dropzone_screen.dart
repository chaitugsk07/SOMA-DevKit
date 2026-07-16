import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DropzoneScreen extends StatefulWidget {
  const DropzoneScreen({super.key});

  @override
  State<DropzoneScreen> createState() => _DropzoneScreenState();
}

class _DropzoneScreenState extends State<DropzoneScreen> {
  String _lastAction = '';
  String _label = 'Drop files here or click to browse';

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Dropzone',
      subtitle: 'Styled file drop target. Tap to trigger the file picker callback.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaDropzone(
            label: _label,
            onTap: () {
              setState(() => _lastAction = 'Tapped — file picker would open here');
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Tapped dropzone — file picker would open here'),
                  duration: Duration(seconds: 2),
                ),
              );
            },
          ),
          if (_lastAction.isNotEmpty) ...[
            const SizedBox(height: 12),
            Text(
              _lastAction,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 12,
                color: c.mutedForeground,
              ),
            ),
          ],
        ],
      ),
      controls: ControlRow(
        label: 'Label',
        child: SomaButton(
          variant: SomaButtonVariant.outline,
          size: SomaButtonSize.sm,
          onPressed: () => setState(() {
            _label = _label.contains('browse')
                ? 'Upload your files'
                : 'Drop files here or click to browse';
          }),
          child: const Text('Toggle label'),
        ),
      ),
    );
  }
}
