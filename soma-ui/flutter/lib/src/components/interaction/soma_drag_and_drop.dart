import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

// ponytail: uses Flutter's native ReorderableListView — no JS event splicing
// needed. The web crate manually tracks dragStart/dragOver indices in signals;
// Flutter handles that natively.
// Ceiling: cross-list drag, touch auto-scroll customisation = upgrade path
// (use ReorderableListView.builder or a dedicated package).

/// A reorderable list with a styled drag handle per row.
///
/// Mirrors the web soma-ui `DragAndDrop` in
/// `packages/ui/src/components/interaction/drag_and_drop.rs`.
/// Uses Flutter's [ReorderableListView] instead of HTML5 drag events.
class SomaDragAndDrop extends StatelessWidget {
  final List<Widget> children;
  final void Function(int oldIndex, int newIndex) onReorder;

  const SomaDragAndDrop({
    super.key,
    required this.children,
    required this.onReorder,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    // onReorderItem already adjusts newIndex for the removed element, so we
    // re-apply the classic adjustment to expose the raw (oldIndex, newIndex)
    // semantics the caller expects (insert-before-removal position).
    return ReorderableListView(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      onReorderItem: (oldIndex, newIndex) {
        // onReorderItem gives the final insert position (post-removal adjusted).
        // Convert back to pre-removal position so onReorder callers can do the
        // standard "if (newIndex > oldIndex) newIndex--" idiom.
        onReorder(oldIndex, newIndex > oldIndex ? newIndex + 1 : newIndex);
      },
      proxyDecorator: (child, index, animation) => Material(
        color: Colors.transparent,
        child: child,
      ),
      children: [
        for (int i = 0; i < children.length; i++)
          Padding(
            key: ValueKey(i),
            padding: const EdgeInsets.only(bottom: 4),
            child: Container(
              decoration: BoxDecoration(
                color: c.card,
                border: Border.all(color: c.border),
                borderRadius: BorderRadius.circular(6),
              ),
              child: Row(
                children: [
                  Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 12),
                    child: Icon(
                      LucideIcons.gripVertical,
                      size: 16,
                      color: c.mutedForeground,
                    ),
                  ),
                  Expanded(
                    child: DefaultTextStyle(
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: c.foreground,
                      ),
                      child: children[i],
                    ),
                  ),
                ],
              ),
            ),
          ),
      ],
    );
  }
}
