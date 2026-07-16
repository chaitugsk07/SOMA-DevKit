import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';

// Curated list of ~120 common Lucide icons grouped by concept.
const _icons = <(String, IconData)>[
  // Arrows & Chevrons
  ('arrowUp', LucideIcons.arrowUp),
  ('arrowDown', LucideIcons.arrowDown),
  ('arrowLeft', LucideIcons.arrowLeft),
  ('arrowRight', LucideIcons.arrowRight),
  ('arrowUpRight', LucideIcons.arrowUpRight),
  ('arrowDownLeft', LucideIcons.arrowDownLeft),
  ('chevronUp', LucideIcons.chevronUp),
  ('chevronDown', LucideIcons.chevronDown),
  ('chevronLeft', LucideIcons.chevronLeft),
  ('chevronRight', LucideIcons.chevronRight),
  ('chevronsUpDown', LucideIcons.chevronsUpDown),
  ('chevronsLeftRight', LucideIcons.chevronsLeftRight),
  // Actions
  ('plus', LucideIcons.plus),
  ('minus', LucideIcons.minus),
  ('x', LucideIcons.x),
  ('check', LucideIcons.check),
  ('checkCheck', LucideIcons.checkCheck),
  ('copy', LucideIcons.copy),
  ('edit', LucideIcons.edit),
  ('pencil', LucideIcons.pencil),
  ('trash', LucideIcons.trash),
  ('save', LucideIcons.save),
  ('download', LucideIcons.download),
  ('upload', LucideIcons.upload),
  ('share', LucideIcons.share),
  ('send', LucideIcons.send),
  ('refreshCw', LucideIcons.refreshCw),
  ('rotateCw', LucideIcons.rotateCw),
  ('undo', LucideIcons.undo),
  ('zoomIn', LucideIcons.zoomIn),
  ('zoomOut', LucideIcons.zoomOut),
  ('expand', LucideIcons.expand),
  ('maximize', LucideIcons.maximize),
  ('minimize', LucideIcons.minimize),
  // Navigation
  ('home', LucideIcons.home),
  ('menu', LucideIcons.menu),
  ('search', LucideIcons.search),
  ('settings', LucideIcons.settings),
  ('layoutDashboard', LucideIcons.layoutDashboard),
  ('layoutGrid', LucideIcons.layoutGrid),
  ('layoutList', LucideIcons.layoutList),
  ('list', LucideIcons.list),
  ('sidebar', LucideIcons.sidebar),
  ('compass', LucideIcons.compass),
  ('map', LucideIcons.map),
  ('mapPin', LucideIcons.mapPin),
  ('externalLink', LucideIcons.externalLink),
  ('link', LucideIcons.link),
  // People
  ('user', LucideIcons.user),
  ('userRound', LucideIcons.userRound),
  ('users', LucideIcons.users),
  ('usersRound', LucideIcons.usersRound),
  ('userPlus', LucideIcons.userPlus),
  ('userMinus', LucideIcons.userMinus),
  ('userCheck', LucideIcons.userCheck),
  ('smile', LucideIcons.smile),
  ('laugh', LucideIcons.laugh),
  ('frown', LucideIcons.frown),
  // Communication
  ('mail', LucideIcons.mail),
  ('mailOpen', LucideIcons.mailOpen),
  ('phone', LucideIcons.phone),
  ('phoneCall', LucideIcons.phoneCall),
  ('bell', LucideIcons.bell),
  ('bellRing', LucideIcons.bellRing),
  ('bellOff', LucideIcons.bellOff),
  ('messageSquare', LucideIcons.messageSquare),
  ('messageCircle', LucideIcons.messageCircle),
  ('atSign', LucideIcons.atSign),
  // Files & Folders
  ('file', LucideIcons.file),
  ('fileText', LucideIcons.fileText),
  ('fileCode', LucideIcons.fileCode),
  ('fileImage', LucideIcons.fileImage),
  ('filePlus', LucideIcons.filePlus),
  ('fileMinus', LucideIcons.fileMinus),
  ('folder', LucideIcons.folder),
  ('folderOpen', LucideIcons.folderOpen),
  ('folderPlus', LucideIcons.folderPlus),
  ('archive', LucideIcons.archive),
  ('package', LucideIcons.package),
  // Editing & Media
  ('image', LucideIcons.image),
  ('camera', LucideIcons.camera),
  ('video', LucideIcons.video),
  ('play', LucideIcons.play),
  ('pause', LucideIcons.pause),
  ('mic', LucideIcons.mic),
  ('volume', LucideIcons.volume),
  ('headset', LucideIcons.headset),
  ('bold', LucideIcons.bold),
  ('italic', LucideIcons.italic),
  ('heading', LucideIcons.heading),
  ('type', LucideIcons.type),
  // Status
  ('info', LucideIcons.info),
  ('circleCheck', LucideIcons.circleCheck),
  ('circleX', LucideIcons.circleX),
  ('circleAlert', LucideIcons.circleAlert),
  ('circleHelp', LucideIcons.circleHelp),
  ('alertTriangle', LucideIcons.alertTriangle),
  ('alertCircle', LucideIcons.alertCircle),
  ('shieldCheck', LucideIcons.shieldCheck),
  ('shieldAlert', LucideIcons.shieldAlert),
  ('zap', LucideIcons.zap),
  ('sparkles', LucideIcons.sparkles),
  ('flame', LucideIcons.flame),
  // Theme & Settings
  ('sun', LucideIcons.sun),
  ('moon', LucideIcons.moon),
  ('sunMoon', LucideIcons.sunMoon),
  ('eye', LucideIcons.eye),
  ('eyeOff', LucideIcons.eyeOff),
  ('lock', LucideIcons.lock),
  ('unlock', LucideIcons.unlock),
  ('key', LucideIcons.key),
  ('palette', LucideIcons.palette),
  ('layers', LucideIcons.layers),
  ('sliders', LucideIcons.sliders),
  // Time
  ('calendar', LucideIcons.calendar),
  ('clock', LucideIcons.clock),
  ('hourglass', LucideIcons.hourglass),
  ('timer', LucideIcons.timer),
  // Tech
  ('code', LucideIcons.code),
  ('terminal', LucideIcons.terminal),
  ('database', LucideIcons.database),
  ('server', LucideIcons.server),
  ('cloud', LucideIcons.cloud),
  ('cpu', LucideIcons.cpu),
  ('monitor', LucideIcons.monitor),
  ('laptop', LucideIcons.laptop),
  ('wifi', LucideIcons.wifi),
  ('bluetooth', LucideIcons.bluetooth),
  ('gitBranch', LucideIcons.gitBranch),
  ('gitCommit', LucideIcons.gitCommit),
  ('gitMerge', LucideIcons.gitMerge),
  ('gitPullRequest', LucideIcons.gitPullRequest),
];

