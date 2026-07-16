import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaMobileAppScreen extends StatefulWidget {
  const SomaMobileAppScreen({super.key});
  @override
  State<SomaMobileAppScreen> createState() => _SomaMobileAppScreenState();
}

class _SomaMobileAppScreenState extends State<SomaMobileAppScreen> {
  int _activeTab = 0;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 390),
        child: Column(children: [
          // Top bar
          Container(
            height: 48,
            color: c.card,
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Row(children: [
              Text('FOUNDRY MOBILE', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 16, fontWeight: FontWeight.w700, color: c.foreground)),
              const Spacer(),
              Icon(LucideIcons.search, size: 18, color: c.mutedForeground),
              const SizedBox(width: 16),
              Icon(LucideIcons.bell, size: 18, color: c.mutedForeground),
            ]),
          ),
          // Content
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(16),
              child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                // Status row
                Row(children: [
                  Container(width: 8, height: 8, decoration: const BoxDecoration(color: Color(0xFF22c55e), shape: BoxShape.circle)),
                  const SizedBox(width: 8),
                  Text('OPERATIONAL', style: TextStyle(fontFamily: 'Outfit', fontSize: 12, color: c.mutedForeground)),
                ]),
                const SizedBox(height: 12),
                // Stat cards
                Row(children: [
                  Expanded(child: SomaCard(
                    child: Padding(
                      padding: const EdgeInsets.all(12),
                      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                        Text('PIPELINES', style: TextStyle(fontFamily: 'Outfit', fontSize: 11, color: c.mutedForeground)),
                        const SizedBox(height: 4),
                        Text('142', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 24, fontWeight: FontWeight.w700, color: c.foreground)),
                      ]),
                    ),
                  )),
                  const SizedBox(width: 8),
                  Expanded(child: SomaCard(
                    child: Padding(
                      padding: const EdgeInsets.all(12),
                      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                        Text('LATENCY', style: TextStyle(fontFamily: 'Outfit', fontSize: 11, color: c.mutedForeground)),
                        const SizedBox(height: 4),
                        Text('45ms', style: TextStyle(fontFamily: 'Rajdhani', fontSize: 24, fontWeight: FontWeight.w700, color: c.foreground)),
                      ]),
                    ),
                  )),
                ]),
                const SizedBox(height: 12),
                Text('Active Pipelines', style: TextStyle(fontFamily: 'Outfit', fontSize: 13, fontWeight: FontWeight.w600, color: c.foreground)),
                const SizedBox(height: 8),
                ..._buildPipelineRows(c),
              ]),
            ),
          ),
          // Bottom nav
          SomaBottomNav(
            items: const [
              SomaBottomNavItem(icon: LucideIcons.house, label: 'Home'),
              SomaBottomNavItem(icon: LucideIcons.gitBranch, label: 'Pipelines'),
              SomaBottomNavItem(icon: LucideIcons.bell, label: 'Alerts'),
              SomaBottomNavItem(icon: LucideIcons.user, label: 'Profile'),
            ],
            index: _activeTab,
            onTap: (i) => setState(() => _activeTab = i),
          ),
        ]),
      ),
    );
  }

  List<Widget> _buildPipelineRows(SomaColors c) {
    const names = ['user-events-ingest', 'etl-transform-v2', 'report-gen-daily', 'data-sync-prod', 'backup-cold-store'];
    const timestamps = ['2 min ago', '8 min ago', '15 min ago', '1 hr ago', '3 hr ago'];
    const colors = [Color(0xFF22c55e), Color(0xFF22c55e), Color(0xFFf59e0b), Color(0xFF22c55e), Color(0xFFef4444)];
    return List.generate(names.length, (i) => Container(
      margin: const EdgeInsets.only(bottom: 8),
      child: Row(children: [
        Container(width: 8, height: 8, decoration: BoxDecoration(color: colors[i], shape: BoxShape.circle)),
        const SizedBox(width: 10),
        Expanded(child: Text(names[i], style: TextStyle(fontFamily: 'Outfit', fontSize: 13, color: c.foreground))),
        Text(timestamps[i], style: TextStyle(fontFamily: 'Outfit', fontSize: 11, color: c.mutedForeground)),
      ]),
    ));
  }
}
