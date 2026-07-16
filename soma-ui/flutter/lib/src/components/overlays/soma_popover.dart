import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import 'soma_menu.dart';

export 'soma_menu.dart' show SomaMenuItem;

// ── SomaPopover ───────────────────────────────────────────────────────────────

class SomaPopover extends StatefulWidget {
  final Widget trigger;
  final Widget content;

  const SomaPopover({super.key, required this.trigger, required this.content});

  @override
  State<SomaPopover> createState() => _SomaPopoverState();
}

class _SomaPopoverState extends State<SomaPopover> {
  final LayerLink _link = LayerLink();
  OverlayEntry? _entry;

  void _open() {
    if (_entry != null) return;
    final c = SomaTheme.of(context);
    final panel = Container(
      constraints: const BoxConstraints(maxWidth: 320),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: c.card,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: c.border),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withAlpha(12),
            blurRadius: 6,
          ),
          BoxShadow(
            color: Colors.black.withAlpha(22),
            blurRadius: 24,
            offset: const Offset(0, 10),
          ),
        ],
      ),
      child: DefaultTextStyle(
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 14,
          color: c.cardForeground,
        ),
        child: widget.content,
      ),
    );
    _entry = buildAnchoredEntry(link: _link, panel: panel, onClose: _close);
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
