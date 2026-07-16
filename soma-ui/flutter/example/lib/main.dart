import 'package:flutter/material.dart';
import 'package:soma_ui/soma_ui.dart';
import 'screens/getting_started_page.dart';
import 'screens/foundations_page.dart';
import 'screens/hooks_page.dart';
import 'screens/icons_page.dart';
import 'screens/button_screen.dart';
import 'screens/badge_screen.dart';
import 'screens/input_screen.dart';
import 'screens/switch_screen.dart';
import 'screens/checkbox_screen.dart';
import 'screens/radio_group_screen.dart';
import 'screens/slider_screen.dart';
import 'screens/textarea_screen.dart';
import 'screens/select_screen.dart';
import 'screens/toggle_group_screen.dart';
import 'screens/label_screen.dart';
import 'screens/avatar_screen.dart';
import 'screens/separator_screen.dart';
import 'screens/spinner_screen.dart';
import 'screens/progress_screen.dart';
import 'screens/alert_screen.dart';
import 'screens/tabs_screen.dart';
import 'screens/dialog_screen.dart';
import 'screens/bottom_nav_screen.dart';
import 'screens/card_screen.dart';
import 'screens/animate_screen.dart';
import 'screens/marquee_screen.dart';
import 'screens/pressable_screen.dart';
import 'screens/shimmer_screen.dart';
import 'screens/skeleton_screen.dart';
import 'screens/button_group_screen.dart';
import 'screens/button_action_screen.dart';
import 'screens/combobox_screen.dart';
import 'screens/multi_select_screen.dart';
import 'screens/date_picker_screen.dart';
import 'screens/input_otp_screen.dart';
import 'screens/chip_screen.dart';
import 'screens/kbd_screen.dart';
import 'screens/empty_screen.dart';
import 'screens/item_screen.dart';
import 'screens/table_screen.dart';
import 'screens/data_table_screen.dart';
import 'screens/pagination_screen.dart';
import 'screens/breadcrumb_screen.dart';
import 'screens/accordion_screen.dart';
import 'screens/collapsible_screen.dart';
import 'screens/popover_screen.dart';
import 'screens/tooltip_screen.dart';
import 'screens/dropdown_menu_screen.dart';
import 'screens/hover_card_screen.dart';
import 'screens/context_menu_screen.dart';
import 'screens/menubar_screen.dart';
import 'screens/command_screen.dart';
import 'screens/drawer_screen.dart';
import 'screens/sheet_screen.dart';
import 'screens/theme_toggle_screen.dart';
import 'screens/input_group_screen.dart';
import 'screens/input_phone_screen.dart';
import 'screens/input_prompt_screen.dart';
import 'screens/radio_button_screen.dart';
import 'screens/callout_screen.dart';
import 'screens/status_screen.dart';
import 'screens/toast_screen.dart';
import 'screens/field_screen.dart';
import 'screens/form_screen.dart';
import 'screens/auto_form_screen.dart';
import 'screens/aspect_ratio_screen.dart';
import 'screens/scroll_area_screen.dart';
import 'screens/alert_dialog_screen.dart';
import 'screens/navigation_menu_screen.dart';
import 'screens/data_grid_screen.dart';
import 'screens/direction_provider_screen.dart';
import 'screens/drag_and_drop_screen.dart';
import 'screens/dropzone_screen.dart';
import 'screens/animate_group_screen.dart';
import 'screens/carousel_screen.dart';
import 'screens/card_carousel_screen.dart';
import 'screens/area_chart_screen.dart';
import 'screens/bar_chart_screen.dart';
import 'screens/line_chart_screen.dart';
import 'screens/pie_chart_screen.dart';
import 'screens/radar_chart_screen.dart';
import 'screens/radial_chart_screen.dart';
import 'screens/login_block_screen.dart';
import 'screens/sidenav_block_screen.dart';
import 'screens/header_block_screen.dart';
import 'screens/footer_block_screen.dart';
import 'screens/faq_block_screen.dart';
import 'screens/integrations_block_screen.dart';
import 'screens/login_screen_demo.dart';
import 'screens/dashboard_screen_demo.dart';
import 'screens/mobile_app_screen_demo.dart';
import 'screens/settings_screen_demo.dart';

