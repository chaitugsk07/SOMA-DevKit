import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class DashboardScreenDemo extends StatelessWidget {
  const DashboardScreenDemo({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Dashboard Screen',
      subtitle: 'Admin shell with sidebar, KPIs, chart, and activity table',
      preview: const SizedBox(height: 600, child: SomaDashboardScreen()),
      controls: const SizedBox.shrink(),
    );
  }
}
