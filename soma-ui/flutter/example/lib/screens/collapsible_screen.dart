import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class CollapsibleScreen extends StatefulWidget {
  const CollapsibleScreen({super.key});

  @override
  State<CollapsibleScreen> createState() => _CollapsibleScreenState();
}

class _CollapsibleScreenState extends State<CollapsibleScreen> {
  bool _open = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Collapsible',
      subtitle: 'Controlled show/hide panel — caller owns the open state and trigger.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                'Details',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 14,
                  fontWeight: FontWeight.w500,
                  color: c.foreground,
                ),
              ),
              const SizedBox(width: 12),
              SomaButton(
                variant: SomaButtonVariant.ghost,
                size: SomaButtonSize.sm,
                onPressed: () => setState(() => _open = !_open),
                child: Text(_open ? 'Hide' : 'Show'),
              ),
            ],
          ),
          const SizedBox(height: 8),
          SomaCollapsible(
            open: _open,
            child: SomaCard(
              padding: const EdgeInsets.all(16),
              child: Text(
                'This content animates in and out based on the open state. '
                'The trigger is the caller\'s responsibility — SomaCollapsible is purely '
                'a controlled animation container.',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 14,
                  color: c.mutedForeground,
                ),
              ),
            ),
          ),
        ],
      ),
      controls: Row(
        children: [
          SizedBox(
            width: 120,
            child: Text(
              'Open',
              style: TextStyle(
                color: c.mutedForeground,
                fontFamily: 'Outfit',
                fontSize: 13,
              ),
            ),
          ),
          SomaSwitch(
            value: _open,
            onChanged: (v) => setState(() => _open = v),
          ),
        ],
      ),
    );
  }
}
