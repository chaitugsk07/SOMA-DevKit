import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

class SomaCommandItem {
  final String label;
  final String keywords;
  final VoidCallback? onSelect;
  final IconData? icon;

  const SomaCommandItem({
    required this.label,
    this.keywords = '',
    this.onSelect,
    this.icon,
  });
}

class SomaCommandGroup {
  final String heading;
  final List<SomaCommandItem> items;

  const SomaCommandGroup({required this.heading, required this.items});
}

Future<void> showSomaCommand(
  BuildContext context, {
  required List<SomaCommandGroup> groups,
}) {
  return showGeneralDialog(
    context: context,
    barrierDismissible: true,
    barrierLabel: 'Close',
    barrierColor: Colors.black.withAlpha(140),
    transitionDuration: const Duration(milliseconds: 150),
    pageBuilder: (ctx, _, __) => _CommandDialog(groups: groups),
    transitionBuilder: (ctx, anim, _, child) => FadeTransition(
      opacity: CurvedAnimation(parent: anim, curve: Curves.easeOut),
      child: ScaleTransition(
        scale: Tween(begin: 0.95, end: 1.0).animate(
          CurvedAnimation(parent: anim, curve: Curves.easeOut),
        ),
        child: child,
      ),
    ),
  );
}

class _CommandDialog extends StatefulWidget {
  final List<SomaCommandGroup> groups;

  const _CommandDialog({required this.groups});

  @override
  State<_CommandDialog> createState() => _CommandDialogState();
}

class _CommandDialogState extends State<_CommandDialog> {
  final TextEditingController _ctrl = TextEditingController();
  String _filter = '';

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  List<SomaCommandGroup> get _filtered {
    if (_filter.isEmpty) return widget.groups;
    final q = _filter.toLowerCase();
    return widget.groups
        .map((g) => SomaCommandGroup(
              heading: g.heading,
              items: g.items
                  .where((i) =>
                      i.label.toLowerCase().contains(q) ||
                      i.keywords.toLowerCase().contains(q))
                  .toList(),
            ))
        .where((g) => g.items.isNotEmpty)
        .toList();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final filtered = _filtered;
    final screenWidth = MediaQuery.of(context).size.width;
    final maxW = min(screenWidth * 0.9, 640.0);

    return Focus(
      autofocus: true,
      onKeyEvent: (node, e) {
        if (e is KeyDownEvent &&
            e.logicalKey == LogicalKeyboardKey.escape) {
          Navigator.of(context).pop();
          return KeyEventResult.handled;
        }
        return KeyEventResult.ignored;
      },
      child: Align(
        alignment: const Alignment(0, -0.3),
        child: ConstrainedBox(
          constraints: BoxConstraints(maxWidth: maxW),
          child: Material(
            color: Colors.transparent,
            child: Container(
              decoration: BoxDecoration(
                color: c.card,
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: c.border),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withAlpha(20),
                    blurRadius: 8,
                  ),
                  BoxShadow(
                    color: Colors.black.withAlpha(40),
                    blurRadius: 48,
                    offset: const Offset(0, 24),
                  ),
                ],
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  // Search bar
                  Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 12,
                      vertical: 8,
                    ),
                    decoration: BoxDecoration(
                      border: Border(
                        bottom: BorderSide(color: c.border),
                      ),
                    ),
                    child: Row(
                      children: [
                        Icon(
                          LucideIcons.search,
                          size: 16,
                          color: c.mutedForeground,
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: TextField(
                            controller: _ctrl,
                            autofocus: true,
                            style: TextStyle(
                              fontFamily: 'Outfit',
                              fontSize: 14,
                              color: c.foreground,
                            ),
                            decoration: InputDecoration(
                              hintText: 'Type a command or search…',
                              hintStyle: TextStyle(color: c.mutedForeground),
                              border: InputBorder.none,
                              isDense: true,
                              contentPadding: EdgeInsets.zero,
                            ),
                            cursorColor: c.ring,
                            onChanged: (v) => setState(() => _filter = v),
                          ),
                        ),
                      ],
                    ),
                  ),
                  // Results
                  ConstrainedBox(
                    constraints: const BoxConstraints(maxHeight: 320),
                    child: SingleChildScrollView(
                      padding: const EdgeInsets.all(4),
                      child: filtered.isEmpty
                          ? Padding(
                              padding: const EdgeInsets.all(16),
                              child: Text(
                                'No results found.',
                                style: TextStyle(
                                  fontFamily: 'Outfit',
                                  fontSize: 14,
                                  color: c.mutedForeground,
                                ),
                                textAlign: TextAlign.center,
                              ),
                            )
                          : Column(
                              crossAxisAlignment: CrossAxisAlignment.stretch,
                              children: filtered.map((group) {
                                return Column(
                                  crossAxisAlignment:
                                      CrossAxisAlignment.stretch,
                                  children: [
                                    Padding(
                                      padding: const EdgeInsets.symmetric(
                                        horizontal: 8,
                                        vertical: 4,
                                      ),
                                      child: Text(
                                        group.heading.toUpperCase(),
                                        style: TextStyle(
                                          fontFamily: 'Outfit',
                                          fontSize: 11,
                                          fontWeight: FontWeight.w600,
                                          color: c.mutedForeground,
                                          letterSpacing: 0.5,
                                        ),
                                      ),
                                    ),
                                    ...group.items.map(
                                      (item) => _CommandItemRow(
                                        item: item,
                                        onTap: () {
                                          Navigator.of(context).pop();
                                          item.onSelect?.call();
                                        },
                                      ),
                                    ),
                                  ],
                                );
                              }).toList(),
                            ),
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

class _CommandItemRow extends StatefulWidget {
  final SomaCommandItem item;
  final VoidCallback onTap;

  const _CommandItemRow({required this.item, required this.onTap});

  @override
  State<_CommandItemRow> createState() => _CommandItemRowState();
}

class _CommandItemRowState extends State<_CommandItemRow> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 100),
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
          decoration: BoxDecoration(
            color: _hovered ? c.accent : Colors.transparent,
            borderRadius: BorderRadius.circular(4),
          ),
          child: Row(
            children: [
              if (widget.item.icon != null) ...[
                Icon(
                  widget.item.icon,
                  size: 16,
                  color: _hovered ? c.accentForeground : c.foreground,
                ),
                const SizedBox(width: 10),
              ],
              Expanded(
                child: Text(
                  widget.item.label,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: _hovered ? c.accentForeground : c.foreground,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
