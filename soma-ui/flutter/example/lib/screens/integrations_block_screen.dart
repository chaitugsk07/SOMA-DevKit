import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import '../component_page.dart';

class IntegrationsBlockScreen extends StatelessWidget {
  const IntegrationsBlockScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComponentPage(
      title: 'Integrations Block',
      subtitle: 'Responsive grid of integration cards with connect/disconnect actions.',
      preview: SizedBox(
        width: double.infinity,
        child: SomaIntegrationsBlock(
          items: const [
            SomaIntegrationItem(
              initials: 'GH',
              name: 'GitHub',
              description: 'Connect your repositories and automate deployments.',
              connected: true,
            ),
            SomaIntegrationItem(
              initials: 'SL',
              name: 'Slack',
              description: 'Receive real-time notifications in your workspace.',
              connected: true,
            ),
            SomaIntegrationItem(
              initials: 'PG',
              name: 'PostgreSQL',
              description: 'Connect your database for direct query access.',
            ),
            SomaIntegrationItem(
              initials: 'S3',
              name: 'AWS S3',
              description: 'Store and serve files from scalable object storage.',
            ),
            SomaIntegrationItem(
              initials: 'ST',
              name: 'Stripe',
              description: 'Accept payments and manage subscriptions.',
            ),
            SomaIntegrationItem(
              initials: 'DD',
              name: 'Datadog',
              description: 'Monitor infrastructure, logs, and application performance.',
            ),
          ],
        ),
      ),
      controls: const SizedBox.shrink(),
    );
  }
}
