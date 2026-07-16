import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';

// ponytail: curated 12-country list matches the web crate's COUNTRIES constant.
// Ceiling: no flag icons, no libphonenumber formatting, no search.
// Upgrade: add country search combobox + flag emoji map when needed.

class _Country {
  final String dial;
  final String label;
  const _Country(this.dial, this.label);
}

const _kCountries = [
  _Country('+1',   'US +1'),
  _Country('+44',  'UK +44'),
  _Country('+91',  'IN +91'),
  _Country('+61',  'AU +61'),
  _Country('+49',  'DE +49'),
  _Country('+33',  'FR +33'),
  _Country('+81',  'JP +81'),
  _Country('+86',  'CN +86'),
  _Country('+971', 'AE +971'),
  _Country('+966', 'SA +966'),
  _Country('+880', 'BD +880'),
  _Country('+92',  'PK +92'),
];

class SomaInputPhone extends StatefulWidget {
  final String? value;
  final ValueChanged<String>? onChanged;
  final String placeholder;

  const SomaInputPhone({
    super.key,
    this.value,
    this.onChanged,
    this.placeholder = 'Phone number',
  });

  @override
  State<SomaInputPhone> createState() => _SomaInputPhoneState();
}

class _SomaInputPhoneState extends State<SomaInputPhone> {
  String _dial = '+1';
  late final TextEditingController _ctrl;
  final FocusNode _focus = FocusNode();
  bool _focused = false;
  bool _dialHovered = false;
  bool _dialOpen = false;
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlay;

  @override
  void initState() {
    super.initState();
    _ctrl = TextEditingController();
    _focus.addListener(() => setState(() => _focused = _focus.hasFocus));
  }

  @override
  void dispose() {
    _overlay?.remove();
    _ctrl.dispose();
    _focus.dispose();
    super.dispose();
  }

  void _emit() {
    final num = _ctrl.text;
    widget.onChanged?.call(num.isEmpty ? _dial : '$_dial $num');
  }

  void _openDial(BuildContext context) {
    final c = SomaTheme.of(context);
    _overlay = OverlayEntry(builder: (_) => _buildDialOverlay(context, c));
    Overlay.of(context).insert(_overlay!);
    setState(() => _dialOpen = true);
  }

  void _closeDial() {
    _overlay?.remove();
    _overlay = null;
    if (mounted) setState(() => _dialOpen = false);
  }

  Widget _buildDialOverlay(BuildContext outerCtx, dynamic c) {
    return Stack(
      children: [
        Positioned.fill(
          child: GestureDetector(
            onTap: _closeDial,
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
            child: Container(
              width: 144,
              constraints: const BoxConstraints(maxHeight: 240),
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
                child: SingleChildScrollView(
                  padding: const EdgeInsets.all(4),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: _kCountries.map((country) {
                      return _DialRow(
                        country: country,
                        selected: country.dial == _dial,
                        colors: c,
                        onTap: () {
                          setState(() => _dial = country.dial);
                          _closeDial();
                          _emit();
                        },
                      );
                    }).toList(),
                  ),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final selectedLabel = _kCountries.firstWhere((e) => e.dial == _dial).label;

    return Row(
      children: [
        CompositedTransformTarget(
          link: _layerLink,
          child: MouseRegion(
            cursor: SystemMouseCursors.click,
            onEnter: (_) => setState(() => _dialHovered = true),
            onExit: (_) => setState(() => _dialHovered = false),
            child: GestureDetector(
              onTap: () => _dialOpen ? _closeDial() : _openDial(context),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 150),
                height: 40,
                width: 108,
                padding: const EdgeInsets.symmetric(horizontal: 10),
                decoration: BoxDecoration(
                  color: _dialHovered ? c.accent.withAlpha(30) : Colors.transparent,
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(
                    color: _dialOpen ? c.ring : (_dialHovered ? c.ring.withAlpha(120) : c.input),
                    width: _dialOpen ? 2 : 1,
                  ),
                  boxShadow: _dialOpen
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
                        selectedLabel,
                        style: TextStyle(
                          fontFamily: 'Outfit',
                          fontSize: 13,
                          color: c.foreground,
                        ),
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    AnimatedRotation(
                      turns: _dialOpen ? 0.5 : 0,
                      duration: const Duration(milliseconds: 150),
                      child: Icon(LucideIcons.chevronDown, size: 16, color: c.mutedForeground),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
        const SizedBox(width: 8),
        Expanded(
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            height: 40,
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
                  : [
                      BoxShadow(color: Colors.black.withAlpha(12), blurRadius: 4, offset: const Offset(0, 1)),
                    ],
            ),
            child: TextField(
              controller: _ctrl,
              focusNode: _focus,
              keyboardType: TextInputType.phone,
              inputFormatters: [FilteringTextInputFormatter.digitsOnly],
              onChanged: (_) => _emit(),
              style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.foreground),
              decoration: InputDecoration(
                hintText: widget.placeholder,
                hintStyle: TextStyle(color: c.mutedForeground),
                border: InputBorder.none,
                contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                isDense: true,
              ),
              cursorColor: c.ring,
            ),
          ),
        ),
      ],
    );
  }
}

class _DialRow extends StatefulWidget {
  final _Country country;
  final bool selected;
  final dynamic colors;
  final VoidCallback onTap;

  const _DialRow({
    required this.country,
    required this.selected,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_DialRow> createState() => _DialRowState();
}

class _DialRowState extends State<_DialRow> {
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
                  widget.country.label,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 13,
                    color: _hovered ? c.accentForeground : c.foreground,
                  ),
                ),
              ),
              if (widget.selected)
                Icon(LucideIcons.check, size: 13, color: c.primary),
            ],
          ),
        ),
      ),
    );
  }
}