void main() => runApp(const PlaygroundApp());

class PlaygroundApp extends StatefulWidget {
  const PlaygroundApp({super.key});

  @override
  State<PlaygroundApp> createState() => _PlaygroundAppState();
}

class _PlaygroundAppState extends State<PlaygroundApp> {
  ThemeMode _themeMode = ThemeMode.dark;

  @override
  Widget build(BuildContext context) {
    return SomaThemeProvider(
      themeMode: _themeMode,
      child: Builder(builder: (ctx) {
        return MaterialApp(
          title: 'soma_ui playground',
          debugShowCheckedModeBanner: false,
          theme: SomaTheme.buildThemeData(ctx),
          home: AppShell(
            themeMode: _themeMode,
            onToggleTheme: () => setState(() {
              _themeMode = _themeMode == ThemeMode.dark ? ThemeMode.light : ThemeMode.dark;
            }),
          ),
        );
      }),
    );
  }
}

// Nav entry
class _NavEntry {
  final String group;
  final String label;
  final Widget screen;
  final bool topLevel;
  const _NavEntry({
    required this.group,
    required this.label,
    required this.screen,
    this.topLevel = false,
  });
}

const _groupOrder = [
  'Inputs',
  'Forms',
  'Data Display',
  'Feedback',
  'Layout',
  'Navigation',
  'Disclosure',
  'Overlays',
  'Motion',
  'Media',
  'Interaction',
  'Charts',
  'Blocks',
  'Screens',
];

