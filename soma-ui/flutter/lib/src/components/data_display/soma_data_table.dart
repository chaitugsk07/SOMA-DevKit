import 'package:flutter/material.dart';
import '../../theme/soma_theme.dart';
import '../../icons/soma_icons.dart';
import '../inputs/soma_checkbox.dart';
import '../inputs/soma_input.dart';
import '../navigation/soma_pagination.dart';

class SomaDataColumn {
  final String key;
  final String header;
  final bool sortable;

  const SomaDataColumn({
    required this.key,
    required this.header,
    this.sortable = false,
  });
}

class SomaDataTable extends StatefulWidget {
  final List<SomaDataColumn> columns;
  final List<Map<String, String>> rows;
  final bool selectable;
  final bool filterable;
  final int pageSize;

  const SomaDataTable({
    super.key,
    required this.columns,
    required this.rows,
    this.selectable = false,
    this.filterable = false,
    this.pageSize = 0,
  });

  @override
  State<SomaDataTable> createState() => _SomaDataTableState();
}

class _SomaDataTableState extends State<SomaDataTable> {
  String _filter = '';
  String? _sortKey;
  bool _sortAsc = true;
  final Set<int> _selected = {};
  int _page = 1;

  List<({int origIndex, Map<String, String> row})> get _processed {
    var result = widget.rows
        .asMap()
        .entries
        .where((e) {
          if (_filter.isEmpty) return true;
          final f = _filter.toLowerCase();
          return e.value.values.any((v) => v.toLowerCase().contains(f));
        })
        .map((e) => (origIndex: e.key, row: e.value))
        .toList();

    if (_sortKey != null) {
      final key = _sortKey!;
      final asc = _sortAsc;
      result.sort((a, b) {
        final av = a.row[key] ?? '';
        final bv = b.row[key] ?? '';
        final na = double.tryParse(av);
        final nb = double.tryParse(bv);
        final ord = (na != null && nb != null)
            ? na.compareTo(nb)
            : av.compareTo(bv);
        return asc ? ord : -ord;
      });
    }

    return result;
  }

  int get _totalPages {
    if (widget.pageSize <= 0) return 1;
    final n = _processed.length;
    return n == 0 ? 1 : (n + widget.pageSize - 1) ~/ widget.pageSize;
  }

  List<({int origIndex, Map<String, String> row})> get _paged {
    final rows = _processed;
    if (widget.pageSize <= 0) return rows;
    final page = _page.clamp(1, _totalPages) - 1;
    return rows.skip(page * widget.pageSize).take(widget.pageSize).toList();
  }

  void _toggleSort(String key) {
    setState(() {
      if (_sortKey == key) {
        _sortAsc = !_sortAsc;
      } else {
        _sortKey = key;
        _sortAsc = true;
      }
      _page = 1;
    });
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final paged = _paged;
    final allOnPageSelected = paged.isNotEmpty &&
        paged.every((r) => _selected.contains(r.origIndex));

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisSize: MainAxisSize.min,
      children: [
        if (widget.filterable) ...[
          SomaInput(
            placeholder: 'Filter…',
            onChanged: (v) => setState(() {
              _filter = v;
              _page = 1;
            }),
          ),
          const SizedBox(height: 12),
        ],
        // Header row
        Container(
          decoration: BoxDecoration(
            color: c.muted.withAlpha(80),
            border: Border(bottom: BorderSide(color: c.border)),
          ),
          child: Row(
            children: [
              if (widget.selectable)
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                  child: SomaCheckbox(
                    value: allOnPageSelected,
                    onChanged: (_) {
                      setState(() {
                        if (allOnPageSelected) {
                          for (final r in paged) { _selected.remove(r.origIndex); }
                        } else {
                          for (final r in paged) { _selected.add(r.origIndex); }
                        }
                      });
                    },
                  ),
                ),
              ...widget.columns.map((col) => Expanded(
                    child: _HeaderCell(
                      col: col,
                      sortKey: _sortKey,
                      sortAsc: _sortAsc,
                      onSort: col.sortable ? () => _toggleSort(col.key) : null,
                    ),
                  )),
            ],
          ),
        ),
        // Data rows
        for (final entry in paged)
          _DataRow(
            entry: entry,
            columns: widget.columns,
            isSelected: _selected.contains(entry.origIndex),
            selectable: widget.selectable,
            onToggleSelect: () {
              setState(() {
                if (_selected.contains(entry.origIndex)) {
                  _selected.remove(entry.origIndex);
                } else {
                  _selected.add(entry.origIndex);
                }
              });
            },
          ),
        if (widget.pageSize > 0 && _totalPages > 1) ...[
          const SizedBox(height: 8),
          Align(
            alignment: Alignment.centerRight,
            child: SomaPagination(
              page: _page.clamp(1, _totalPages),
              totalPages: _totalPages,
              onChanged: (p) => setState(() => _page = p),
            ),
          ),
        ],
      ],
    );
  }
}

class _HeaderCell extends StatefulWidget {
  final SomaDataColumn col;
  final String? sortKey;
  final bool sortAsc;
  final VoidCallback? onSort;

  const _HeaderCell({
    required this.col,
    required this.sortKey,
    required this.sortAsc,
    this.onSort,
  });

  @override
  State<_HeaderCell> createState() => _HeaderCellState();
}

class _HeaderCellState extends State<_HeaderCell> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
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
            size: 14,
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

class _DataRow extends StatefulWidget {
  final ({int origIndex, Map<String, String> row}) entry;
  final List<SomaDataColumn> columns;
  final bool isSelected;
  final bool selectable;
  final VoidCallback onToggleSelect;

  const _DataRow({
    required this.entry,
    required this.columns,
    required this.isSelected,
    required this.selectable,
    required this.onToggleSelect,
  });

  @override
  State<_DataRow> createState() => _DataRowState();
}

class _DataRowState extends State<_DataRow> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    Color rowColor;
    if (_hovered) {
      rowColor = c.accent.withAlpha(60);
    } else if (widget.isSelected) {
      rowColor = c.primary.withAlpha(18);
    } else {
      rowColor = Colors.transparent;
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 120),
        curve: Curves.easeOutCubic,
        decoration: BoxDecoration(
          color: rowColor,
          border: Border(top: BorderSide(color: c.border)),
        ),
        child: Row(
          children: [
            if (widget.selectable)
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                child: SomaCheckbox(
                  value: widget.isSelected,
                  onChanged: (_) => widget.onToggleSelect(),
                ),
              ),
            ...widget.columns.map((col) => Expanded(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                    child: Text(
                      widget.entry.row[col.key] ?? '',
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        fontSize: 14,
                        color: c.foreground,
                      ),
                    ),
                  ),
                )),
          ],
        ),
      ),
    );
  }
}
