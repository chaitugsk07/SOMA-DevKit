import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaDialog extends StatelessWidget {
  final String? title;
  final String? description;
  final List<Widget> actions;
  final Widget? child;

  const SomaDialog({
    super.key,
    this.title,
    this.description,
    this.actions = const [],
    this.child,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Dialog(
      backgroundColor: c.card,
      elevation: 8,
      shadowColor: Colors.black.withAlpha(60),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(8),
        side: BorderSide(color: c.border),
      ),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 512),
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              if (title != null || description != null) ...[
                if (title != null)
                  Text(
                    title!,
                    style: TextStyle(
                      fontFamily: 'Rajdhani',
                      fontSize: 20,
                      fontWeight: FontWeight.w600,
                      color: c.cardForeground,
                      height: 1.0,
                    ),
                  ),
                if (description != null) ...[
                  const SizedBox(height: 4),
                  Text(
                    description!,
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: c.mutedForeground,
                    ),
                  ),
                ],
                const SizedBox(height: 16),
              ],
              // Body
              if (child != null) ...[
                DefaultTextStyle(
                  style: TextStyle(fontFamily: 'Outfit', color: c.cardForeground),
                  child: child!,
                ),
                const SizedBox(height: 24),
              ],
              // Footer actions
              if (actions.isNotEmpty)
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: actions
                      .map((a) => Padding(
                            padding: const EdgeInsets.only(left: 8),
                            child: a,
                          ))
                      .toList(),
                ),
            ],
          ),
        ),
      ),
    );
  }
}

/// Shows a [SomaDialog] as a modal route.
Future<T?> showSomaDialog<T>({
  required BuildContext context,
  String? title,
  String? description,
  List<Widget> actions = const [],
  Widget? child,
  bool barrierDismissible = true,
}) {
  return showDialog<T>(
    context: context,
    barrierDismissible: barrierDismissible,
    barrierColor: Colors.black.withAlpha(140),
    builder: (_) => SomaDialog(
      title: title,
      description: description,
      actions: actions,
      child: child,
    ),
  );
}
