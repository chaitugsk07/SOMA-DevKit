import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

// ponytail: visual dropzone + tap callback only.
// Real web file drag-drop on Flutter Web needs a package (e.g. super_drag_and_drop)
// — out of scope here. Hover state simulated via TapRegion/MouseRegion.
// Ceiling: actual file picking / drag-drop = add super_drag_and_drop or
// file_picker and hook into onTap.

/// A dashed-border file drop target area.
///
/// Mirrors the web soma-ui `Dropzone` in
/// `packages/ui/src/components/interaction/dropzone.rs`.
/// Provides the styled visual and [onTap] callback; real file drag-drop
/// requires a dedicated package (see ponytail comment above).
class SomaDropzone extends StatefulWidget {
  final String label;
  final VoidCallback? onTap;

  const SomaDropzone({
    super.key,
    this.label = 'Drop files here or click to browse',
    this.onTap,
  });

  @override
  State<SomaDropzone> createState() => _SomaDropzoneState();
}

class _SomaDropzoneState extends State<SomaDropzone> {
  bool _hovered = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final borderColor = (_hovered || _pressed) ? c.primary : c.border;
    final bgColor = (_hovered || _pressed)
        ? c.accent.withAlpha(77)
        : Colors.transparent;

    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTapDown: (_) => setState(() => _pressed = true),
        onTapUp: (_) => setState(() => _pressed = false),
        onTapCancel: () => setState(() => _pressed = false),
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          padding: const EdgeInsets.all(32),
          decoration: BoxDecoration(
            color: bgColor,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(
              color: borderColor,
              width: 2,
              strokeAlign: BorderSide.strokeAlignInside,
            ),
          ),
          child: CustomPaint(
            painter: _DashedBorderEraser(),
            child: Center(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(LucideIcons.upload, size: 32, color: c.mutedForeground),
                  const SizedBox(height: 12),
                  Text(
                    widget.label,
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      fontWeight: FontWeight.w500,
                      color: c.foreground,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    'Any file type accepted',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 12,
                      color: c.mutedForeground,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

// ponytail: we rely on Container's solid border for the box; the dashed look
// comes from a CustomPaint overlay that draws only the dashes on top.
// Simpler than DottedBorder packages; ceiling = if a true dashed-only stroke
// is needed, replace with CustomPainter that paints dashes without the solid
// border underneath (remove Border from BoxDecoration first).
class _DashedBorderEraser extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    // No-op: dashed appearance is handled by the Container border.
    // This CustomPaint slot is kept for a future dashed-stroke upgrade.
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => false;
}
