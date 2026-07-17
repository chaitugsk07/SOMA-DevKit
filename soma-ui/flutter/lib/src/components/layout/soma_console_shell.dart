import 'package:flutter/material.dart';

import '../../icons/soma_icons.dart';
import '../../theme/soma_colors.dart';
import '../../theme/soma_theme.dart';

class SomaConsoleShell extends StatefulWidget {
  final String brand;
  final String? version;
  final Widget Function(VoidCallback closeNavigation) navigationBuilder;
  final Widget header;
  final Widget body;
  final Widget? sidebarFooter;
  final double breakpoint;
  final double sidebarWidth;

  const SomaConsoleShell({
    super.key,
    required this.brand,
    required this.navigationBuilder,
    required this.header,
    required this.body,
    this.version,
    this.sidebarFooter,
    this.breakpoint = 760,
    this.sidebarWidth = 240,
  });

  @override
  State<SomaConsoleShell> createState() => _SomaConsoleShellState();
}

class _SomaConsoleShellState extends State<SomaConsoleShell> {
  bool _navigationOpen = false;

  Widget _brandRow(SomaColors colors, {bool showClose = false}) {
    return Container(
      height: 56,
      padding: const EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: colors.border)),
      ),
      child: Row(
        children: [
          Icon(LucideIcons.hexagon, size: 20, color: colors.primary),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              widget.brand,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 16,
                fontWeight: FontWeight.w700,
                color: colors.foreground,
              ),
            ),
          ),
          if (widget.version != null)
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
              decoration: BoxDecoration(
                color: colors.primary.withAlpha(18),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Text(
                widget.version!,
                style: TextStyle(
                  fontFamily: 'Roboto Mono',
                  fontSize: 10,
                  color: colors.primary,
                ),
              ),
            ),
          if (showClose)
            IconButton(
              tooltip: 'Close navigation',
              onPressed: () => setState(() => _navigationOpen = false),
              icon: Icon(
                LucideIcons.x,
                size: 18,
                color: colors.mutedForeground,
              ),
            ),
        ],
      ),
    );
  }

  Widget _sidebar(SomaColors colors, {bool showClose = false}) {
    return Container(
      width: widget.sidebarWidth,
      color: colors.card,
      child: Column(
        children: [
          _brandRow(colors, showClose: showClose),
          Expanded(
            child: widget.navigationBuilder(
              () => setState(() => _navigationOpen = false),
            ),
          ),
          if (widget.sidebarFooter != null) widget.sidebarFooter!,
        ],
      ),
    );
  }

  Widget _topbar(SomaColors colors, {required bool showMenu}) {
    return Container(
      height: 56,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      decoration: BoxDecoration(
        color: colors.background,
        border: Border(bottom: BorderSide(color: colors.border)),
      ),
      child: Row(
        children: [
          if (showMenu) ...[
            IconButton(
              tooltip: 'Open navigation',
              onPressed: () => setState(() => _navigationOpen = true),
              icon: Icon(
                LucideIcons.menu,
                size: 18,
                color: colors.mutedForeground,
              ),
            ),
            const SizedBox(width: 4),
          ],
          Expanded(child: widget.header),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final colors = SomaTheme.of(context);

    return LayoutBuilder(
      builder: (context, constraints) {
        final isWide = constraints.maxWidth >= widget.breakpoint;
        if (isWide) {
          return ColoredBox(
            color: colors.background,
            child: Row(
              children: [
                DecoratedBox(
                  decoration: BoxDecoration(
                    border: Border(
                      right: BorderSide(color: colors.border),
                    ),
                  ),
                  child: _sidebar(colors),
                ),
                Expanded(
                  child: Column(
                    children: [
                      _topbar(colors, showMenu: false),
                      Expanded(child: widget.body),
                    ],
                  ),
                ),
              ],
            ),
          );
        }

        return ColoredBox(
          color: colors.background,
          child: Stack(
            children: [
              Column(
                children: [
                  _topbar(colors, showMenu: true),
                  Expanded(child: widget.body),
                ],
              ),
              if (_navigationOpen) ...[
                Positioned.fill(
                  child: Semantics(
                    button: true,
                    label: 'Close navigation',
                    child: GestureDetector(
                      behavior: HitTestBehavior.opaque,
                      onTap: () => setState(() => _navigationOpen = false),
                      child: ColoredBox(color: Colors.black.withAlpha(140)),
                    ),
                  ),
                ),
                Align(
                  alignment: Alignment.centerLeft,
                  child: Material(
                    elevation: 16,
                    color: colors.card,
                    child: _sidebar(colors, showClose: true),
                  ),
                ),
              ],
            ],
          ),
        );
      },
    );
  }
}
