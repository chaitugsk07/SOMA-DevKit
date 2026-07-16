import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../inputs/soma_button.dart';

/// A confirm/cancel alert dialog with optional destructive styling.
///
/// Preferred entry point: [showSomaAlertDialog], which returns [true] when
/// the user confirms and [false] (or null→false) when they cancel/dismiss.
class SomaAlertDialog extends StatelessWidget {
  final String title;
  final String? description;
  final String confirmLabel;
  final String cancelLabel;
  final bool destructive;

  const SomaAlertDialog({
    super.key,
    required this.title,
    this.description,
    this.confirmLabel = 'Continue',
    this.cancelLabel = 'Cancel',
    this.destructive = true,
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
        constraints: const BoxConstraints(maxWidth: 480),
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              Text(
                title,
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 20,
                  fontWeight: FontWeight.w600,
                  color: c.cardForeground,
                  height: 1.0,
                ),
              ),
              if (description != null) ...[
                const SizedBox(height: 8),
                Text(
                  description!,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.mutedForeground,
                  ),
                ),
              ],
              const SizedBox(height: 24),
              // Footer
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  SomaButton(
                    variant: SomaButtonVariant.outline,
                    onPressed: () => Navigator.of(context).pop(false),
                    child: Text(cancelLabel),
                  ),
                  const SizedBox(width: 8),
                  SomaButton(
                    variant: destructive
                        ? SomaButtonVariant.destructive
                        : SomaButtonVariant.primary,
                    onPressed: () => Navigator.of(context).pop(true),
                    child: Text(confirmLabel),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// Shows a [SomaAlertDialog] modally. Returns [true] if confirmed, [false] otherwise.
Future<bool> showSomaAlertDialog(
  BuildContext context, {
  required String title,
  String? description,
  String confirmLabel = 'Continue',
  String cancelLabel = 'Cancel',
  bool destructive = true,
}) async {
  final result = await showDialog<bool>(
    context: context,
    barrierDismissible: true,
    barrierColor: Colors.black.withAlpha(140),
    builder: (_) => SomaAlertDialog(
      title: title,
      description: description,
      confirmLabel: confirmLabel,
      cancelLabel: cancelLabel,
      destructive: destructive,
    ),
  );
  return result ?? false;
}
