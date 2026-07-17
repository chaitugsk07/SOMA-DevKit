import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

// Covers both toast.rs (ToastVariant + title/description) and sonner.rs
// (SonnerVariant + single message). One implementation; showSomaSonner is
// an alias at the bottom of this file.

enum SomaToastVariant { normal, success, destructive, warning, info }

// ── Manager ──────────────────────────────────────────────────────────────────

class _ToastEntry {
  final OverlayEntry overlay;
  final GlobalKey<_ToastCardState> cardKey;

  _ToastEntry(this.overlay, this.cardKey);
}

class _ToastManager {
  _ToastManager._();
  static final _ToastManager instance = _ToastManager._();

  final List<_ToastEntry> _entries = [];

  void add(_ToastEntry entry) {
    _entries.add(entry);
    _relayout();
  }

  void remove(_ToastEntry entry) {
    _entries.remove(entry);
    entry.overlay.remove();
    _relayout();
  }

  // Recompute bottom offsets so toasts stack without overlap.
  void _relayout() {
    const cardHeight = 72.0; // approximate; enough gap for stacking
    const gap = 8.0;
    const baseBottom = 24.0;
    for (int i = 0; i < _entries.length; i++) {
      final bottomOffset = baseBottom + i * (cardHeight + gap);
      _entries[i].cardKey.currentState?._updateBottom(bottomOffset);
    }
  }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Shows a stacked toast notification anchored to the bottom-right of the screen.
///
/// Covers both Toast (title + optional description) and Sonner (single message).
void showSomaToast(
  BuildContext context, {
  required String message,
  String? description,
  SomaToastVariant variant = SomaToastVariant.normal,
  Duration duration = const Duration(seconds: 4),
}) {
  final overlay = Overlay.of(context, rootOverlay: true);

  late _ToastEntry entry;
  final cardKey = GlobalKey<_ToastCardState>();

  final overlayEntry = OverlayEntry(
    builder: (_) => _ToastCard(
      key: cardKey,
      message: message,
      description: description,
      variant: variant,
      duration: duration,
      onDismiss: () => _ToastManager.instance.remove(entry),
    ),
  );

  entry = _ToastEntry(overlayEntry, cardKey);
  _ToastManager.instance.add(entry);
  overlay.insert(overlayEntry);
}

/// Alias for Sonner parity — same stacked toaster, single-message style.
void showSomaSonner(
  BuildContext context, {
  required String message,
  SomaToastVariant variant = SomaToastVariant.normal,
  Duration duration = const Duration(seconds: 4),
}) =>
    showSomaToast(context, message: message, variant: variant, duration: duration);

// ── Card widget ───────────────────────────────────────────────────────────────

class _ToastCard extends StatefulWidget {
  final String message;
  final String? description;
  final SomaToastVariant variant;
  final Duration duration;
  final VoidCallback onDismiss;

  const _ToastCard({
    super.key,
    required this.message,
    this.description,
    required this.variant,
    required this.duration,
    required this.onDismiss,
  });

  @override
  State<_ToastCard> createState() => _ToastCardState();
}

class _ToastCardState extends State<_ToastCard>
    with SingleTickerProviderStateMixin {
  double _bottom = 24.0;
  late AnimationController _ctrl;
  late Animation<Offset> _slide;
  late Animation<double> _fade;
  bool _dismissed = false;

  @override
  void initState() {
    super.initState();
    _ctrl = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 280),
    );
    _slide = Tween<Offset>(
      begin: const Offset(0.4, 0),
      end: Offset.zero,
    ).animate(CurvedAnimation(parent: _ctrl, curve: Curves.easeOutCubic));
    _fade = Tween<double>(begin: 0.0, end: 1.0)
        .animate(CurvedAnimation(parent: _ctrl, curve: Curves.easeOutCubic));
    _ctrl.forward();

    Future.delayed(widget.duration, _dismiss);
  }

  void _updateBottom(double v) {
    if (mounted) setState(() => _bottom = v);
  }

  void _dismiss() {
    if (_dismissed) return;
    _dismissed = true;
    _ctrl.reverse().then((_) => widget.onDismiss());
  }

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final Color accentColor;
    final IconData iconData;

    switch (widget.variant) {
      case SomaToastVariant.normal:
        accentColor = c.border;
        iconData = LucideIcons.info;
      case SomaToastVariant.success:
        accentColor = c.success;
        iconData = LucideIcons.circleCheck;
      case SomaToastVariant.destructive:
        accentColor = c.destructive;
        iconData = LucideIcons.circleX;
      case SomaToastVariant.warning:
        accentColor = c.warning;
        iconData = LucideIcons.triangleAlert;
      case SomaToastVariant.info:
        accentColor = c.info;
        iconData = LucideIcons.info;
    }

    return AnimatedPositioned(
      duration: const Duration(milliseconds: 200),
      curve: Curves.easeOutCubic,
      right: 16,
      bottom: _bottom,
      child: FadeTransition(
        opacity: _fade,
        child: SlideTransition(
          position: _slide,
          child: Material(
            color: Colors.transparent,
            child: Container(
              constraints: const BoxConstraints(minWidth: 280, maxWidth: 360),
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(8),
                border: Border.fromBorderSide(BorderSide(color: c.border)),
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
              child: ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: Stack(
                  children: [
                    Container(
                      color: c.card,
                      padding: const EdgeInsets.fromLTRB(16 + 4, 12, 16, 12),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Padding(
                            padding: const EdgeInsets.only(top: 1),
                            child: Icon(iconData, size: 16, color: accentColor),
                          ),
                          const SizedBox(width: 10),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Text(
                                  widget.message,
                                  style: TextStyle(
                                    fontFamily: 'Rajdhani',
                                    fontSize: 15,
                                    fontWeight: FontWeight.w600,
                                    color: c.foreground,
                                    letterSpacing: 0.1,
                                  ),
                                ),
                                if (widget.description != null) ...[
                                  const SizedBox(height: 2),
                                  Text(
                                    widget.description!,
                                    style: TextStyle(
                                      fontFamily: 'Outfit',
                                      fontSize: 13,
                                      color: c.mutedForeground,
                                    ),
                                  ),
                                ],
                              ],
                            ),
                          ),
                          GestureDetector(
                            onTap: _dismiss,
                            child: Padding(
                              padding: const EdgeInsets.only(left: 8),
                              child: Icon(LucideIcons.x, size: 14, color: c.mutedForeground),
                            ),
                          ),
                        ],
                      ),
                    ),
                    // Left accent strip — clipped to rounded corners by ClipRRect above
                    Positioned(
                      left: 0,
                      top: 0,
                      bottom: 0,
                      child: Container(width: 4, color: accentColor),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
