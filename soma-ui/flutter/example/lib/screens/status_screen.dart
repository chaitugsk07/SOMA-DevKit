import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class StatusScreen extends StatefulWidget {
  const StatusScreen({super.key});

  @override
  State<StatusScreen> createState() => _StatusScreenState();
}

class _StatusScreenState extends State<StatusScreen> {
  SomaStatusKind _kind = SomaStatusKind.online;
  bool _showLabel = true;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'Status',
      subtitle: 'Small colored dot indicator with an optional label.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SomaStatus(
            kind: _kind,
            label: _showLabel ? _kind.name : null,
          ),
          const SizedBox(height: 24),
          Text(
            'All Kinds',
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 24,
            runSpacing: 12,
            children: SomaStatusKind.values
                .map((k) => SomaStatus(kind: k, label: k.name))
                .toList(),
          ),
        ],
      ),
      controls: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Kind',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            Expanded(
              child: SomaSelect<SomaStatusKind>(
                items: SomaStatusKind.values
                    .map((k) => SomaSelectItem(value: k, label: k.name))
                    .toList(),
                value: _kind,
                onChanged: (k) => setState(() => _kind = k),
              ),
            ),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            SizedBox(
              width: 120,
              child: Text(
                'Show label',
                style: TextStyle(
                  color: c.mutedForeground,
                  fontFamily: 'Outfit',
                  fontSize: 13,
                ),
              ),
            ),
            SomaSwitch(
              value: _showLabel,
              onChanged: (v) => setState(() => _showLabel = v),
            ),
          ]),
        ],
      ),
    );
  }
}
