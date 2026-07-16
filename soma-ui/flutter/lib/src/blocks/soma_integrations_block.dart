import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

class SomaIntegrationItem {
  final String initials;
  final String name;
  final String description;
  final bool connected;
  const SomaIntegrationItem({
    required this.initials,
    required this.name,
    required this.description,
    this.connected = false,
  });
}

class SomaIntegrationsBlock extends StatelessWidget {
  final String title;
  final String subtitle;
  final List<SomaIntegrationItem> items;
  final void Function(SomaIntegrationItem)? onConnect;
  final void Function(SomaIntegrationItem)? onDisconnect;

  const SomaIntegrationsBlock({
    super.key,
    this.title = 'Integrations',
    this.subtitle = 'Connect your favourite tools and services.',
    this.items = const [],
    this.onConnect,
    this.onDisconnect,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Padding(
      padding: const EdgeInsets.all(32),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: TextStyle(
              fontFamily: 'Rajdhani',
              fontSize: 24,
              fontWeight: FontWeight.w700,
              color: c.foreground,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            subtitle,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              color: c.mutedForeground,
            ),
          ),
          const SizedBox(height: 24),
          LayoutBuilder(
            builder: (context, constraints) {
              final width = constraints.maxWidth;
              final columnCount = width >= 900 ? 3 : (width >= 560 ? 2 : 1);
              return GridView.builder(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: columnCount,
                  mainAxisSpacing: 16,
                  crossAxisSpacing: 16,
                  mainAxisExtent: 200,
                ),
                itemCount: items.length,
                itemBuilder: (context, index) => _IntegrationCard(
                  item: items[index],
                  onConnect: onConnect,
                  onDisconnect: onDisconnect,
                ),
              );
            },
          ),
        ],
      ),
    );
  }
}

class _IntegrationCard extends StatelessWidget {
  final SomaIntegrationItem item;
  final void Function(SomaIntegrationItem)? onConnect;
  final void Function(SomaIntegrationItem)? onDisconnect;

  const _IntegrationCard({
    required this.item,
    this.onConnect,
    this.onDisconnect,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return SomaCard(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              // Initials box
              Container(
                width: 40,
                height: 40,
                decoration: BoxDecoration(
                  color: c.muted,
                  borderRadius: BorderRadius.circular(8),
                ),
                alignment: Alignment.center,
                child: Text(
                  item.initials,
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    color: c.foreground,
                  ),
                ),
              ),
              const Spacer(),
              if (item.connected)
                SomaBadge(
                  variant: SomaBadgeVariant.success,
                  child: const Text('Connected'),
                ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            item.name,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              fontWeight: FontWeight.w500,
              color: c.foreground,
            ),
          ),
          const SizedBox(height: 4),
          Expanded(
            child: Text(
              item.description,
              style: TextStyle(
                fontFamily: 'Outfit',
                fontSize: 12,
                color: c.mutedForeground,
              ),
              overflow: TextOverflow.ellipsis,
              maxLines: 2,
            ),
          ),
          const SizedBox(height: 12),
          SizedBox(
            width: double.infinity,
            child: SomaButton(
              variant: item.connected
                  ? SomaButtonVariant.outline
                  : SomaButtonVariant.primary,
              size: SomaButtonSize.sm,
              onPressed: () {
                if (item.connected) {
                  onDisconnect?.call(item);
                } else {
                  onConnect?.call(item);
                }
              },
              child: Text(item.connected ? 'Disconnect' : 'Connect'),
            ),
          ),
        ],
      ),
    );
  }
}