class IconsPage extends StatefulWidget {
  const IconsPage({super.key});

  @override
  State<IconsPage> createState() => _IconsPageState();
}

class _IconsPageState extends State<IconsPage> {
  String _query = '';

  List<(String, IconData)> get _filtered {
    if (_query.isEmpty) return _icons;
    final q = _query.toLowerCase();
    return _icons.where((e) => e.$1.toLowerCase().contains(q)).toList();
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final filtered = _filtered;

    return Column(
      children: [
        // Search bar
        Padding(
          padding: const EdgeInsets.all(24),
          child: Row(
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text('Icons',
                        style: Theme.of(context).textTheme.headlineMedium),
                    const SizedBox(height: 4),
                    Text(
                      'Lucide icon set (parity with the web crate).',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: c.mutedForeground,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
        Padding(
          padding: const EdgeInsets.fromLTRB(24, 0, 24, 16),
          child: Row(
            children: [
              Icon(LucideIcons.search, size: 16, color: c.mutedForeground),
              const SizedBox(width: 8),
              Expanded(
                child: SomaInput(
                  placeholder: 'Search icons…',
                  onChanged: (v) => setState(() => _query = v),
                ),
              ),
              const SizedBox(width: 12),
              Text(
                '${filtered.length} / ${_icons.length}',
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 12,
                  color: c.mutedForeground,
                ),
              ),
            ],
          ),
        ),

        // Grid
        Expanded(
          child: LayoutBuilder(builder: (context, constraints) {
            final cols = (constraints.maxWidth / 96).floor().clamp(3, 8);
            return filtered.isEmpty
                ? Center(
                    child: Text(
                      'No icons match "$_query"',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: c.mutedForeground,
                      ),
                    ),
                  )
                : GridView.builder(
                    padding: const EdgeInsets.fromLTRB(24, 0, 24, 24),
                    gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                      crossAxisCount: cols,
                      mainAxisSpacing: 8,
                      crossAxisSpacing: 8,
                      mainAxisExtent: 80,
                    ),
                    itemCount: filtered.length,
                    itemBuilder: (context, i) {
                      final (name, data) = filtered[i];
                      return _IconCell(name: name, data: data);
                    },
                  );
          }),
        ),
      ],
    );
  }
}

class _IconCell extends StatefulWidget {
  final String name;
  final IconData data;

  const _IconCell({required this.name, required this.data});

  @override
  State<_IconCell> createState() => _IconCellState();
}

class _IconCellState extends State<_IconCell> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 120),
        decoration: BoxDecoration(
          color: _hovered ? c.accent : Colors.transparent,
          border: Border.all(color: _hovered ? c.ring : c.border),
          borderRadius: BorderRadius.circular(6),
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(widget.data, size: 24, color: c.foreground),
            const SizedBox(height: 6),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 4),
              child: Text(
                widget.name,
                textAlign: TextAlign.center,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: TextStyle(
                  fontFamily: 'Outfit',
                  fontSize: 11,
                  color: c.mutedForeground,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
