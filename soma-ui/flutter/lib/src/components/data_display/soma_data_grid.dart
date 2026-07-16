import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../theme/soma_colors.dart';
import '../../icons/soma_icons.dart';
import 'soma_data_table.dart';

/// A virtualized, sortable data grid using [ListView.builder] for real row
/// virtualization (Flutter-idiomatic analog of the web data_grid.rs virtual grid).
///
/// Reuses [SomaDataColumn] from soma_data_table.dart.
/// Header is sticky (wrapped in a [Column] above the scrollable body).
/// [height] constrains the body; defaults to 400 if omitted.
class SomaDataGrid extends StatefulWidget {
  final List<SomaDataColumn> columns;
  final List<Map<String, String>> rows;

  /// Fixed height for the scrollable body. Defaults to 400.
  final double? height;

  const SomaDataGrid({
    super.key,
    required this.columns,
    required this.rows,
    this.height,
  });

  @override
  State<SomaDataGrid> createState() => _SomaDataGridState();
}

class _SomaDataGridState extends State<SomaDataGrid> {
  String? _sortKey;
  bool _sortAsc = true;

  List<Map<String, String>> get _sorted {
    if (_sortKey == null) return widget.rows;
    final key = _sortKey!;
    final asc = _sortAsc;
    final copy = List<Map<String, String>>.from(widget.rows);
    copy.sort((a, b) {
      final av = a[key] ?? '';
      final bv = b[key] ?? '';
      final na = double.tryParse(av);
      final nb = double.tryParse(bv);
      final ord = (na != null && nb != null)
          ? na.compareTo(nb)
          : av.compareTo(bv);
      return asc ? ord : -ord;
    });
    return copy;
  }

  void _toggleSort(String key) {
    setState(() {
      if (_sortKey == key) {
        _sortAsc = !_sortAsc;
      } else {
        _sortKey = key;
        _sortAsc = true;
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final sorted = _sorted;
    final bodyHeight = widget.height ?? 400.0;

    return Container(
      decoration: BoxDecoration(
        border: Border.all(color: c.border),
        borderRadius: BorderRadius.circular(6),
        boxShadow: [
          BoxShadow(color: Colors.black.withAlpha(10), blurRadius: 8, offset: const Offset(0, 2)),
          BoxShadow(color: Colors.black.withAlpha(5), blurRadius: 2, offset: const Offset(0, 1)),
        ],
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(6),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Sticky header
            Container(
              decoration: BoxDecoration(
                color: c.muted.withAlpha(80),
                border: Border(bottom: BorderSide(color: c.border)),
                boxShadow: [
                  BoxShadow(color: Colors.black.withAlpha(8), blurRadius: 4, offset: const Offset(0, 2)),
                ],
              ),
              child: Row(
                children: widget.columns.map((col) {
                  return Expanded(
                    child: _GridHeaderCell(
                      col: col,
                      sortKey: _sortKey,
                      sortAsc: _sortAsc,
                      onSort: col.sortable ? () => _toggleSort(col.key) : null,
                      colors: c,
                    ),
                  );
                }).toList(),
              ),
            ),
            // Virtualized body
            SizedBox(
              height: bodyHeight,
              child: ListView.builder(
                itemCount: sorted.length,
                itemBuilder: (context, i) {
                  return _GridRow(
                    row: sorted[i],
                    columns: widget.columns,
                    colors: c,
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _GridHeaderCell extends StatefulWidget {
  final SomaDataColumn col;
  final String? sortKey;
  final bool sortAsc;
  final VoidCallback? onSort;
  final SomaColors colors;

  const _GridHeaderCell({
    required this.col,
    required this.sortKey,
    required this.sortAsc,
    required this.colors,
    this.onSort,
  });

  @override
  State<_GridHeaderCell> createState() => _GridHeaderCellState();
}

class _GridHeaderCellState extends State<_GridHeaderCell> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    final isActive = widget.sortKey == widget.col.key;
    final isSortable = widget.onSort != null;

    Widget label = Text(
      widget.col.header.toUpperCase(),
      style: TextStyle(
        fontFamily: 'Rajdhani',
        fontSize: 12,
        fontWeight: FontWeight.w600,
        letterSpacing: 0.8,
        color: c.mutedForeground,
      ),
    );

    if (isSortable) {
      label = Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          label,
          const SizedBox(width: 4),
          Icon(
            isActive
                ? (widget.sortAsc ? LucideIcons.chevronUp : LucideIcons.chevronDown)
                : LucideIcons.chevronsUpDown,
            size: 13,
            color: isActive ? c.foreground : c.mutedForeground,
          ),
        ],
      );
    }

    Widget cell = AnimatedContainer(
      duration: const Duration(milliseconds: 120),
      curve: Curves.easeOutCubic,
      color: isSortable && _hovered ? c.accent.withAlpha(30) : Colors.transparent,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
        child: label,
      ),
    );

    if (isSortable) {
      cell = MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: GestureDetector(
          onTap: widget.onSort,
          child: cell,
        ),
      );
    }

    return cell;
  }
}

class _GridRow extends StatefulWidget {
  final Map<String, String> row;
  final List<SomaDataColumn> columns;
  final SomaColors colors;

  const _GridRow({
    required this.row,
    required this.columns,
    required this.colors,
  });

  @override
  State<_GridRow> createState() => _GridRowState();
}

class _GridRowState extends State<_GridRow> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 100),
        decoration: BoxDecoration(
          color: _hovered ? c.accent.withAlpha(70) : Colors.transparent,
          border: Border(top: BorderSide(color: c.border)),
        ),
        child: Row(
          children: widget.columns.map((col) {
            return Expanded(
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                child: Text(
                  widget.row[col.key] ?? '',
                  style: TextStyle(
                    fontFamily: 'Outfit',
                    fontSize: 14,
                    color: c.foreground,
                  ),
                ),
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}