final _navEntries = <_NavEntry>[
  // Top-level special pages
  _NavEntry(group: '', label: 'Getting Started', screen: const GettingStartedPage(), topLevel: true),
  _NavEntry(group: '', label: 'Foundations', screen: const FoundationsPage(), topLevel: true),
  _NavEntry(group: '', label: 'Hooks', screen: const HooksPage(), topLevel: true),
  _NavEntry(group: '', label: 'Icons', screen: const IconsPage(), topLevel: true),
  // Inputs
  _NavEntry(group: 'Inputs', label: 'Button', screen: const ButtonScreen()),
  _NavEntry(group: 'Inputs', label: 'Input', screen: const InputScreen()),
  _NavEntry(group: 'Inputs', label: 'Switch', screen: const SwitchScreen()),
  _NavEntry(group: 'Inputs', label: 'Checkbox', screen: const CheckboxScreen()),
  _NavEntry(group: 'Inputs', label: 'RadioGroup', screen: const RadioGroupScreen()),
  _NavEntry(group: 'Inputs', label: 'Slider', screen: const SliderScreen()),
  _NavEntry(group: 'Inputs', label: 'Textarea', screen: const TextareaScreen()),
  _NavEntry(group: 'Inputs', label: 'Select', screen: const SelectScreen()),
  _NavEntry(group: 'Inputs', label: 'ToggleGroup', screen: const ToggleGroupScreen()),
  _NavEntry(group: 'Inputs', label: 'ButtonGroup', screen: const ButtonGroupScreen()),
  _NavEntry(group: 'Inputs', label: 'ButtonAction', screen: const ButtonActionScreen()),
  _NavEntry(group: 'Inputs', label: 'Combobox', screen: const ComboboxScreen()),
  _NavEntry(group: 'Inputs', label: 'MultiSelect', screen: const MultiSelectScreen()),
  _NavEntry(group: 'Inputs', label: 'DatePicker', screen: const DatePickerScreen()),
  _NavEntry(group: 'Inputs', label: 'InputOtp', screen: const InputOtpScreen()),
  _NavEntry(group: 'Inputs', label: 'Input Group', screen: const InputGroupScreen()),
  _NavEntry(group: 'Inputs', label: 'Input Phone', screen: const InputPhoneScreen()),
  _NavEntry(group: 'Inputs', label: 'Input Prompt', screen: const InputPromptScreen()),
  _NavEntry(group: 'Inputs', label: 'Radio Button', screen: const RadioButtonScreen()),
  // Forms
  _NavEntry(group: 'Forms', label: 'Field', screen: const FieldScreen()),
  _NavEntry(group: 'Forms', label: 'Form', screen: const FormScreen()),
  _NavEntry(group: 'Forms', label: 'AutoForm', screen: const AutoFormScreen()),
  // Data Display
  _NavEntry(group: 'Data Display', label: 'Badge', screen: const BadgeScreen()),
  _NavEntry(group: 'Data Display', label: 'Label', screen: const LabelScreen()),
  _NavEntry(group: 'Data Display', label: 'Avatar', screen: const AvatarScreen()),
  _NavEntry(group: 'Data Display', label: 'Card', screen: const CardScreen()),
  _NavEntry(group: 'Data Display', label: 'Chip', screen: const ChipScreen()),
  _NavEntry(group: 'Data Display', label: 'Kbd', screen: const KbdScreen()),
  _NavEntry(group: 'Data Display', label: 'Empty', screen: const EmptyScreen()),
  _NavEntry(group: 'Data Display', label: 'Item', screen: const ItemScreen()),
  _NavEntry(group: 'Data Display', label: 'Table', screen: const TableScreen()),
  _NavEntry(group: 'Data Display', label: 'DataTable', screen: const DataTableScreen()),
  _NavEntry(group: 'Data Display', label: 'DataGrid', screen: const DataGridScreen()),
  // Feedback
  _NavEntry(group: 'Feedback', label: 'Spinner', screen: const SpinnerScreen()),
  _NavEntry(group: 'Feedback', label: 'Progress', screen: const ProgressScreen()),
  _NavEntry(group: 'Feedback', label: 'Alert', screen: const AlertScreen()),
  _NavEntry(group: 'Feedback', label: 'Callout', screen: const CalloutScreen()),
  _NavEntry(group: 'Feedback', label: 'Status', screen: const StatusScreen()),
  _NavEntry(group: 'Feedback', label: 'Toast', screen: const ToastScreen()),
  // Layout
  _NavEntry(group: 'Layout', label: 'Separator', screen: const SeparatorScreen()),
  _NavEntry(group: 'Layout', label: 'AspectRatio', screen: const AspectRatioScreen()),
  _NavEntry(group: 'Layout', label: 'ScrollArea', screen: const ScrollAreaScreen()),
  // Navigation
  _NavEntry(group: 'Navigation', label: 'BottomNav', screen: const BottomNavScreen()),
  _NavEntry(group: 'Navigation', label: 'Pagination', screen: const PaginationScreen()),
  _NavEntry(group: 'Navigation', label: 'Breadcrumb', screen: const BreadcrumbScreen()),
  _NavEntry(group: 'Navigation', label: 'Tabs', screen: const TabsScreen()),
  _NavEntry(group: 'Navigation', label: 'NavigationMenu', screen: const NavigationMenuScreen()),
  // Disclosure
  _NavEntry(group: 'Disclosure', label: 'Accordion', screen: const AccordionScreen()),
  _NavEntry(group: 'Disclosure', label: 'Collapsible', screen: const CollapsibleScreen()),
  // Overlays
  _NavEntry(group: 'Overlays', label: 'Dialog', screen: const DialogScreen()),
  _NavEntry(group: 'Overlays', label: 'Popover', screen: const PopoverScreen()),
  _NavEntry(group: 'Overlays', label: 'Tooltip', screen: const TooltipScreen()),
  _NavEntry(group: 'Overlays', label: 'DropdownMenu', screen: const DropdownMenuScreen()),
  _NavEntry(group: 'Overlays', label: 'HoverCard', screen: const HoverCardScreen()),
  _NavEntry(group: 'Overlays', label: 'ContextMenu', screen: const ContextMenuScreen()),
  _NavEntry(group: 'Overlays', label: 'Menubar', screen: const MenubarScreen()),
  _NavEntry(group: 'Overlays', label: 'Command', screen: const CommandScreen()),
  _NavEntry(group: 'Overlays', label: 'Drawer', screen: const DrawerScreen()),
  _NavEntry(group: 'Overlays', label: 'Sheet', screen: const SheetScreen()),
  _NavEntry(group: 'Overlays', label: 'AlertDialog', screen: const AlertDialogScreen()),
  // Motion
  _NavEntry(group: 'Motion', label: 'Animate', screen: const AnimateScreen()),
  _NavEntry(group: 'Motion', label: 'Marquee', screen: const MarqueeScreen()),
  _NavEntry(group: 'Motion', label: 'Pressable', screen: const PressableScreen()),
  _NavEntry(group: 'Motion', label: 'Shimmer', screen: const ShimmerScreen()),
  _NavEntry(group: 'Motion', label: 'Skeleton', screen: const SkeletonScreen()),
  _NavEntry(group: 'Motion', label: 'AnimateGroup', screen: const AnimateGroupScreen()),
  // Media
  _NavEntry(group: 'Media', label: 'Carousel', screen: const CarouselScreen()),
  _NavEntry(group: 'Media', label: 'CardCarousel', screen: const CardCarouselScreen()),
  // Interaction
  _NavEntry(group: 'Interaction', label: 'ThemeToggle', screen: const ThemeToggleScreen()),
  _NavEntry(group: 'Interaction', label: 'DirectionProvider', screen: const DirectionProviderScreen()),
  _NavEntry(group: 'Interaction', label: 'DragAndDrop', screen: const DragAndDropScreen()),
  _NavEntry(group: 'Interaction', label: 'Dropzone', screen: const DropzoneScreen()),
  // Charts
  _NavEntry(group: 'Charts', label: 'Area Chart', screen: const AreaChartScreen()),
  _NavEntry(group: 'Charts', label: 'Bar Chart', screen: const BarChartScreen()),
  _NavEntry(group: 'Charts', label: 'Line Chart', screen: const LineChartScreen()),
  _NavEntry(group: 'Charts', label: 'Pie Chart', screen: const PieChartScreen()),
  _NavEntry(group: 'Charts', label: 'Radar Chart', screen: const RadarChartScreen()),
  _NavEntry(group: 'Charts', label: 'Radial Chart', screen: const RadialChartScreen()),
  // Blocks
  _NavEntry(group: 'Blocks', label: 'Login', screen: const LoginBlockScreen()),
  _NavEntry(group: 'Blocks', label: 'Sidenav', screen: const SidenavBlockScreen()),
  _NavEntry(group: 'Blocks', label: 'Header', screen: const HeaderBlockScreen()),
  _NavEntry(group: 'Blocks', label: 'Footer', screen: const FooterBlockScreen()),
  _NavEntry(group: 'Blocks', label: 'FAQ', screen: const FaqBlockScreen()),
  _NavEntry(group: 'Blocks', label: 'Integrations', screen: const IntegrationsBlockScreen()),
  // Screens
  _NavEntry(group: 'Screens', label: 'Login Screen', screen: const LoginScreenDemo()),
  _NavEntry(group: 'Screens', label: 'Dashboard', screen: const DashboardScreenDemo()),
  _NavEntry(group: 'Screens', label: 'Mobile App', screen: const MobileAppScreenDemo()),
  _NavEntry(group: 'Screens', label: 'Settings', screen: const SettingsScreenDemo()),
];

