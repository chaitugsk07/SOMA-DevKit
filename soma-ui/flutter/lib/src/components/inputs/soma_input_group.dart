import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

/// A bordered container that merges leading/trailing addons with an input child.
///
/// The whole group shares one border and a single focus ring (via FocusScope).
/// Addons get a muted background separated from the child by internal dividers.
class SomaInputGroup extends StatefulWidget {
  final Widget? leading;
  final Widget? trailing;
  final Widget child;

  const SomaInputGroup({
    super.key,
    this.leading,
    this.trailing,
    required this.child,
  });

  @override
  State<SomaInputGroup> createState() => _SomaInputGroupState();
}

class _SomaInputGroupState extends State<SomaInputGroup> {
  bool _focused = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Focus(
      onFocusChange: (f) => setState(() => _focused = f),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(6),
          border: Border.all(
            color: _focused ? c.ring : c.input,
            width: _focused ? 2 : 1,
          ),
          boxShadow: _focused
              ? [
                  BoxShadow(color: c.ring.withAlpha(50), blurRadius: 6, spreadRadius: 1),
                  BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
                ]
              : [BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1))],
        ),
        child: ClipRRect(
          borderRadius: BorderRadius.circular(5),
          child: IntrinsicHeight(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                if (widget.leading != null) ...[
                  _AddonSlot(colors: c, child: widget.leading!),
                  VerticalDivider(width: 1, thickness: 1, color: c.input),
                ],
                Expanded(child: widget.child),
                if (widget.trailing != null) ...[
                  VerticalDivider(width: 1, thickness: 1, color: c.input),
                  _AddonSlot(colors: c, child: widget.trailing!),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _AddonSlot extends StatelessWidget {
  final Widget child;
  final dynamic colors;

  const _AddonSlot({required this.child, required this.colors});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      color: c.muted,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      child: Center(child: child),
    );
  }
}

/// A styled addon widget for use inside [SomaInputGroup].
class SomaInputGroupAddon extends StatelessWidget {
  final Widget child;

  const SomaInputGroupAddon({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return DefaultTextStyle(
      style: TextStyle(
        fontFamily: 'Outfit',
        fontSize: 14,
        color: c.mutedForeground,
      ),
      child: IconTheme(
        data: IconThemeData(color: c.mutedForeground, size: 16),
        child: child,
      ),
    );
  }
}
