import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaDashboardScreen extends StatefulWidget {
  const SomaDashboardScreen({super.key});
  @override
  State<SomaDashboardScreen> createState() => _SomaDashboardScreenState();
}

class _SomaDashboardScreenState extends State<SomaDashboardScreen> {
  int _activeNav = 0;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return LayoutBuilder(builder: (context, constraints) {
      final isWide = constraints.maxWidth >= 720;
      final mainContent = _MainContent(c: c);
      if (isWide) {
        return Row(children: [
          _Sidebar(c: c, activeNav: _activeNav, onNavTap: (i) => setState(() => _activeNav = i)),
          Expanded(child: Column(children: [
            _TopBar(c: c),
            Expanded(child: mainContent),
          ])),
        ]);
      }
      return Column(children: [
        _TopBar(c: c, showHamburger: true),
        Expanded(child: mainContent),
      ]);
    });
  }
}

class _Sidebar extends StatelessWidget {
  final SomaColors c;
  final int activeNav;
  final ValueChanged<int> onNavTap;

  static const _navItems = [
    ('Overview', LucideIcons.layoutDashboard),
    ('Pipelines', LucideIcons.gitBranch),
    ('Datasets', LucideIcons.database),
    ('Reports', LucideIcons.fileText),
    ('Alerts', LucideIcons.bell),
    ('Audit Log', LucideIcons.clipboardList),
  ];

  const _Sidebar({required this.c, required this.activeNav, required this.onNavTap});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 220,
      decoration: BoxDecoration(color: c.card, border: Border(right: BorderSide(color: c.border))),
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(children: [
              Icon(LucideIcons.hexagon, size: 20, color: c.primary),
              const SizedBox(width: 8),
              Text('FOUNDRY', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 18, fontWeight: FontWeight.w700, color: c.foreground)),
            ]),
          ),
          Expanded(
            child: ListView.builder(
              padding: EdgeInsets.zero,
              itemCount: _navItems.length,
              itemBuilder: (context, i) {
                final (label, icon) = _navItems[i];
                final isActive = i == activeNav;
                return GestureDetector(
                  onTap: () => onNavTap(i),
                  child: Container(
                    decoration: BoxDecoration(
                      color: isActive ? c.primary.withAlpha(20) : Colors.transparent,
                      border: Border(left: BorderSide(color: isActive ? c.primary : Colors.transparent, width: 3)),
                    ),
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
                    child: Row(children: [
                      Icon(icon, size: 16, color: isActive ? c.primary : c.mutedForeground),
                      const SizedBox(width: 10),
                      Text(label, style: TextStyle(fontFamily: 'Outfit', fontSize: 13, color: isActive ? c.primary : c.mutedForeground)),
                    ]),
                  ),
                );
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(children: [
              SomaAvatar(initials: 'JD', size: SomaAvatarSize.sm),
              const SizedBox(width: 10),
              Text('John Doe', style: TextStyle(fontFamily: 'Outfit', fontSize: 13, color: c.foreground)),
            ]),
          ),
        ],
      ),
    );
  }
}

class _TopBar extends StatelessWidget {
  final SomaColors c;
  final bool showHamburger;
  const _TopBar({required this.c, this.showHamburger = false});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 56,
      decoration: BoxDecoration(color: c.card, border: Border(bottom: BorderSide(color: c.border))),
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(children: [
        if (showHamburger) ...[
          Icon(LucideIcons.menu, size: 18, color: c.mutedForeground),
          const SizedBox(width: 12),
        ],
        Text('Overview', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 18, fontWeight: FontWeight.w700, color: c.foreground)),
        const Spacer(),
        IconButton(icon: Icon(LucideIcons.search, size: 18, color: c.mutedForeground), onPressed: null),
        IconButton(icon: Icon(LucideIcons.bell, size: 18, color: c.mutedForeground), onPressed: null),
        const SizedBox(width: 4),
        SomaAvatar(initials: 'JD', size: SomaAvatarSize.sm),
      ]),
    );
  }
}

