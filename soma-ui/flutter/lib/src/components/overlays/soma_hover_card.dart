import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '_panel_parts.dart';

class SomaHoverCard extends StatefulWidget {
  final Widget trigger;
  final Widget content;

  const SomaHoverCard({
    super.key,
    required this.trigger,
    required this.content,
  });

  @override
  State<SomaHoverCard> createState() => _SomaHoverCardState();
}

class _SomaHoverCardState extends State<SomaHoverCard> {
  OverlayEntry? _entry;
  bool _mouseInTrigger = false;
  bool _mouseInPanel = false;

  void _open() {
    if (_entry != null || !mounted) return;
    final c = SomaTheme.of(context);

    // Compute trigger position for absolute Positioned placement.
    final box = context.findRenderObject() as RenderBox?;
    if (box == null) return;
    final pos = box.localToGlobal(Offset.zero);
    final triggerSize = box.size;

    final panel = MouseRegion(
      onEnter: (_) => _mouseInPanel = true,
      onExit: (_) {
        _mouseInPanel = false;
        Future.delayed(const Duration(milliseconds: 100), _maybeClose);
      },
      child: Container(
        width: 256,
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
      ),
    );

    // HoverCard uses no tap-dismiss barrier — it's hover-driven.
    // ponytail: Positioned replaces CompositedTransformFollower so the panel
    // is clamped to the viewport width. LayerLink removed — not needed here.
    _entry = OverlayEntry(
      builder: (overlayContext) {
        final screen = MediaQuery.sizeOf(overlayContext);
        const panelWidth = 256.0;
        double left = pos.dx;
        if (left + panelWidth > screen.width) {
          left = screen.width - panelWidth - 8; // 8px right margin
        }
        if (left < 0) left = 0;
        final top = pos.dy + triggerSize.height + 8;
        return Positioned(
          left: left,
          top: top,
          width: panelWidth,
          child: Material(color: Colors.transparent, child: FadeScaleIn(child: panel)),
        );
      },
    );
    Overlay.of(context).insert(_entry!);
  }

  void _close() {
    _entry?.remove();
    _entry = null;
  }

  void _maybeClose() {
    if (!mounted) return;
    if (!_mouseInTrigger && !_mouseInPanel) _close();
  }

  @override
  void dispose() {
    _entry?.remove();
    _entry = null;
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) {
        _mouseInTrigger = true;
        Future.delayed(const Duration(milliseconds: 150), () {
          if (_mouseInTrigger && mounted) _open();
        });
      },
      onExit: (_) {
        _mouseInTrigger = false;
        Future.delayed(const Duration(milliseconds: 100), _maybeClose);
      },
      child: widget.trigger,
    );
  }
}
