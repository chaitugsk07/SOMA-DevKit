import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class BreadcrumbScreen extends StatelessWidget {
  const BreadcrumbScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Breadcrumb',
      subtitle: 'Trail navigation showing the current page location.',
      preview: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SomaBreadcrumb(
            items: [
              SomaBreadcrumbItem(label: 'Home', onTap: () {}),
              SomaBreadcrumbItem(label: 'Projects', onTap: () {}),
              SomaBreadcrumbItem(label: 'soma-ui', onTap: () {}),
              const SomaBreadcrumbItem(label: 'Components', isCurrent: true),
            ],
          ),
          const SizedBox(height: 16),
          SomaBreadcrumb(
            items: [
              SomaBreadcrumbItem(label: 'Dashboard', onTap: () {}),
              const SomaBreadcrumbItem(label: 'Settings', isCurrent: true),
            ],
          ),
        ],
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