class _MainContent extends StatelessWidget {
  final SomaColors c;
  const _MainContent({required this.c});

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(20),
      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Text('Dashboard', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 24, fontWeight: FontWeight.w700, color: c.foreground)),
        const SizedBox(height: 4),
        Text('Good morning, John', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground)),
        const SizedBox(height: 20),
        Wrap(spacing: 12, runSpacing: 12, children: [
          _KpiCard(c: c, label: 'ACTIVE PIPELINES', value: '142', badge: '+12%', badgeVariant: SomaBadgeVariant.success),
          _KpiCard(c: c, label: 'DATA PROCESSED', value: '2.4 TB', badge: '+8%', badgeVariant: SomaBadgeVariant.success),
          _KpiCard(c: c, label: 'QUERY LATENCY', value: '45ms', badge: '-3%', badgeVariant: SomaBadgeVariant.destructive),
          _KpiCard(c: c, label: 'ERROR RATE', value: '0.2%', badge: '-0.1%', badgeVariant: SomaBadgeVariant.success),
        ]),
        const SizedBox(height: 20),
        SomaCard(
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Row(children: [
                Text('Pipeline Activity', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, fontWeight: FontWeight.w600, color: c.foreground)),
                const SizedBox(width: 8),
                Text('Last 7 days', style: TextStyle(fontFamily: 'Outfit', fontSize: 12, color: c.mutedForeground)),
              ]),
              const SizedBox(height: 12),
              SomaAreaChart(
                data: const [
                  SomaChartPoint(label: 'Mon', value: 65),
                  SomaChartPoint(label: 'Tue', value: 72),
                  SomaChartPoint(label: 'Wed', value: 58),
                  SomaChartPoint(label: 'Thu', value: 81),
                  SomaChartPoint(label: 'Fri', value: 76),
                  SomaChartPoint(label: 'Sat', value: 90),
                  SomaChartPoint(label: 'Sun', value: 84),
                ],
                variant: SomaAreaChartVariant.gradient,
              ),
            ]),
          ),
        ),
        const SizedBox(height: 20),
        SomaCard(
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text('Recent Activity', style: TextStyle(fontFamily: 'Outfit', fontSize: 14, fontWeight: FontWeight.w600, color: c.foreground)),
              const SizedBox(height: 12),
              SomaTable(
                columns: const [Text('Pipeline'), Text('Last Run'), Text('Status')],
                rows: [
                  [const Text('data-ingest-01'), const Text('2 min ago'), SomaBadge(variant: SomaBadgeVariant.success, child: const Text('success'))],
                  [const Text('etl-pipeline-02'), const Text('15 min ago'), SomaBadge(variant: SomaBadgeVariant.success, child: const Text('success'))],
                  [const Text('report-gen-03'), const Text('1 hr ago'), SomaBadge(variant: SomaBadgeVariant.secondary, child: const Text('idle'))],
                  [const Text('sync-pipeline-04'), const Text('3 hr ago'), SomaBadge(variant: SomaBadgeVariant.destructive, child: const Text('error'))],
                  [const Text('backup-job-05'), const Text('6 hr ago'), SomaBadge(variant: SomaBadgeVariant.secondary, child: const Text('idle'))],
                ],
              ),
            ]),
          ),
        ),
      ]),
    );
  }
}

class _KpiCard extends StatelessWidget {
  final SomaColors c;
  final String label;
  final String value;
  final String badge;
  final SomaBadgeVariant badgeVariant;

  const _KpiCard({required this.c, required this.label, required this.value, required this.badge, required this.badgeVariant});

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(minWidth: 160),
      child: SomaCard(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Text(label, style: TextStyle(fontFamily: 'Outfit', fontSize: 11, color: c.mutedForeground)),
            const SizedBox(height: 8),
            Text(value, style: TextStyle(fontFamily: 'Rajdhani', fontSize: 28, fontWeight: FontWeight.w700, color: c.foreground)),
            const SizedBox(height: 4),
            SomaBadge(variant: badgeVariant, child: Text(badge)),
          ]),
        ),
      ),
    );
  }
}
