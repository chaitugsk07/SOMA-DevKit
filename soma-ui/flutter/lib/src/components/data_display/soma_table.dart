import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';

class SomaTable extends StatefulWidget {
  final List<Widget> columns;
  final List<List<Widget>> rows;
  final String? caption;

  const SomaTable({
    super.key,
    required this.columns,
    required this.rows,
    this.caption,
  });

  @override
  State<SomaTable> createState() => _SomaTableState();
}

class _SomaTableState extends State<SomaTable> {
  int? _hoveredRow;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisSize: MainAxisSize.min,
      children: [
        _buildHeaderRow(c),
        for (int i = 0; i < widget.rows.length; i++) _buildDataRow(c, i),
        if (widget.caption != null) ...[
          const SizedBox(height: 8),
          Text(
            widget.caption!,
            textAlign: TextAlign.center,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 13,
              color: c.mutedForeground,
            ),
          ),
        ],
      ],
    );
  }

  Widget _buildHeaderRow(SomaColors c) {
    return Container(
      decoration: BoxDecoration(
        color: c.muted.withAlpha(80),
        border: Border(bottom: BorderSide(color: c.border, width: 1)),
      ),
      child: Row(
        children: widget.columns.map((col) {
          return Expanded(
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
              child: DefaultTextStyle(
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 12,
                  fontWeight: FontWeight.w600,
                  letterSpacing: 0.8,
                  color: c.mutedForeground,
                ),
                child: col,
              ),
            ),
          );
        }).toList(),
      ),
    );
  }

  Widget _buildDataRow(SomaColors c, int i) {
    final isHovered = _hoveredRow == i;
    return MouseRegion(
      onEnter: (_) => setState(() => _hoveredRow = i),
      onExit: (_) => setState(() => _hoveredRow = null),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 120),
        curve: Curves.easeOutCubic,
        decoration: BoxDecoration(
          color: isHovered ? c.accent.withAlpha(60) : Colors.transparent,
          border: Border(top: BorderSide(color: c.border, width: 1)),
        ),
        child: Row(
          children: widget.rows[i].map((cell) {
            return Expanded(
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                child: DefaultTextStyle(
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.foreground,
                  ),
                  child: cell,
                ),
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}
