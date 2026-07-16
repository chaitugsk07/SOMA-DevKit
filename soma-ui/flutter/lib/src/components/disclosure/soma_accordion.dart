import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

class SomaAccordionItem {
  final String value;
  final String title;
  final Widget content;
  final bool openByDefault;

  const SomaAccordionItem({
    required this.value,
    required this.title,
    required this.content,
    this.openByDefault = false,
  });
}

class SomaAccordion extends StatefulWidget {
  final List<SomaAccordionItem> items;

  const SomaAccordion({super.key, required this.items});

  @override
  State<SomaAccordion> createState() => _SomaAccordionState();
}

class _SomaAccordionState extends State<SomaAccordion> {
  String? _openValue;

  @override
  void initState() {
    super.initState();
    // Last item with openByDefault wins (matches web: last one set wins).
    for (final item in widget.items) {
      if (item.openByDefault) _openValue = item.value;
    }
  }

  void _toggle(String value) {
    setState(() {
      _openValue = _openValue == value ? null : value;
    });
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (int i = 0; i < widget.items.length; i++) ...[
          _AccordionItemWidget(
            item: widget.items[i],
            isOpen: _openValue == widget.items[i].value,
            onTap: () => _toggle(widget.items[i].value),
            borderColor: c.border,
          ),
          if (i < widget.items.length - 1)
            Divider(height: 1, thickness: 1, color: c.border),
        ],
      ],
    );
  }
}

class _AccordionItemWidget extends StatefulWidget {
  final SomaAccordionItem item;
  final bool isOpen;
  final VoidCallback onTap;
  final Color borderColor;

  const _AccordionItemWidget({
    required this.item,
    required this.isOpen,
    required this.onTap,
    required this.borderColor,
  });

  @override
  State<_AccordionItemWidget> createState() => _AccordionItemWidgetState();
}

class _AccordionItemWidgetState extends State<_AccordionItemWidget> {
  bool _hovered = false;
  bool _focused = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    List<BoxShadow>? focusShadows;
    if (_focused) {
      focusShadows = [
        BoxShadow(color: c.ring.withAlpha(50), blurRadius: 0, spreadRadius: 2),
      ];
    }

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Focus(
          onFocusChange: (f) => setState(() => _focused = f),
          child: MouseRegion(
            cursor: SystemMouseCursors.click,
            onEnter: (_) => setState(() => _hovered = true),
            onExit: (_) => setState(() => _hovered = false),
            child: GestureDetector(
              onTap: widget.onTap,
              behavior: HitTestBehavior.opaque,
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 140),
                curve: Curves.easeOutCubic,
                decoration: BoxDecoration(
                  color: _hovered ? c.accent.withAlpha(80) : Colors.transparent,
                  borderRadius: BorderRadius.circular(4),
                  boxShadow: focusShadows,
                ),
                padding: const EdgeInsets.symmetric(vertical: 16, horizontal: 2),
                child: Row(
                  children: [
                    Expanded(
                      child: Text(
                        widget.item.title,
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 14,
                          fontWeight: FontWeight.w500,
                          color: c.foreground,
                        ),
                      ),
                    ),
                    AnimatedRotation(
                      turns: widget.isOpen ? 0.5 : 0.0,
                      duration: const Duration(milliseconds: 160),
                      curve: Curves.easeOutCubic,
                      child: Icon(
                        LucideIcons.chevronDown,
                        size: 16,
                        color: c.mutedForeground,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
        AnimatedSize(
          duration: const Duration(milliseconds: 160),
          curve: Curves.easeOutCubic,
          alignment: Alignment.topCenter,
          child: widget.isOpen
              ? Padding(
                  padding: const EdgeInsets.only(bottom: 16),
                  child: Align(
                    alignment: Alignment.topLeft,
                    child: DefaultTextStyle(
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: c.mutedForeground,
                      ),
                      child: widget.item.content,
                    ),
                  ),
                )
              : const SizedBox.shrink(),
        ),
      ],
    );
  }
}
