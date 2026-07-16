import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaLabel extends StatelessWidget {
  final String text;
  // ignore: non_constant_identifier_names
  final bool required_;

  const SomaLabel(
    this.text, {
    super.key,
    // ignore: non_constant_identifier_names
    this.required_ = false,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return RichText(
      text: TextSpan(
        children: [
          TextSpan(
            text: text,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              fontWeight: FontWeight.w500,
              color: c.foreground,
            ),
          ),
          if (required_)
            TextSpan(
              text: ' *',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 14,
                fontWeight: FontWeight.w500,
                color: c.destructive,
              ),
            ),
        ],
      ),
    );
  }
}
