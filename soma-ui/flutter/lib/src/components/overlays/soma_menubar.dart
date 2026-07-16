import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import 'soma_menu.dart';

class SomaMenu {
  final String label;
  final List<SomaMenuItem> items;

  const SomaMenu({required this.label, required this.items});
}

class SomaMenubar extends StatefulWidget {
  final List<SomaMenu> menus;

  const SomaMenubar({super.key, required this.menus});

  @override
  State<SomaMenubar> createState() => _SomaMenubarState();
}

class _SomaMenubarState extends State<SomaMenubar> {
  int? _openIndex;
  late final List<LayerLink> _links;
  OverlayEntry? _entry;

  @override
  void initState() {
    super.initState();
    _links = List.generate(widget.menus.length, (_) => LayerLink());
  }

  void _openMenu(int index) {
    _closeMenu();
    final menu = widget.menus[index];
    _entry = buildAnchoredEntry(
      link: _links[index],
      onClose: _closeMenu,
      panel: MenuPanel(items: menu.items, onClose: _closeMenu),
      offset: const Offset(0, 4),
    );
    Overlay.of(context).insert(_entry!);
    setState(() => _openIndex = index);
  }

  void _closeMenu() {
    _entry?.remove();
    _entry = null;
    if (mounted) setState(() => _openIndex = null);
  }

  @override
  void dispose() {
    _entry?.remove();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      padding: const EdgeInsets.all(4),
      decoration: BoxDecoration(
        color: c.card,
        borderRadius: BorderRadius.circular(6),
        border: Border.all(color: c.border),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: List.generate(widget.menus.length, (i) {
          final isOpen = _openIndex == i;
          return CompositedTransformTarget(
            link: _links[i],
            child: _MenubarTrigger(
              label: widget.menus[i].label,
              isOpen: isOpen,
              onTap: () => isOpen ? _closeMenu() : _openMenu(i),
              onHover: () {
                if (_openIndex != null && !isOpen) _openMenu(i);
              },
            ),
          );
        }),
      ),
    );
  }
}

class _MenubarTrigger extends StatefulWidget {
  final String label;
  final bool isOpen;
  final VoidCallback onTap;
  final VoidCallback onHover;

  const _MenubarTrigger({
    required this.label,
    required this.isOpen,
    required this.onTap,
    required this.onHover,
  });

  @override
  State<_MenubarTrigger> createState() => _MenubarTriggerState();
}

class _MenubarTriggerState extends State<_MenubarTrigger> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final active = widget.isOpen || _hovered;
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) {
        setState(() => _hovered = true);
        widget.onHover();
      },
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 100),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
          decoration: BoxDecoration(
            color: active ? c.accent : Colors.transparent,
            borderRadius: BorderRadius.circular(4),
          ),
          child: Text(
            widget.label,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: active ? c.accentForeground : c.foreground,
            ),
          ),
        ),
      ),
    );
  }
}
