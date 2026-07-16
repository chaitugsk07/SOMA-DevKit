import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';
import '_panel_parts.dart';

enum SomaSheetSide { right, left, top, bottom }

Future<T?> showSomaSheet<T>(
  BuildContext context, {
  SomaSheetSide side = SomaSheetSide.right,
  required WidgetBuilder builder,
}) {
  return showGeneralDialog<T>(
    context: context,
    barrierDismissible: true,
    barrierLabel: 'Close',
    barrierColor: Colors.black.withAlpha(140),
    transitionDuration: const Duration(milliseconds: 250),
    pageBuilder: (ctx, _, __) => _SheetPanel(side: side, child: builder(ctx)),
    transitionBuilder: (ctx, anim, _, child) {
      final Offset begin;
      switch (side) {
        case SomaSheetSide.right:
          begin = const Offset(1, 0);
        case SomaSheetSide.left:
          begin = const Offset(-1, 0);
        case SomaSheetSide.top:
          begin = const Offset(0, -1);
        case SomaSheetSide.bottom:
          begin = const Offset(0, 1);
      }
      return SlideTransition(
        position: Tween(begin: begin, end: Offset.zero).animate(
          CurvedAnimation(parent: anim, curve: Curves.easeOutCubic),
        ),
        child: child,
      );
    },
  );
}

class _SheetPanel extends StatefulWidget {
  final SomaSheetSide side;
  final Widget child;

  const _SheetPanel({required this.side, required this.child});

  @override
  State<_SheetPanel> createState() => _SheetPanelState();
}

class _SheetPanelState extends State<_SheetPanel> {
  bool _closeHovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final isVertical = widget.side == SomaSheetSide.left ||
        widget.side == SomaSheetSide.right;

    final BorderRadius radius;
    switch (widget.side) {
      case SomaSheetSide.right:
        radius = const BorderRadius.horizontal(left: Radius.circular(12));
      case SomaSheetSide.left:
        radius = const BorderRadius.horizontal(right: Radius.circular(12));
      case SomaSheetSide.top:
        radius = const BorderRadius.vertical(bottom: Radius.circular(12));
      case SomaSheetSide.bottom:
        radius = const BorderRadius.vertical(top: Radius.circular(12));
    }
    // Border.all is uniform — required when borderRadius is set.
    final border = Border.all(color: c.border);

    final Alignment alignment;
    switch (widget.side) {
      case SomaSheetSide.right:
        alignment = Alignment.centerRight;
      case SomaSheetSide.left:
        alignment = Alignment.centerLeft;
      case SomaSheetSide.top:
        alignment = Alignment.topCenter;
      case SomaSheetSide.bottom:
        alignment = Alignment.bottomCenter;
    }

    return Focus(
      autofocus: true,
      onKeyEvent: (node, e) {
        if (e is KeyDownEvent && e.logicalKey == LogicalKeyboardKey.escape) {
          Navigator.of(context).pop();
          return KeyEventResult.handled;
        }
        return KeyEventResult.ignored;
      },
      child: Align(
        alignment: alignment,
        child: Container(
          width: isVertical ? 320 : double.infinity,
          height: isVertical
              ? double.infinity
              : MediaQuery.of(context).size.height * 0.5,
          decoration: BoxDecoration(
            color: c.card,
            borderRadius: radius,
            border: border,
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
          child: Stack(
            children: [
              Padding(
                padding: const EdgeInsets.all(24),
                child: DefaultTextStyle(
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.cardForeground,
                  ),
                  child: widget.child,
                ),
              ),
              Positioned(
                top: 12,
                right: 12,
                child: MouseRegion(
                  cursor: SystemMouseCursors.click,
                  onEnter: (_) => setState(() => _closeHovered = true),
                  onExit: (_) => setState(() => _closeHovered = false),
                  child: GestureDetector(
                    onTap: () => Navigator.of(context).pop(),
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 100),
                      width: 28,
                      height: 28,
                      decoration: BoxDecoration(
                        color: _closeHovered ? c.accent : Colors.transparent,
                        borderRadius: BorderRadius.circular(4),
                      ),
                      child: Icon(
                        LucideIcons.x,
                        size: 16,
                        color: _closeHovered
                            ? c.accentForeground
                            : c.mutedForeground,
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class SomaSheetHeader extends StatelessWidget {
  final Widget? child;

  const SomaSheetHeader({super.key, this.child});

  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.only(bottom: 16, right: 32),
        child: child,
      );
}

class SomaSheetTitle extends StatelessWidget {
  final String text;

  const SomaSheetTitle({super.key, required this.text});

  @override
  Widget build(BuildContext context) => PanelTitle(text: text);
}

class SomaSheetDescription extends StatelessWidget {
  final String text;

  const SomaSheetDescription({super.key, required this.text});

  @override
  Widget build(BuildContext context) => PanelDescription(text: text);
}

class SomaSheetFooter extends StatelessWidget {
  final List<Widget> children;

  const SomaSheetFooter({super.key, required this.children});

  @override
  Widget build(BuildContext context) => PanelFooter(children: children);
}
