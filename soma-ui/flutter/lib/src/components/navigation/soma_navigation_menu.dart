import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '../overlays/soma_menu.dart';

/// A single item in a [SomaNavigationMenu].
class SomaNavigationMenuItem {
  final String label;
  final Widget content;

  const SomaNavigationMenuItem({
    required this.label,
    required this.content,
  });
}

/// Horizontal navigation menu where each trigger opens an anchored content panel.
///
/// Data-driven (divergence from web's composable NavigationMenu*): accepts
/// [items] and renders trigger buttons; clicking a trigger opens its [content]
/// in a panel anchored below using [buildAnchoredEntry] from soma_menu.dart.
/// Tapping outside or switching triggers closes the open panel.
class SomaNavigationMenu extends StatefulWidget {
  final List<SomaNavigationMenuItem> items;

  const SomaNavigationMenu({super.key, required this.items});

  @override
  State<SomaNavigationMenu> createState() => _SomaNavigationMenuState();
}

class _SomaNavigationMenuState extends State<SomaNavigationMenu> {
  final List<LayerLink> _links = [];
  OverlayEntry? _entry;
  int? _openIndex;

  @override
  void initState() {
    super.initState();
    _links.addAll(List.generate(widget.items.length, (_) => LayerLink()));
  }

  @override
  void didUpdateWidget(SomaNavigationMenu old) {
    super.didUpdateWidget(old);
    if (widget.items.length != old.items.length) {
      _links.clear();
      _links.addAll(List.generate(widget.items.length, (_) => LayerLink()));
    }
  }

  void _close() {
    _entry?.remove();
    _entry = null;
    if (mounted) setState(() => _openIndex = null);
  }

  void _open(int index) {
    _close();
    final c = SomaTheme.of(context);
    _entry = buildAnchoredEntry(
      link: _links[index],
      onClose: _close,
      panel: _ContentPanel(
        content: widget.items[index].content,
        colors: c,
      ),
    );
    Overlay.of(context).insert(_entry!);
    setState(() => _openIndex = index);
  }

  void _toggle(int index) {
    if (_openIndex == index) {
      _close();
    } else {
      _open(index);
    }
  }

  @override
  void dispose() {
    _entry?.remove();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: List.generate(widget.items.length, (i) {
        final isOpen = _openIndex == i;
        return CompositedTransformTarget(
          link: _links[i],
          child: _TriggerButton(
            label: widget.items[i].label,
            isOpen: isOpen,
            colors: c,
            onTap: () => _toggle(i),
          ),
        );
      }),
    );
  }
}

class _TriggerButton extends StatefulWidget {
  final String label;
  final bool isOpen;
  final SomaColors colors;
  final VoidCallback onTap;

  const _TriggerButton({
    required this.label,
    required this.isOpen,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_TriggerButton> createState() => _TriggerButtonState();
}

class _TriggerButtonState extends State<_TriggerButton> {
  bool _hovered = false;
  bool _focused = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    final active = widget.isOpen || _hovered;

    List<BoxShadow>? shadows;
    if (_focused) {
      shadows = [BoxShadow(color: c.ring.withAlpha(50), blurRadius: 0, spreadRadius: 2)];
    }

    return Focus(
      onFocusChange: (f) => setState(() => _focused = f),
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: widget.onTap,
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 140),
            curve: Curves.easeOutCubic,
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            decoration: BoxDecoration(
              color: active ? c.accent : Colors.transparent,
              borderRadius: BorderRadius.circular(6),
              boxShadow: shadows,
            ),
            child: Text(
              widget.label,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 14,
                fontWeight: FontWeight.w500,
                color: active ? c.accentForeground : c.foreground,
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _ContentPanel extends StatelessWidget {
  final Widget content;
  final SomaColors colors;

  const _ContentPanel({required this.content, required this.colors});

  @override
  Widget build(BuildContext context) {
    final c = colors;
    return Container(
      constraints: const BoxConstraints(minWidth: 200, maxWidth: 480),
      decoration: BoxDecoration(
        color: c.card,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: c.border),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(50),
            blurRadius: 20,
            offset: const Offset(0, 8),
          ),
          BoxShadow(
            color: Colors.black.withAlpha(20),
            blurRadius: 6,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: DefaultTextStyle(
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 14,
            color: c.foreground,
          ),
          child: content,
        ),
      ),
    );
  }
}
