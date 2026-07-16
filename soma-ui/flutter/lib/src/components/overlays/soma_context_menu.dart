import 'package:flutter/material.dart';
import 'soma_menu.dart';

class SomaContextMenu extends StatefulWidget {
  final Widget child;
  final List<SomaMenuItem> items;

  const SomaContextMenu({
    super.key,
    required this.child,
    required this.items,
  });

  @override
  State<SomaContextMenu> createState() => _SomaContextMenuState();
}

class _SomaContextMenuState extends State<SomaContextMenu> {
  OverlayEntry? _entry;

  void _show(Offset globalPosition) {
    _dismiss();
    final items = widget.items;
    _entry = OverlayEntry(
      builder: (ctx) => Stack(
        children: [
          Positioned.fill(
            child: GestureDetector(
              onTap: _dismiss,
              behavior: HitTestBehavior.translucent,
              child: const SizedBox.expand(),
            ),
          ),
          Positioned(
            left: globalPosition.dx,
            top: globalPosition.dy,
            child: Material(
              color: Colors.transparent,
              child: MenuPanel(items: items, onClose: _dismiss),
            ),
          ),
        ],
      ),
    );
    Overlay.of(context).insert(_entry!);
  }

  void _dismiss() {
    _entry?.remove();
    _entry = null;
  }

  @override
  void dispose() {
    _entry?.remove();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onSecondaryTapDown: (d) => _show(d.globalPosition),
      onLongPressStart: (d) => _show(d.globalPosition),
      child: widget.child,
    );
  }
}
