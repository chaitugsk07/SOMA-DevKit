import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '../../icons/soma_icons.dart';

class SomaBreadcrumbItem {
  final String label;
  final VoidCallback? onTap;
  final bool isCurrent;

  const SomaBreadcrumbItem({
    required this.label,
    this.onTap,
    this.isCurrent = false,
  });
}

class SomaBreadcrumb extends StatelessWidget {
  final List<SomaBreadcrumbItem> items;

  const SomaBreadcrumb({super.key, required this.items});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final children = <Widget>[];

    for (int i = 0; i < items.length; i++) {
      if (i > 0) {
        children.add(Padding(
          padding: const EdgeInsets.symmetric(horizontal: 6),
          child: Icon(LucideIcons.chevronRight, size: 13, color: c.border),
        ));
      }
      children.add(_BreadcrumbLabel(item: items[i], colors: c));
    }

    return Row(mainAxisSize: MainAxisSize.min, children: children);
  }
}

class _BreadcrumbLabel extends StatefulWidget {
  final SomaBreadcrumbItem item;
  final SomaColors colors;

  const _BreadcrumbLabel({required this.item, required this.colors});

  @override
  State<_BreadcrumbLabel> createState() => _BreadcrumbLabelState();
}

class _BreadcrumbLabelState extends State<_BreadcrumbLabel> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    final item = widget.item;

    if (item.isCurrent) {
      return Text(
        item.label,
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 14,
          fontWeight: FontWeight.w500,
          color: c.foreground,
        ),
      );
    }

    return MouseRegion(
      cursor: item.onTap != null ? SystemMouseCursors.click : MouseCursor.defer,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: item.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 140),
          curve: Curves.easeOutCubic,
          padding: const EdgeInsets.symmetric(horizontal: 2, vertical: 1),
          decoration: BoxDecoration(
            color: _hovered ? c.accent.withAlpha(80) : Colors.transparent,
            borderRadius: BorderRadius.circular(3),
          ),
          child: AnimatedDefaultTextStyle(
            duration: const Duration(milliseconds: 140),
            curve: Curves.easeOutCubic,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: _hovered ? c.foreground : c.mutedForeground,
            ),
            child: Text(item.label),
          ),
        ),
      ),
    );
  }
}
