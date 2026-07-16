import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';

class SomaSwitch extends StatefulWidget {
  final bool value;
  final ValueChanged<bool>? onChanged;
  final bool enabled;

  const SomaSwitch({
    super.key,
    required this.value,
    this.onChanged,
    this.enabled = true,
  });

  @override
  State<SomaSwitch> createState() => _SomaSwitchState();
}

class _SomaSwitchState extends State<SomaSwitch> with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final Animation<double> _thumbPos;
  bool _hovered = false;
  bool _focused = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 150),
      value: widget.value ? 1.0 : 0.0,
    );
    _thumbPos = CurvedAnimation(parent: _controller, curve: Curves.easeOutCubic);
  }

  @override
  void didUpdateWidget(SomaSwitch oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.value != widget.value) {
      widget.value ? _controller.forward() : _controller.reverse();
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _handleTap() {
    if (widget.enabled && widget.onChanged != null) {
      widget.onChanged!(!widget.value);
    }
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    Color trackColor = widget.value ? c.primary : c.muted;
    if (_hovered && !widget.value) {
      trackColor = Color.alphaBlend(Colors.white.withAlpha(15), trackColor);
    }

    final List<BoxShadow> trackShadows = [
      if (_focused)
        BoxShadow(color: c.ring.withAlpha(60), blurRadius: 0, spreadRadius: 3),
      if (widget.value)
        BoxShadow(color: c.primary.withAlpha(60), blurRadius: 6, offset: const Offset(0, 1)),
    ];

    return Focus(
      onFocusChange: (focused) => setState(() => _focused = focused),
      child: MouseRegion(
        cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.forbidden,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: _handleTap,
          child: AnimatedOpacity(
            opacity: widget.enabled ? 1.0 : 0.5,
            duration: const Duration(milliseconds: 150),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 150),
              width: 44,
              height: 24,
              decoration: BoxDecoration(
                color: trackColor,
                borderRadius: BorderRadius.circular(12),
                border: widget.value ? null : Border.all(color: c.border, width: 1.5),
                boxShadow: trackShadows.isEmpty ? null : trackShadows,
              ),
              child: AnimatedBuilder(
                animation: _thumbPos,
                builder: (context, _) {
                  return Padding(
                    padding: const EdgeInsets.all(2),
                    child: Align(
                      alignment: Alignment((_thumbPos.value * 2) - 1, 0),
                      child: Container(
                        width: 20,
                        height: 20,
                        decoration: BoxDecoration(
                          color: Colors.white,
                          shape: BoxShape.circle,
                          boxShadow: [
                            BoxShadow(
                              color: Colors.black.withAlpha(35),
                              blurRadius: 4,
                              offset: const Offset(0, 1),
                            ),
                          ],
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),
          ),
        ),
      ),
    );
  }
}
