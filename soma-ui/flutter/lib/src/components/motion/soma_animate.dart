import 'package:flutter/material.dart';

enum SomaAnimationType {
  fadeIn,
  slideUp,
  slideDown,
  slideLeft,
  slideRight,
  scaleIn,
  bounceIn,
  pulseSoft,
}

class SomaAnimate extends StatefulWidget {
  final SomaAnimationType animation;
  final Widget child;
  final Duration duration;

  const SomaAnimate({
    super.key,
    this.animation = SomaAnimationType.fadeIn,
    required this.child,
    this.duration = const Duration(milliseconds: 400),
  });

  @override
  State<SomaAnimate> createState() => _SomaAnimateState();
}

class _SomaAnimateState extends State<SomaAnimate>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final Animation<double> _opacity;
  late final Animation<Offset> _slide;
  late final Animation<double> _scale;

  @override
  void initState() {
    super.initState();

    final isPulse = widget.animation == SomaAnimationType.pulseSoft;
    final isBounce = widget.animation == SomaAnimationType.bounceIn;

    _controller = AnimationController(vsync: this, duration: widget.duration);

    _opacity = Tween<double>(begin: 0, end: 1).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeOut),
    );

    Offset slideBegin = Offset.zero;
    switch (widget.animation) {
      case SomaAnimationType.slideUp:
        slideBegin = const Offset(0, 0.12);
      case SomaAnimationType.slideDown:
        slideBegin = const Offset(0, -0.12);
      case SomaAnimationType.slideLeft:
        slideBegin = const Offset(0.12, 0);
      case SomaAnimationType.slideRight:
        slideBegin = const Offset(-0.12, 0);
      default:
        slideBegin = Offset.zero;
    }
    _slide = Tween<Offset>(begin: slideBegin, end: Offset.zero).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeOut),
    );

    final scaleCurve = isBounce ? Curves.elasticOut : Curves.easeOut;
    final scaleBegin = (widget.animation == SomaAnimationType.scaleIn ||
            widget.animation == SomaAnimationType.bounceIn)
        ? 0.9
        : 1.0;
    final scaleEnd = isPulse ? 1.03 : 1.0;

    _scale = Tween<double>(begin: scaleBegin, end: scaleEnd).animate(
      CurvedAnimation(parent: _controller, curve: scaleCurve),
    );

    if (isPulse) {
      _controller.repeat(reverse: true);
    } else {
      _controller.forward();
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    Widget child = widget.child;

    switch (widget.animation) {
      case SomaAnimationType.fadeIn:
        child = FadeTransition(opacity: _opacity, child: child);
      case SomaAnimationType.slideUp:
      case SomaAnimationType.slideDown:
      case SomaAnimationType.slideLeft:
      case SomaAnimationType.slideRight:
        child = FadeTransition(
          opacity: _opacity,
          child: SlideTransition(position: _slide, child: child),
        );
      case SomaAnimationType.scaleIn:
        child = FadeTransition(
          opacity: _opacity,
          child: ScaleTransition(scale: _scale, child: child),
        );
      case SomaAnimationType.bounceIn:
        child = ScaleTransition(scale: _scale, child: child);
      case SomaAnimationType.pulseSoft:
        child = ScaleTransition(scale: _scale, child: child);
    }

    return child;
  }
}
