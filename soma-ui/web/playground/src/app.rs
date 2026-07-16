use crate::i18n::{dir, Locale};
use crate::layout::sidebar::Sidebar;
use crate::pages::*;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub use crate::viewport::ViewportCanvas;

#[component]
pub fn App() -> impl IntoView {
    let locale = RwSignal::new(Locale::En);
    provide_context(locale);
    let _ = js_sys::eval(&format!(
        "document.documentElement.dir='{}'",
        dir(Locale::En)
    ));

    view! {
        <Router>
            <div class="flex min-h-screen bg-background">
                <Sidebar />
                <main class="flex-1 p-4 md:p-8 overflow-auto w-full">
                    <Routes fallback=|| view! { <p class="text-muted-foreground">"Not found"</p> }>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/theme") view=ThemePage />
                        <Route path=path!("/hooks") view=HooksPage />
                        <Route path=path!("/components/button") view=ButtonPage />
                        <Route path=path!("/components/badge") view=BadgePage />
                        <Route path=path!("/components/card") view=CardPage />
                        <Route path=path!("/components/input") view=InputPage />
                        <Route path=path!("/components/animate") view=AnimatePage />
                        <Route path=path!("/components/separator") view=SeparatorPage />
                        <Route path=path!("/icons") view=IconsPage />
                        <Route path=path!("/components/skeleton") view=SkeletonPage />
                        <Route path=path!("/components/spinner") view=SpinnerPage />
                        <Route path=path!("/components/alert") view=AlertPage />
                        <Route path=path!("/components/callout") view=CalloutPage />
                        <Route path=path!("/components/progress") view=ProgressPage />
                        <Route path=path!("/components/status") view=StatusPage />
                        <Route path=path!("/components/avatar") view=AvatarPage />
                        <Route path=path!("/components/kbd") view=KbdPage />
                        <Route path=path!("/components/chip") view=ChipPage />
                        <Route path=path!("/components/empty") view=EmptyPage />
                        <Route path=path!("/components/item") view=ItemPage />
                        <Route path=path!("/components/breadcrumb") view=BreadcrumbNavPage />
                        <Route path=path!("/components/label") view=LabelPage />
                        <Route path=path!("/components/field") view=FieldPage />
                        <Route path=path!("/components/form") view=FormPage />
                        <Route path=path!("/components/auto-form") view=AutoFormPage />
                        <Route path=path!("/components/aspect-ratio") view=AspectRatioPage />
                        <Route path=path!("/components/scroll-area") view=ScrollAreaPage />
                        <Route path=path!("/components/textarea") view=TextareaPage />
                        <Route path=path!("/components/animate-group") view=AnimateGroupPage />
                        <Route path=path!("/components/marquee") view=MarqueePage />
                        <Route path=path!("/components/pressable") view=PressablePage />
                        <Route path=path!("/components/shimmer") view=ShimmerPage />
                        <Route path=path!("/components/checkbox") view=CheckboxPage />
                        <Route path=path!("/components/switch") view=SwitchPage />
                        <Route path=path!("/components/slider") view=SliderPage />
                        <Route path=path!("/components/radio-button") view=RadioButtonPage />
                        <Route path=path!("/components/radio-button-group") view=RadioButtonGroupPage />
                        <Route path=path!("/components/toggle-group") view=ToggleGroupPage />
                        <Route path=path!("/components/button-group") view=ButtonGroupPage />
                        <Route path=path!("/components/button-action") view=ButtonActionPage />
                        <Route path=path!("/components/input-group") view=InputGroupPage />
                        <Route path=path!("/components/date-picker") view=DatePickerPage />
                        <Route path=path!("/components/input-otp") view=InputOtpPage />
                        <Route path=path!("/components/input-phone") view=InputPhonePage />
                        <Route path=path!("/components/input-prompt") view=InputPromptPage />
                        <Route path=path!("/components/select") view=SelectPage />
                        <Route path=path!("/components/combobox") view=ComboboxPage />
                        <Route path=path!("/components/multi-select") view=MultiSelectPage />
                        <Route path=path!("/components/accordion") view=AccordionPage />
                        <Route path=path!("/components/collapsible") view=CollapsiblePage />
                        <Route path=path!("/components/tabs") view=TabsPage />
                        <Route path=path!("/components/pagination") view=PaginationPage />
                        <Route path=path!("/components/bottom-nav") view=BottomNavPage />
                        <Route path=path!("/components/table") view=TablePage />
                        <Route path=path!("/components/data-table") view=DataTablePage />
                        <Route path=path!("/components/data-grid") view=DataGridPage />
                        <Route path=path!("/components/code-block") view=CodeBlockPage />
                        <Route path=path!("/components/timeline") view=TimelinePage />
                        <Route path=path!("/components/comparison-table") view=ComparisonTablePage />
                        <Route path=path!("/components/schema-diagram") view=SchemaDiagramPage />
                        <Route path=path!("/components/command") view=CommandPage />
                        <Route path=path!("/components/dialog") view=DialogPage />
                        <Route path=path!("/components/alert-dialog") view=AlertDialogPage />
                        <Route path=path!("/components/drawer") view=DrawerPage />
                        <Route path=path!("/components/sheet") view=SheetPage />
                        <Route path=path!("/components/popover") view=PopoverPage />
                        <Route path=path!("/components/tooltip") view=TooltipPage />
                        <Route path=path!("/components/hover-card") view=HoverCardPage />
                        <Route path=path!("/components/dropdown-menu") view=DropdownMenuPage />
                        <Route path=path!("/components/context-menu") view=ContextMenuPage />
                        <Route path=path!("/components/menubar") view=MenubarPage />
                        <Route path=path!("/components/toast") view=ToastPage />
                        <Route path=path!("/components/sonner") view=SonnerPage />
                        <Route path=path!("/components/theme-toggle") view=ThemeTogglePage />
                        <Route path=path!("/components/direction-provider") view=DirectionProviderPage />
                        <Route path=path!("/components/carousel") view=CarouselPage />
                        <Route path=path!("/components/card-carousel") view=CardCarouselPage />
                        <Route path=path!("/components/navigation-menu") view=NavigationMenuPage />
                        <Route path=path!("/components/sidebar") view=SidebarPage />
                        <Route path=path!("/components/stat") view=StatPage />
                        <Route path=path!("/components/page-header") view=PageHeaderPage />
                        <Route path=path!("/components/drag-and-drop") view=DragAndDropPage />
                        <Route path=path!("/components/dropzone") view=DropzonePage />
                        <Route path=path!("/charts/area") view=AreaChartPage />
                        <Route path=path!("/charts/bar") view=BarChartPage />
                        <Route path=path!("/charts/heatmap") view=HeatmapPage />
                        <Route path=path!("/charts/line") view=LineChartPage />
                        <Route path=path!("/charts/pie") view=PieChartPage />
                        <Route path=path!("/charts/radar") view=RadarChartPage />
                        <Route path=path!("/charts/radial") view=RadialChartPage />
                        <Route path=path!("/charts/sparkline") view=SparklinePage />
                        <Route path=path!("/charts/stat-tile") view=StatTilePage />
                        <Route path=path!("/blocks/login") view=LoginBlockPage />
                        <Route path=path!("/blocks/sidenav") view=SidenavBlockPage />
                        <Route path=path!("/blocks/headers") view=HeaderBlockPage />
                        <Route path=path!("/blocks/footers") view=FooterBlockPage />
                        <Route path=path!("/blocks/faq") view=FaqBlockPage />
                        <Route path=path!("/blocks/integrations") view=IntegrationsBlockPage />
                        <Route path=path!("/blocks/hero") view=HeroPage />
                        <Route path=path!("/blocks/feature-grid") view=FeatureGridPage />
                        <Route path=path!("/screens/login") view=LoginScreenPage />
                        <Route path=path!("/screens/dashboard") view=DashboardScreenPage />
                        <Route path=path!("/screens/mobile") view=MobileAppScreenPage />
                        <Route path=path!("/screens/settings") view=SettingsScreenPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
