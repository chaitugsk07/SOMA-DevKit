import 'package:flutter/material.dart';
import 'soma_menu.dart';

class SomaDropdownMenu extends StatefulWidget {
  final Widget trigger;
  final List<SomaMenuItem> items;

  const SomaDropdownMenu({
    super.key,
    required this.trigger,
    required this.items,
  });

  @override
  State<SomaDropdownMenu> createState() => _SomaDropdownMenuState();
}

class _SomaDropdownMenuState extends State<SomaDropdownMenu> {
  final LayerLink _link = LayerLink();
  OverlayEntry? _entry;

  void _open() {
    if (_entry != null) return;
    _entry = buildAnchoredEntry(
      link: _link,
      onClose: _close,
      panel: MenuPanel(items: widget.items, onClose: _close),
    );
    Overlay.of(context).insert(_entry!);
    setState(() {});
  }

  void _close() {
    _entry?.remove();
    _entry = null;
    if (mounted) setState(() {});
  }

  @override
  void dispose() {
    _entry?.remove();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return CompositedTransformTarget(
      link: _link,
      child: GestureDetector(
        onTap: _entry == null ? _open : _close,
        child: widget.trigger,
      ),
    );
  }
}
