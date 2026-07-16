import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '../../icons/soma_icons.dart';
import 'soma_input.dart';

class SomaComboboxOption {
  final String value;
  final String label;
  const SomaComboboxOption({required this.value, required this.label});
}

class SomaCombobox extends StatefulWidget {
  final List<SomaComboboxOption> options;
  final String? value;
  final ValueChanged<String>? onChanged;
  final String placeholder;

  const SomaCombobox({
    super.key,
    required this.options,
    this.value,
    this.onChanged,
    this.placeholder = '',
  });

  @override
  State<SomaCombobox> createState() => _SomaComboboxState();
}

class _SomaComboboxState extends State<SomaCombobox> {
  bool _open = false;
  bool _hovered = false;
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlay;
  final _searchCtrl = TextEditingController();

  String get _selectedLabel {
    if (widget.value == null || widget.value!.isEmpty) return '';
    return widget.options
        .where((o) => o.value == widget.value)
        .map((o) => o.label)
        .firstOrNull ??
        widget.value!;
  }

  void _openMenu(BuildContext context) {
    final c = SomaTheme.of(context);
    _searchCtrl.clear();
    _overlay = OverlayEntry(builder: (_) => _buildOverlay(context, c));
    Overlay.of(context).insert(_overlay!);
    setState(() => _open = true);
  }

  void _closeMenu() {
    _overlay?.remove();
    _overlay = null;
    if (mounted) setState(() => _open = false);
  }

  Widget _buildOverlay(BuildContext outerContext, SomaColors c) {
    return Stack(
      children: [
        Positioned.fill(
          child: GestureDetector(
            onTap: _closeMenu,
            behavior: HitTestBehavior.translucent,
            child: const SizedBox.expand(),
          ),
        ),
        CompositedTransformFollower(
          link: _layerLink,
          showWhenUnlinked: false,
          offset: const Offset(0, 44),
          child: Material(
            color: Colors.transparent,
            child: StatefulBuilder(
              builder: (_, setInnerState) {
                final query = _searchCtrl.text.toLowerCase();
                final filtered = widget.options
                    .where((o) => o.label.toLowerCase().contains(query))
                    .toList();

                return Container(
                  constraints: const BoxConstraints(maxHeight: 280),
                  decoration: BoxDecoration(
                    color: c.card,
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(color: c.border),
                    boxShadow: [
                      BoxShadow(color: Colors.black.withAlpha(8), blurRadius: 4, offset: const Offset(0, 1)),
                      BoxShadow(color: Colors.black.withAlpha(25), blurRadius: 16, offset: const Offset(0, 8)),
                    ],
                  ),
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(6),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        // Search row
                        Padding(
                          padding: const EdgeInsets.all(8),
                          child: SomaInput(
                            controller: _searchCtrl,
                            placeholder: 'Search…',
                            onChanged: (_) => setInnerState(() {}),
                          ),
                        ),
                        Divider(height: 1, color: c.border),
                        // Options
                        Flexible(
                          child: SingleChildScrollView(
                            padding: const EdgeInsets.all(4),
                            child: filtered.isEmpty
                                ? Padding(
                                    padding: const EdgeInsets.all(8),
                                    child: Text(
                                      'No results',
                                      style: TextStyle(
                                        fontFamily: 'Outfit',
                                        fontSize: 14,
                                        color: c.mutedForeground,
                                      ),
                                    ),
                                  )
                                : Column(
                                    mainAxisSize: MainAxisSize.min,
                                    crossAxisAlignment:
                                        CrossAxisAlignment.stretch,
                                    children: filtered.map((opt) {
                                      final selected =
                                          opt.value == widget.value;
                                      return _ComboboxRow(
                                        label: opt.label,
                                        selected: selected,
                                        colors: c,
                                        onTap: () {
                                          _closeMenu();
                                          widget.onChanged?.call(opt.value);
                                        },
                                      );
                                    }).toList(),
                                  ),
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _overlay?.remove();
    _searchCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final label = _selectedLabel;
    final isPlaceholder = label.isEmpty;
    final displayText = isPlaceholder ? widget.placeholder : label;

    return CompositedTransformTarget(
      link: _layerLink,
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: () => _open ? _closeMenu() : _openMenu(context),
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            height: 40,
            padding: const EdgeInsets.symmetric(horizontal: 12),
            decoration: BoxDecoration(
              color: _hovered ? c.accent.withAlpha(30) : Colors.transparent,
              borderRadius: BorderRadius.circular(6),
              border: Border.all(
                color: _open ? c.ring : (_hovered ? c.ring.withAlpha(120) : c.input),
                width: _open ? 2 : 1,
              ),
              boxShadow: _open
                  ? [
                      BoxShadow(color: c.ring.withAlpha(50), blurRadius: 6, spreadRadius: 1),
                      BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
                    ]
                  : [
                      BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
                    ],
            ),
            child: Row(
              children: [
                Expanded(
                  child: Text(
                    displayText,
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      color: isPlaceholder ? c.mutedForeground : c.foreground,
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                AnimatedRotation(
                  turns: _open ? 0.5 : 0,
                  duration: const Duration(milliseconds: 150),
                  child: Icon(LucideIcons.chevronDown,
                      size: 18, color: c.mutedForeground),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _ComboboxRow extends StatefulWidget {
  final String label;
  final bool selected;
  final SomaColors colors;
  final VoidCallback onTap;

  const _ComboboxRow({
    required this.label,
    required this.selected,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_ComboboxRow> createState() => _ComboboxRowState();
}

class _ComboboxRowState extends State<_ComboboxRow> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
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
              Expanded(
                child: Text(
                  widget.label,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: _hovered ? c.accentForeground : c.foreground,
                  ),
                ),
              ),
              if (widget.selected)
                Icon(LucideIcons.check, size: 14, color: c.primary),
            ],
          ),
        ),
      ),
    );
  }
}
