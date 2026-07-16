import 'package:flutter/material.dart';
import '../inputs/soma_button.dart';
import '../inputs/soma_checkbox.dart';
import '../inputs/soma_input.dart';
import '../inputs/soma_select.dart';
import '../inputs/soma_textarea.dart';
import 'soma_field.dart';
import 'soma_form.dart';

// ponytail: Runtime schema-driven form. Manages one TextEditingController / bool per field.
// Ceiling: no regex/type validation — callers validate in onSubmit.

enum SomaAutoFormFieldKind {
  text,
  email,
  password,
  number,
  textarea,
  checkbox,
  select,
}

class SomaAutoFormField {
  final String name;
  final String label;
  final SomaAutoFormFieldKind kind;
  /// Required for [SomaAutoFormFieldKind.select]
  final List<String>? options;
  final String? placeholder;
  // ignore: non_constant_identifier_names
  final bool required_;

  const SomaAutoFormField({
    required this.name,
    required this.label,
    required this.kind,
    this.options,
    this.placeholder,
    // ignore: non_constant_identifier_names
    this.required_ = false,
  });
}

class SomaAutoForm extends StatefulWidget {
  final List<SomaAutoFormField> fields;
  final void Function(Map<String, String> values)? onSubmit;
  final String submitLabel;

  const SomaAutoForm({
    super.key,
    required this.fields,
    this.onSubmit,
    this.submitLabel = 'Submit',
  });

  @override
  State<SomaAutoForm> createState() => _SomaAutoFormState();
}

class _SomaAutoFormState extends State<SomaAutoForm> {
  late final List<TextEditingController> _controllers;
  late final List<String?> _selectValues;
  late final List<bool> _checkValues;

  @override
  void initState() {
    super.initState();
    _controllers = widget.fields.map((_) => TextEditingController()).toList();
    _selectValues = widget.fields.map((_) => null as String?).toList();
    _checkValues = widget.fields.map((_) => false).toList();
  }

  @override
  void dispose() {
    for (final c in _controllers) {
      c.dispose();
    }
    super.dispose();
  }

  void _handleSubmit() {
    final values = <String, String>{};
    for (var i = 0; i < widget.fields.length; i++) {
      final f = widget.fields[i];
      final String v;
      switch (f.kind) {
        case SomaAutoFormFieldKind.checkbox:
          v = _checkValues[i] ? 'true' : 'false';
        case SomaAutoFormFieldKind.select:
          v = _selectValues[i] ?? '';
        default:
          v = _controllers[i].text;
      }
      values[f.name] = v;
    }
    widget.onSubmit?.call(values);
  }

  @override
  Widget build(BuildContext context) {
    final fieldWidgets = <Widget>[
      for (var i = 0; i < widget.fields.length; i++) _buildField(i),
      SomaButton(
        onPressed: _handleSubmit,
        child: Text(widget.submitLabel),
      ),
    ];

    return SomaForm(children: fieldWidgets);
  }

  Widget _buildField(int i) {
    final f = widget.fields[i];
    Widget input;

    switch (f.kind) {
      case SomaAutoFormFieldKind.textarea:
        input = SomaTextarea(
          controller: _controllers[i],
          placeholder: f.placeholder,
        );
      case SomaAutoFormFieldKind.checkbox:
        input = SomaCheckbox(
          value: _checkValues[i],
          onChanged: (v) => setState(() => _checkValues[i] = v),
        );
      case SomaAutoFormFieldKind.select:
        final opts = f.options ?? [];
        input = SomaSelect<String>(
          items: opts.map((o) => SomaSelectItem(value: o, label: o)).toList(),
          value: _selectValues[i],
          placeholder: f.placeholder,
          onChanged: (v) => setState(() => _selectValues[i] = v),
        );
      case SomaAutoFormFieldKind.email:
        input = SomaInput(
          controller: _controllers[i],
          placeholder: f.placeholder,
          keyboardType: TextInputType.emailAddress,
        );
      case SomaAutoFormFieldKind.number:
        input = SomaInput(
          controller: _controllers[i],
          placeholder: f.placeholder,
          keyboardType: TextInputType.number,
        );
      case SomaAutoFormFieldKind.password:
        input = SomaInput(
          controller: _controllers[i],
          placeholder: f.placeholder,
          obscureText: true,
        );
      default:
        input = SomaInput(
          controller: _controllers[i],
          placeholder: f.placeholder,
        );
    }

    return SomaField(
      label: f.label,
      // ignore: non_constant_identifier_names
      required_: f.required_,
      child: input,
    );
  }
}
