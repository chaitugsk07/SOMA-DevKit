import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AlertDialogScreen extends StatefulWidget {
  const AlertDialogScreen({super.key});

  @override
  State<AlertDialogScreen> createState() => _AlertDialogScreenState();
}

class _AlertDialogScreenState extends State<AlertDialogScreen> {
  bool _destructive = true;
  String _lastResult = '—';

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'AlertDialog',
      subtitle: 'A modal confirm/cancel dialog with optional destructive styling.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaButton(
            variant: _destructive ? SomaButtonVariant.destructive : SomaButtonVariant.primary,
            onPressed: () async {
              final confirmed = await showSomaAlertDialog(
                context,
                title: _destructive ? 'Delete item?' : 'Confirm action',
                description: _destructive
                    ? 'This action cannot be undone. The item will be permanently deleted.'
                    : 'Are you sure you want to proceed with this action?',
                confirmLabel: _destructive ? 'Delete' : 'Continue',
                destructive: _destructive,
              );
              setState(() => _lastResult = confirmed ? 'Confirmed' : 'Cancelled');
            },
            child: Text(_destructive ? 'Delete item' : 'Open dialog'),
          ),
          const SizedBox(height: 16),
          Text(
            'Result: $_lastResult',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: SomaTheme.of(context).mutedForeground,
            ),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ControlRow(
            label: 'Destructive',
            child: SomaSwitch(
              value: _destructive,
              onChanged: (v) => setState(() => _destructive = v),
            ),
          ),
        ],
      ),
    );
  }
}
