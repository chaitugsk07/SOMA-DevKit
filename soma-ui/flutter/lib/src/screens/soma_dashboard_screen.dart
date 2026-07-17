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
    return SomaConsoleShell(
      brand: 'SOMA Console',
      version: 'v0.2',
      navigationBuilder: (closeNavigation) => _Sidebar(
        c: c,
        activeNav: _activeNav,
        onNavTap: (i) {
          setState(() => _activeNav = i);
          closeNavigation();
        },
      ),
      sidebarFooter: _SidebarFooter(c: c),
      header: _TopBar(c: c),
      body: _MainContent(c: c),
    );
  }
}

class _Sidebar extends StatelessWidget {
  final SomaColors c;
  final int activeNav;
  final ValueChanged<int> onNavTap;

  static const _navItems = [
    ('Overview', LucideIcons.layoutDashboard),
    ('Agents', LucideIcons.bot),
    ('Workflows', LucideIcons.gitBranch),
    ('Data', LucideIcons.database),
    ('Runs', LucideIcons.play),
    ('Observability', LucideIcons.activity),
    ('Audit Log', LucideIcons.clipboardList),
  ];

  const _Sidebar(
      {required this.c, required this.activeNav, required this.onNavTap});

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      padding: const EdgeInsets.all(12),
      itemCount: _navItems.length,
      itemBuilder: (context, i) {
        final (label, icon) = _navItems[i];
        final isActive = i == activeNav;
        return GestureDetector(
          onTap: () => onNavTap(i),
          child: Container(
            decoration: BoxDecoration(
              color: isActive ? c.primary.withAlpha(20) : Colors.transparent,
              border: Border(
                left: BorderSide(
                  color: isActive ? c.primary : Colors.transparent,
                  width: 3,
                ),
              ),
              borderRadius: BorderRadius.circular(4),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
            child: Row(
              children: [
                Icon(
                  icon,
                  size: 16,
                  color: isActive ? c.primary : c.mutedForeground,
                ),
                const SizedBox(width: 10),
                Text(
                  label,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 13,
                    color: isActive ? c.primary : c.mutedForeground,
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

class _SidebarFooter extends StatelessWidget {
  final SomaColors c;

  const _SidebarFooter({required this.c});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        border: Border(top: BorderSide(color: c.border)),
      ),
      child: Column(
        children: [
          Row(
            children: [
              Container(
                width: 7,
                height: 7,
                decoration: BoxDecoration(
                  color: c.success,
                  shape: BoxShape.circle,
                ),
              ),
              const SizedBox(width: 8),
              Text(
                'PRODUCTION · HEALTHY',
                style: TextStyle(
                  fontFamily: 'Roboto Mono',
                  fontSize: 10,
                  color: c.mutedForeground,
                ),
              ),
            ],
          ),
          const SizedBox(height: 10),
          Row(
            children: [
              const SomaAvatar(initials: 'AO', size: SomaAvatarSize.sm),
              const SizedBox(width: 10),
              Text(
                'Alex Operator',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 13,
                  color: c.foreground,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class _TopBar extends StatelessWidget {
  final SomaColors c;

  const _TopBar({required this.c});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: Text(
            'Acme Operations  /  Production',
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              fontWeight: FontWeight.w500,
              color: c.foreground,
            ),
          ),
        ),
        IconButton(
          tooltip: 'Search commands',
          icon: Icon(
            LucideIcons.search,
            size: 18,
            color: c.mutedForeground,
          ),
          onPressed: () => showSomaCommand(
            context,
            groups: const [
              SomaCommandGroup(
                heading: 'Create',
                items: [
                  SomaCommandItem(
                    label: 'Create agent',
                    keywords: 'new assistant',
                    icon: LucideIcons.bot,
                  ),
                  SomaCommandItem(
                    label: 'Create workflow',
                    keywords: 'new automation',
                    icon: LucideIcons.gitBranch,
                  ),
                ],
              ),
              SomaCommandGroup(
                heading: 'Navigate',
                items: [
                  SomaCommandItem(
                    label: 'View runs',
                    keywords: 'jobs executions',
                    icon: LucideIcons.play,
                  ),
                  SomaCommandItem(
                    label: 'Open audit log',
                    keywords: 'security governance',
                    icon: LucideIcons.clipboardList,
                  ),
                ],
              ),
            ],
          ),
        ),
        IconButton(
          tooltip: 'Notifications',
          icon: Icon(
            LucideIcons.bell,
            size: 18,
            color: c.mutedForeground,
          ),
          onPressed: () {},
        ),
        const SizedBox(width: 4),
        const SomaAvatar(initials: 'AO', size: SomaAvatarSize.sm),
      ],
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
        Row(
          children: [
            const SomaBadge(
              variant: SomaBadgeVariant.outline,
              child: Text('PRODUCTION'),
            ),
            const SizedBox(width: 8),
            Container(
              width: 7,
              height: 7,
              decoration: BoxDecoration(
                color: c.success,
                shape: BoxShape.circle,
              ),
            ),
            const SizedBox(width: 6),
            Text(
              'All systems operational',
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 12,
                color: c.success,
              ),
            ),
          ],
        ),
        const SizedBox(height: 8),
        Text('Operations overview',
            style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 24,
                fontWeight: FontWeight.w700,
                color: c.foreground)),
        const SizedBox(height: 4),
        Text(
            'Build agents, orchestrate workflows, and monitor production from one workspace.',
            style: TextStyle(
                fontFamily: 'Outfit', fontSize: 14, color: c.mutedForeground)),
        const SizedBox(height: 20),
        Wrap(spacing: 12, runSpacing: 12, children: [
          _KpiCard(
              c: c,
              label: 'ACTIVE AGENTS',
              value: '24',
              badge: '+3 this week',
              badgeVariant: SomaBadgeVariant.success),
          _KpiCard(
              c: c,
              label: 'WORKFLOW RUNS',
              value: '1,284',
              badge: '+12.4%',
              badgeVariant: SomaBadgeVariant.success),
          _KpiCard(
              c: c,
              label: 'SUCCESS RATE',
              value: '98.7%',
              badge: '+0.8%',
              badgeVariant: SomaBadgeVariant.success),
          _KpiCard(
              c: c,
              label: 'COMPUTE SPEND',
              value: '\$8.4K',
              badge: '72% budget',
              badgeVariant: SomaBadgeVariant.outline),
        ]),
        const SizedBox(height: 20),
        SomaCard(
          child: Padding(
            padding: const EdgeInsets.all(16),
            child:
                Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Row(children: [
                Text('Execution Volume',
                    style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                        color: c.foreground)),
                const SizedBox(width: 8),
                Text('Last 7 days',
                    style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 12,
                        color: c.mutedForeground)),
              ]),
              const SizedBox(height: 12),
              SomaAreaChart(
                data: const [
                  SomaChartPoint(label: 'Mon', value: 128),
                  SomaChartPoint(label: 'Tue', value: 184),
                  SomaChartPoint(label: 'Wed', value: 156),
                  SomaChartPoint(label: 'Thu', value: 221),
                  SomaChartPoint(label: 'Fri', value: 206),
                  SomaChartPoint(label: 'Sat', value: 144),
                  SomaChartPoint(label: 'Sun', value: 178),
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
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'Operations',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    color: c.foreground,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Live workload and governance signals',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 12,
                    color: c.mutedForeground,
                  ),
                ),
                const SizedBox(height: 12),
                _OperationRow(c: c, label: 'Running now', value: '18'),
                _OperationRow(c: c, label: 'Awaiting review', value: '3'),
                _OperationRow(
                    c: c,
                    label: 'Open incidents',
                    value: '1',
                    destructive: true),
                _OperationRow(
                    c: c,
                    label: 'Policy coverage',
                    value: '100%',
                    success: true,
                    showDivider: false),
              ],
            ),
          ),
        ),
        const SizedBox(height: 20),
        SomaCard(
          child: Padding(
            padding: const EdgeInsets.all(16),
            child:
                Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text('Recent Activity',
                  style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 14,
                      fontWeight: FontWeight.w600,
                      color: c.foreground)),
              const SizedBox(height: 12),
              SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                child: SizedBox(
                  width: 760,
                  child: SomaTable(
                    columns: const [
                      Text('Resource'),
                      Text('Type'),
                      Text('Owner'),
                      Text('Last Run'),
                      Text('Status'),
                    ],
                    rows: const [
                      [
                        Text('support-triage'),
                        Text('Agent'),
                        Text('Maya Chen'),
                        Text('2 min ago'),
                        SomaBadge(
                            variant: SomaBadgeVariant.success,
                            child: Text('running'))
                      ],
                      [
                        Text('invoice-review'),
                        Text('Workflow'),
                        Text('Finance Ops'),
                        Text('8 min ago'),
                        SomaBadge(
                            variant: SomaBadgeVariant.success,
                            child: Text('completed'))
                      ],
                      [
                        Text('customer-health'),
                        Text('Agent'),
                        Text('RevOps'),
                        Text('23 min ago'),
                        SomaBadge(
                            variant: SomaBadgeVariant.secondary,
                            child: Text('review'))
                      ],
                      [
                        Text('sync-crm-contacts'),
                        Text('Workflow'),
                        Text('Data Platform'),
                        Text('1 hr ago'),
                        SomaBadge(
                            variant: SomaBadgeVariant.destructive,
                            child: Text('failed'))
                      ],
                      [
                        Text('forecast-refresh'),
                        Text('Workflow'),
                        Text('Planning'),
                        Text('2 hr ago'),
                        SomaBadge(
                            variant: SomaBadgeVariant.success,
                            child: Text('completed'))
                      ],
                    ],
                  ),
                ),
              ),
            ]),
          ),
        ),
      ]),
    );
  }
}

