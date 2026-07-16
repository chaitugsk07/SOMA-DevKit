import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import 'soma_label.dart';

/// Form-field wrapper: label + child input + optional description + optional error.
///
/// ponytail: thin column wrapper; callers own validation state.
class SomaField extends StatelessWidget {
  final String? label;
  final String? description;
  final String? error;
  final bool required_;
  final Widget child;

  const SomaField({
    super.key,
    this.label,
    this.description,
    this.error,
    // ignore: non_constant_identifier_names
    this.required_ = false,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        if (label != null) ...[
          SomaLabel(label!, required_: required_),
          const SizedBox(height: 6),
        ],
        child,
        if (error != null) ...[
          const SizedBox(height: 4),
          Text(
            error!,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 12,
              color: c.destructive,
            ),
          ),
        ] else if (description != null) ...[
          const SizedBox(height: 4),
          Text(
            description!,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 12,
              color: c.mutedForeground,
            ),
          ),
        ],
      ],
    );
  }
}

/// Thin wrappers for manual composition (FieldLabel, FieldDescription, FieldError).

class SomaFieldLabel extends StatelessWidget {
  final String text;
  // ignore: non_constant_identifier_names
  final bool required_;

  const SomaFieldLabel(
    this.text, {
    super.key,
    // ignore: non_constant_identifier_names
    this.required_ = false,
  });

  @override
  Widget build(BuildContext context) => SomaLabel(text, required_: required_);
}

class SomaFieldDescription extends StatelessWidget {
  final String text;
  const SomaFieldDescription(this.text, {super.key});

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

class SomaFieldError extends StatelessWidget {
  final String text;
  const SomaFieldError(this.text, {super.key});

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
