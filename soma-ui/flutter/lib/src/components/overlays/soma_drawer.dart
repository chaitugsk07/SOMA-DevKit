import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '_panel_parts.dart';

Future<T?> showSomaDrawer<T>(
  BuildContext context, {
  required WidgetBuilder builder,
}) {
  return showModalBottomSheet<T>(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    barrierColor: Colors.black.withAlpha(140),
    builder: (ctx) => _DrawerPanel(child: builder(ctx)),
  );
}

class _DrawerPanel extends StatelessWidget {
  final Widget child;

  const _DrawerPanel({required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      constraints: BoxConstraints(
        maxHeight: MediaQuery.of(context).size.height * 0.85,
      ),
      decoration: BoxDecoration(
        color: c.card,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
        // Border.all is uniform — required when borderRadius is set.
        border: Border.all(color: c.border),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(20),
            blurRadius: 8,
          ),
          BoxShadow(
            color: Colors.black.withAlpha(40),
            blurRadius: 48,
            offset: const Offset(0, 24),
          ),
        ],
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // Drag handle
          Padding(
            padding: const EdgeInsets.only(top: 12, bottom: 8),
            child: Container(
              width: 48,
              height: 4,
              decoration: BoxDecoration(
                color: c.muted,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
          ),
          Flexible(
            child: SingleChildScrollView(
              padding: const EdgeInsets.fromLTRB(24, 0, 24, 24),
              child: child,
            ),
          ),
        ],
      ),
    );
  }
}

class SomaDrawerHeader extends StatelessWidget {
  final Widget? child;

  const SomaDrawerHeader({super.key, this.child});

  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.only(bottom: 16),
        child: child,
      );
}

class SomaDrawerTitle extends StatelessWidget {
  final String text;

  const SomaDrawerTitle({super.key, required this.text});

  @override
  Widget build(BuildContext context) => PanelTitle(text: text);
}

class SomaDrawerDescription extends StatelessWidget {
  final String text;

  const SomaDrawerDescription({super.key, required this.text});

  @override
  Widget build(BuildContext context) => PanelDescription(text: text);
}

class SomaDrawerFooter extends StatelessWidget {
  final List<Widget> children;

  const SomaDrawerFooter({super.key, required this.children});

  @override
  Widget build(BuildContext context) => PanelFooter(children: children);
}
