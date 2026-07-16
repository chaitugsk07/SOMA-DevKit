import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaSidenavItem {
  final IconData icon;
  final String label;
  final String id;
  const SomaSidenavItem({
    required this.icon,
    required this.label,
    required this.id,
  });
}

class SomaSidenavBlock extends StatefulWidget {
  final List<SomaSidenavItem> items;
  final String? activeId;
  final void Function(String id)? onItemTap;
  final String brandName;
  final String userName;
  final String userEmail;
  final String userInitials;

  const SomaSidenavBlock({
    super.key,
    this.items = const [],
    this.activeId,
    this.onItemTap,
    this.brandName = 'Soma UI',
    this.userName = 'Jane Doe',
    this.userEmail = 'jane@example.com',
    this.userInitials = 'JD',
  });

  @override
  State<SomaSidenavBlock> createState() => _SomaSidenavBlockState();
}

class _SomaSidenavBlockState extends State<SomaSidenavBlock> {
  String? _internalActiveId;
  String? _hoveredId;

  @override
  void initState() {
    super.initState();
    if (widget.activeId == null && widget.items.isNotEmpty) {
      _internalActiveId = widget.items.first.id;
    }
  }

  String? get _effectiveActiveId => widget.activeId ?? _internalActiveId;

  void _onTap(String id) {
    if (widget.activeId == null) {
      setState(() => _internalActiveId = id);
    }
    widget.onItemTap?.call(id);
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ConstrainedBox(
      constraints: const BoxConstraints(minHeight: 600, maxWidth: 256, minWidth: 256),
      child: Container(
        decoration: BoxDecoration(
          color: c.card,
          border: Border(right: BorderSide(color: c.border)),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Brand header
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
              decoration: BoxDecoration(
                border: Border(bottom: BorderSide(color: c.border)),
              ),
              child: Text(
                widget.brandName,
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 18,
                  fontWeight: FontWeight.w700,
                  color: c.foreground,
                ),
              ),
            ),
            // Nav list
            Expanded(
              child: SingleChildScrollView(
                padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 8),
                child: Column(
                  children: widget.items.map((item) {
                    final isActive = item.id == _effectiveActiveId;
                    final isHovered = item.id == _hoveredId;
                    return MouseRegion(
                      onEnter: (_) => setState(() => _hoveredId = item.id),
                      onExit: (_) => setState(() => _hoveredId = null),
                      cursor: SystemMouseCursors.click,
                      child: GestureDetector(
                        onTap: () => _onTap(item.id),
                        child: AnimatedContainer(
                          duration: const Duration(milliseconds: 120),
                          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 9),
                          margin: const EdgeInsets.only(bottom: 2),
                          decoration: BoxDecoration(
                            color: isActive
                                ? c.accent
                                : isHovered
                                    ? c.accent.withAlpha(120)
                                    : Colors.transparent,
                            borderRadius: BorderRadius.circular(6),
                          ),
                          child: Row(
                            children: [
                              Icon(
                                item.icon,
                                size: 16,
                                color: isActive ? c.foreground : c.mutedForeground,
                              ),
                              const SizedBox(width: 10),
                              Text(
                                item.label,
                                style: TextStyle(
                                  fontFamily: 'Outfit',
                                  fontSize: 14,
                                  fontWeight: isActive ? FontWeight.w500 : FontWeight.w400,
                                  color: isActive ? c.foreground : c.mutedForeground,
                                ),
                              ),
                            ],
                          ),
                        ),
                      ),
                    );
                  }).toList(),
                ),
              ),
            ),
            // User footer
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              decoration: BoxDecoration(
                border: Border(top: BorderSide(color: c.border)),
              ),
              child: Row(
                children: [
                  SomaAvatar(initials: widget.userInitials, size: SomaAvatarSize.sm),
                  const SizedBox(width: 10),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Text(
                          widget.userName,
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 13,
                            fontWeight: FontWeight.w500,
                            color: c.foreground,
                          ),
                          overflow: TextOverflow.ellipsis,
                        ),
                        Text(
                          widget.userEmail,
                          style: TextStyle(
                            fontFamily: 'Outfit',
                            fontSize: 11,
                            color: c.mutedForeground,
                          ),
                          overflow: TextOverflow.ellipsis,
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
