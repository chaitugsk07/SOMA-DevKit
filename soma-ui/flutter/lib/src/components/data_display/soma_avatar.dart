import 'package:flutter/material.dart';
import '../../theme/soma_colors.dart';
import '../../theme/soma_theme.dart';

enum SomaAvatarSize { sm, md, lg }

class SomaAvatar extends StatelessWidget {
  final String? imageUrl;
  final String? initials;
  final SomaAvatarSize size;

  const SomaAvatar({
    super.key,
    this.imageUrl,
    this.initials,
    this.size = SomaAvatarSize.md,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    final double diameter;
    final double fontSize;
    switch (size) {
      case SomaAvatarSize.sm:
        diameter = 32;
        fontSize = 12;
      case SomaAvatarSize.md:
        diameter = 40;
        fontSize = 14;
      case SomaAvatarSize.lg:
        diameter = 48;
        fontSize = 16;
    }

    final innerDiameter = diameter - 3;

    Widget inner;
    if (imageUrl != null) {
      inner = ClipOval(
        child: Image.network(
          imageUrl!,
          width: innerDiameter,
          height: innerDiameter,
          fit: BoxFit.cover,
          errorBuilder: (_, __, ___) => _fallback(c, innerDiameter, fontSize),
        ),
      );
    } else {
      inner = _fallback(c, innerDiameter, fontSize);
    }

    return Container(
      width: diameter,
      height: diameter,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        border: Border.all(color: c.border, width: 2),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(28), blurRadius: 10, offset: const Offset(0, 3)),
          BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 3, offset: const Offset(0, 1)),
        ],
      ),
      child: Center(child: inner),
    );
  }

  Widget _fallback(SomaColors c, double diameter, double fontSize) {
    return Container(
      width: diameter,
      height: diameter,
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: [
            Color.alphaBlend(Colors.white.withAlpha(18), c.muted),
            Color.alphaBlend(Colors.black.withAlpha(22), c.muted),
          ],
        ),
        shape: BoxShape.circle,
      ),
      alignment: Alignment.center,
      child: Text(
        initials ?? '',
        style: TextStyle(
          fontFamily: 'Rajdhani',
          fontSize: fontSize,
          fontWeight: FontWeight.w600,
          color: c.mutedForeground,
        ),
      ),
    );
  }
}
