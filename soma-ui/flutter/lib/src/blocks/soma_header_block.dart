import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaHeaderBlock extends StatelessWidget {
  final String brandName;
  final List<String> navLinks;
  final String userInitials;
  final VoidCallback? onNotificationTap;

  const SomaHeaderBlock({
    super.key,
    this.brandName = 'Soma UI',
    this.navLinks = const [],
    this.userInitials = 'JD',
    this.onNotificationTap,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      height: 64,
      padding: const EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        color: c.card,
        border: Border(bottom: BorderSide(color: c.border)),
      ),
      child: LayoutBuilder(
        builder: (context, constraints) {
          final isWide = constraints.maxWidth >= 640;
          return Row(
            children: [
              // Brand
              Text(
                brandName,
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 18,
                  fontWeight: FontWeight.w700,
                  color: c.foreground,
                ),
              ),
              // Nav links (wide only)
              if (isWide) ...[
                const SizedBox(width: 24),
                for (final link in navLinks) ...[
                  _NavLink(label: link),
                  const SizedBox(width: 4),
                ],
              ],
              const Spacer(),
              // Search (wide only)
              if (isWide) ...[
                SizedBox(
                  width: 192,
                  child: SomaInput(placeholder: 'Search…'),
                ),
                const SizedBox(width: 8),
              ],
              // Notification button
              SomaButton(
                variant: SomaButtonVariant.ghost,
                size: SomaButtonSize.icon,
                onPressed: onNotificationTap ?? () {},
                child: const Icon(LucideIcons.bell, size: 18),
              ),
              const SizedBox(width: 8),
              // Avatar
              SomaAvatar(initials: userInitials, size: SomaAvatarSize.sm),
            ],
          );
        },
      ),
    );
  }
}

class _NavLink extends StatefulWidget {
  final String label;
  const _NavLink({required this.label});

  @override
  State<_NavLink> createState() => _NavLinkState();
}

class _NavLinkState extends State<_NavLink> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      cursor: SystemMouseCursors.click,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
        child: Text(
          widget.label,
          style: TextStyle(
            fontFamily: 'Outfit',
            fontSize: 14,
            color: _hovered ? c.foreground : c.mutedForeground,
          ),
        ),
      ),
    );
  }
}
