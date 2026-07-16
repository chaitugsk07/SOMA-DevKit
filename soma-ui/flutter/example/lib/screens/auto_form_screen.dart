import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class AutoFormScreen extends StatefulWidget {
  const AutoFormScreen({super.key});

  @override
  State<AutoFormScreen> createState() => _AutoFormScreenState();
}

class _AutoFormScreenState extends State<AutoFormScreen> {
  Map<String, String>? _submitted;

  static const _fields = [
    SomaAutoFormField(
      name: 'name',
      label: 'Full name',
      kind: SomaAutoFormFieldKind.text,
      placeholder: 'Jane Doe',
    ),
    SomaAutoFormField(
      name: 'email',
      label: 'Email',
      kind: SomaAutoFormFieldKind.email,
      placeholder: 'you@example.com',
    ),
    SomaAutoFormField(
      name: 'role',
      label: 'Role',
      kind: SomaAutoFormFieldKind.select,
      options: ['Engineer', 'Designer', 'Product', 'Other'],
      placeholder: 'Select a role…',
    ),
    SomaAutoFormField(
      name: 'subscribe',
      label: 'Subscribe to newsletter',
      kind: SomaAutoFormFieldKind.checkbox,
    ),
  ];

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ComponentPage(
      title: 'AutoForm',
      subtitle: 'Schema-driven form that renders inputs from a field spec.',
      preview: SizedBox(
        width: 360,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            SomaAutoForm(
              fields: _fields,
              submitLabel: 'Submit',
              onSubmit: (values) => setState(() => _submitted = values),
            ),
            if (_submitted != null) ...[
              const SizedBox(height: 16),
              Text(
                'Submitted:',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 13,
                  fontWeight: FontWeight.w600,
                  color: c.foreground,
                ),
              ),
              const SizedBox(height: 6),
              ..._submitted!.entries.map(
                (e) => Text(
                  '${e.key}: ${e.value}',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 12,
                    color: c.mutedForeground,
                  ),
                ),
              ),
            ],
          ],
        ),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