class AppShell extends StatefulWidget {
  final ThemeMode themeMode;
  final VoidCallback onToggleTheme;

  const AppShell({super.key, required this.themeMode, required this.onToggleTheme});

  @override
  State<AppShell> createState() => _AppShellState();
}

class _AppShellState extends State<AppShell> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);
    final isDark = widget.themeMode == ThemeMode.dark;

    return LayoutBuilder(builder: (context, constraints) {
      final isWide = constraints.maxWidth >= 720;

      final sidebarContent = _SidebarContent(
        selectedIndex: _selectedIndex,
        onSelect: (i) => setState(() => _selectedIndex = i),
        isDark: isDark,
        onToggleTheme: widget.onToggleTheme,
      );

      if (isWide) {
        return Scaffold(
          backgroundColor: c.background,
          body: Row(
            children: [
              Container(
                width: 240,
                decoration: BoxDecoration(
                  color: c.card,
                  border: Border(right: BorderSide(color: c.border)),
                ),
                child: sidebarContent,
              ),
              Expanded(
                child: MediaQuery.removePadding(
                  context: context,
                  removeTop: true,
                  child: _navEntries[_selectedIndex].screen,
                ),
              ),
            ],
          ),
        );
      } else {
        return Scaffold(
          backgroundColor: c.background,
          appBar: AppBar(
            toolbarHeight: 48,
            leading: Builder(builder: (ctx) => IconButton(
              icon: const Icon(LucideIcons.menu, size: 20),
              onPressed: () => Scaffold.of(ctx).openDrawer(),
            )),
            title: const Text(
              'soma_ui',
              style: TextStyle(fontFamily: 'Rajdhani', fontWeight: FontWeight.w700, fontSize: 18),
            ),
            actions: [
              Padding(
                padding: const EdgeInsets.only(right: 8),
                child: SomaThemeToggle(
                  isDark: isDark,
                  onChanged: (_) => widget.onToggleTheme(),
                ),
              ),
            ],
          ),
          drawer: Drawer(
            width: 240,
            backgroundColor: c.card,
            child: SafeArea(child: sidebarContent),
          ),
          body: _navEntries[_selectedIndex].screen,
        );
      }
    });
  }
}