class _OperationRow extends StatelessWidget {
  final SomaColors c;
  final String label;
  final String value;
  final bool destructive;
  final bool success;
  final bool showDivider;

  const _OperationRow({
    required this.c,
    required this.label,
    required this.value,
    this.destructive = false,
    this.success = false,
    this.showDivider = true,
  });

  @override
  Widget build(BuildContext context) {
    final valueColor = destructive
        ? c.destructive
        : success
            ? c.success
            : c.foreground;
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 10),
      decoration: BoxDecoration(
        border:
            showDivider ? Border(bottom: BorderSide(color: c.border)) : null,
      ),
      child: Row(
        children: [
          Expanded(
            child: Text(
              label,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 13,
                color: c.mutedForeground,
              ),
            ),
          ),
          Text(
            value,
            style: TextStyle(
              fontFamily: 'Roboto Mono',
              fontSize: 13,
              fontWeight: FontWeight.w600,
              color: valueColor,
            ),
          ),
        ],
      ),
    );
  }
}

class _KpiCard extends StatelessWidget {
  final SomaColors c;
  final String label;
  final String value;
  final String badge;
  final SomaBadgeVariant badgeVariant;

  const _KpiCard(
      {required this.c,
      required this.label,
      required this.value,
      required this.badge,
      required this.badgeVariant});

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(minWidth: 160),
      child: SomaCard(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child:
              Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Text(label,
                style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 11,
                    color: c.mutedForeground)),
            const SizedBox(height: 8),
            Text(value,
                style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontSize: 28,
                    fontWeight: FontWeight.w700,
                    color: c.foreground)),
            const SizedBox(height: 4),
            SomaBadge(variant: badgeVariant, child: Text(badge)),
          ]),
        ),
      ),
    );
  }
}
