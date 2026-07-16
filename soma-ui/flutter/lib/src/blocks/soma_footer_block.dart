import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaFooterLinkColumn {
  final String heading;
  final List<String> links;
  const SomaFooterLinkColumn({required this.heading, required this.links});
}

class SomaFooterBlock extends StatelessWidget {
  final String brandName;
  final String tagline;
  final List<SomaFooterLinkColumn> columns;
  final String copyright;
  final VoidCallback? onGithubTap;
  final VoidCallback? onTwitterTap;

  const SomaFooterBlock({
    super.key,
    this.brandName = 'Soma UI',
    this.tagline = 'Composable Flutter components with a Palantir-inspired design system.',
    this.columns = const [],
    this.copyright = '© 2025 Soma UI. All rights reserved.',
    this.onGithubTap,
    this.onTwitterTap,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Container(
      decoration: BoxDecoration(
        color: c.card,
        border: Border(top: BorderSide(color: c.border)),
      ),
      padding: const EdgeInsets.symmetric(vertical: 48, horizontal: 24),
      child: LayoutBuilder(
        builder: (context, constraints) {
          final isWide = constraints.maxWidth >= 640;
          Widget columnsSection;
          if (isWide) {
            columnsSection = Wrap(
              spacing: 32,
              runSpacing: 32,
              children: [
                _BrandColumn(
                  brandName: brandName,
                  tagline: tagline,
                ),
                for (final col in columns) _LinkColumn(column: col),
              ],
            );
          } else {
            columnsSection = Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _BrandColumn(brandName: brandName, tagline: tagline),
                const SizedBox(height: 32),
                for (int i = 0; i < columns.length; i++) ...[
                  _LinkColumn(column: columns[i]),
                  if (i < columns.length - 1) const SizedBox(height: 32),
                ],
              ],
            );
          }

          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              columnsSection,
              const SizedBox(height: 32),
              Divider(color: c.border),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: Text(
                      copyright,
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 13,
                        color: c.mutedForeground,
                      ),
                    ),
                  ),
                  SomaButton(
                    variant: SomaButtonVariant.ghost,
                    size: SomaButtonSize.icon,
                    onPressed: onGithubTap ?? () {},
                    child: const Icon(LucideIcons.externalLink, size: 18),
                  ),
                  const SizedBox(width: 4),
                  SomaButton(
                    variant: SomaButtonVariant.ghost,
                    size: SomaButtonSize.icon,
                    onPressed: onTwitterTap ?? () {},
                    child: const Icon(LucideIcons.share, size: 18),
                  ),
                ],
              ),
            ],
          );
        },
      ),
    );
  }
}

class _BrandColumn extends StatelessWidget {
  final String brandName;
  final String tagline;
  const _BrandColumn({required this.brandName, required this.tagline});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ConstrainedBox(
      constraints: const BoxConstraints(minWidth: 140, maxWidth: 200),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            brandName,
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 18,
              fontWeight: FontWeight.w700,
              color: c.foreground,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            tagline,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
              height: 1.5,
            ),
          ),
        ],
      ),
    );
  }
}

class _LinkColumn extends StatelessWidget {
  final SomaFooterLinkColumn column;
  const _LinkColumn({required this.column});

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return ConstrainedBox(
      constraints: const BoxConstraints(minWidth: 140, maxWidth: 180),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            column.heading,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              fontWeight: FontWeight.w600,
              color: c.foreground,
            ),
          ),
          const SizedBox(height: 12),
          for (final link in column.links) ...[
            _FooterLink(label: link),
            const SizedBox(height: 8),
          ],
        ],
      ),
    );
  }
}

class _FooterLink extends StatefulWidget {
  final String label;
  const _FooterLink({required this.label});

  @override
  State<_FooterLink> createState() => _FooterLinkState();
}

class _FooterLinkState extends State<_FooterLink> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      cursor: SystemMouseCursors.click,
      child: Text(
        widget.label,
        style: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 13,
          color: _hovered ? c.foreground : c.mutedForeground,
        ),
      ),
    );
  }
}