class _SidebarContent extends StatefulWidget {
  final int selectedIndex;
  final ValueChanged<int> onSelect;
  final bool isDark;
  final VoidCallback onToggleTheme;

  const _SidebarContent({
    required this.selectedIndex,
    required this.onSelect,
    required this.isDark,
    required this.onToggleTheme,
  });

  @override
  State<_SidebarContent> createState() => _SidebarContentState();
}

class _SidebarContentState extends State<_SidebarContent> {
  late Map<String, bool> _sectionOpen;

  @override
  void initState() {
    super.initState();
    final selectedGroup = _navEntries[widget.selectedIndex].group;
    _sectionOpen = {for (final g in _groupOrder) g: g == selectedGroup};
  }

  @override
  void didUpdateWidget(_SidebarContent old) {
    super.didUpdateWidget(old);
    if (old.selectedIndex != widget.selectedIndex) {
      final newGroup = _navEntries[widget.selectedIndex].group;
      if (newGroup.isNotEmpty && _sectionOpen[newGroup] != true) {
        setState(() => _sectionOpen[newGroup] = true);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    // Determine active groups (those with entries)
    final activeGroups = _groupOrder.where((g) => _navEntries.any((e) => e.group == g)).toList();

    // Top-level entries
    final topLevelEntries = _navEntries.where((e) => e.topLevel).toList();

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        // Header
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 16, 12, 12),
          child: Row(
            children: [
              Text(
                'soma_ui',
                style: TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 18,
                  fontWeight: FontWeight.w700,
                  color: c.foreground,
                ),
              ),
              const SizedBox(width: 8),
              Container(
                padding: const EdgeInsets.fromLTRB(6, 2, 6, 2),
                decoration: BoxDecoration(
                  color: c.primary,
                  borderRadius: BorderRadius.circular(4),
                ),
                child: Text(
                  '0.1',
                  style: TextStyle(
                    fontFamily: 'monospace',
                    fontSize: 11,
                    color: c.primaryForeground,
                  ),
                ),
              ),
              const Spacer(),
              SomaThemeToggle(isDark: widget.isDark, onChanged: (_) => widget.onToggleTheme()),
            ],
          ),
        ),
        Divider(color: c.border, height: 1),
        // Scrollable nav
        Expanded(
          child: SingleChildScrollView(
            padding: const EdgeInsets.fromLTRB(16, 12, 16, 16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Top-level links
                for (final entry in topLevelEntries)
                  _NavItem(
                    label: entry.label,
                    selected: _navEntries.indexOf(entry) == widget.selectedIndex,
                    colors: c,
                    onTap: () => widget.onSelect(_navEntries.indexOf(entry)),
                  ),
                const SizedBox(height: 20),
                // Grouped sections
                for (int gi = 0; gi < activeGroups.length; gi++) ...[
                  if (gi > 0) const SizedBox(height: 20),
                  _NavSection(
                    title: activeGroups[gi],
                    isOpen: _sectionOpen[activeGroups[gi]] ?? false,
                    onToggle: () => setState(() {
                      _sectionOpen[activeGroups[gi]] = !(_sectionOpen[activeGroups[gi]] ?? false);
                    }),
                    children: [
                      for (int i = 0; i < _navEntries.length; i++)
                        if (_navEntries[i].group == activeGroups[gi])
                          _NavItem(
                            label: _navEntries[i].label,
                            selected: i == widget.selectedIndex,
                            colors: c,
                            onTap: () => widget.onSelect(i),
                          ),
                    ],
                  ),
                ],
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class _NavSection extends StatelessWidget {
  final String title;
  final bool isOpen;
  final VoidCallback onToggle;
  final List<Widget> children;

  const _NavSection({
    required this.title,
    required this.isOpen,
    required this.onToggle,
    required this.children,
  });

  @override
  Widget build(BuildContext context) {
    final c = SomaTheme.of(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        // Section header
        GestureDetector(
          onTap: onToggle,
          child: MouseRegion(
            cursor: SystemMouseCursors.click,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
              child: Row(
                children: [
                  Text(
                    title.toUpperCase(),
                    style: TextStyle(
                      fontFamily: 'Rajdhani',
                      fontSize: 12,
                      fontWeight: FontWeight.w600,
                      letterSpacing: 1.5,
                      color: c.mutedForeground,
                    ),
                  ),
                  const Spacer(),
                  AnimatedRotation(
                    turns: isOpen ? 0 : -0.25,
                    duration: const Duration(milliseconds: 150),
                    child: Icon(LucideIcons.chevronDown, size: 14, color: c.mutedForeground),
                  ),
                ],
              ),
            ),
          ),
        ),
        // Section body
        AnimatedSize(
          duration: const Duration(milliseconds: 150),
          clipBehavior: Clip.hardEdge,
          child: isOpen
              ? Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: children,
                )
              : const SizedBox.shrink(),
        ),
      ],
    );
  }
}

class _NavItem extends StatefulWidget {
  final String label;
  final bool selected;
  final SomaColors colors;
  final VoidCallback onTap;

  const _NavItem({
    required this.label,
    required this.selected,
    required this.colors,
    required this.onTap,
  });

  @override
  State<_NavItem> createState() => _NavItemState();
}

class _NavItemState extends State<_NavItem> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final c = widget.colors;
    final bg = widget.selected
        ? c.accent
        : (_hovered ? c.accent : Colors.transparent);
    final fg = widget.selected ? c.foreground : c.mutedForeground;

    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        onTap: () {
          widget.onTap();
          if (Navigator.canPop(context)) Navigator.pop(context);
        },
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 100),
          margin: const EdgeInsets.symmetric(vertical: 1),
          padding: EdgeInsets.fromLTRB(widget.selected ? 10 : 12, 8, 12, 8),
          decoration: BoxDecoration(
            color: bg,
            borderRadius: BorderRadius.circular(6),
            border: widget.selected
                ? Border(left: BorderSide(color: c.primary, width: 2))
                : null,
          ),
          child: Text(
            widget.label,
            style: TextStyle(
              fontFamily: 'Outfit',
              fontSize: 14,
              fontWeight: widget.selected ? FontWeight.w600 : FontWeight.w400,
              color: fg,
            ),
          ),
        ),
      ),
    );
  }
}
