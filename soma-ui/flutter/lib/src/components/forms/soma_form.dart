import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import 'soma_label.dart';
import 'soma_field.dart';

/// Form container with consistent vertical gaps.
///
/// ponytail: SomaForm is a column with 24px gaps; state lives in the caller.
/// SomaFormItem consolidates to SomaField (same structure, no duplication).
class SomaForm extends StatelessWidget {
  final List<Widget> children;

  const SomaForm({super.key, required this.children});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: _interleave(children, const SizedBox(height: 24)),
    );
  }
}

List<Widget> _interleave(List<Widget> items, Widget separator) {
  if (items.isEmpty) return [];
  final result = <Widget>[items.first];
  for (var i = 1; i < items.length; i++) {
    result.add(separator);
    result.add(items[i]);
  }
  return result;
}

/// FormItem = SomaField re-export for shadcn-compatible manual composition.
/// ponytail: no duplication — alias via typedef.
typedef SomaFormItem = SomaField;

/// Thin wrappers matching the web's FormLabel/FormDescription/FormMessage.

class SomaFormLabel extends StatelessWidget {
  final String text;
  // ignore: non_constant_identifier_names
  final bool required_;

  const SomaFormLabel(
    this.text, {
    super.key,
    // ignore: non_constant_identifier_names
    this.required_ = false,
  });

  @override
  Widget build(BuildContext context) => SomaLabel(text, required_: required_);
}

class SomaFormDescription extends StatelessWidget {
  final String text;
  const SomaFormDescription(this.text, {super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Text(
      text,
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 12,
        color: c.mutedForeground,
      ),
    );
  }
}

class SomaFormMessage extends StatelessWidget {
  final String text;
  const SomaFormMessage(this.text, {super.key});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Text(
      text,
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 13,
        color: c.destructive,
      ),
    );
  }
}
