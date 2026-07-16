use crate::i18n::{strings, Locale};
use leptos::prelude::*;
use soma_ui::icons::{icondata, Icon};

// ponytail: curated ~120-icon subset of the full Lucide catalog (1 600+ icons).
// Ceiling: only what's listed here appears in the gallery.
// Upgrade path: add more (&str, icondata::Icon) pairs, or codegen the full
// list from `icondata_lu` at build time.
fn all_icons() -> Vec<(&'static str, icondata::Icon)> {
    vec![
        // Arrows
        ("ArrowUp", icondata::LuArrowUp),
        ("ArrowDown", icondata::LuArrowDown),
        ("ArrowLeft", icondata::LuArrowLeft),
        ("ArrowRight", icondata::LuArrowRight),
        ("ArrowUpLeft", icondata::LuArrowUpLeft),
        ("ArrowUpRight", icondata::LuArrowUpRight),
        ("ArrowDownLeft", icondata::LuArrowDownLeft),
        ("ArrowDownRight", icondata::LuArrowDownRight),
        ("ArrowUpDown", icondata::LuArrowUpDown),
        ("ArrowLeftRight", icondata::LuArrowLeftRight),
        // Chevrons
        ("ChevronUp", icondata::LuChevronUp),
        ("ChevronDown", icondata::LuChevronDown),
        ("ChevronLeft", icondata::LuChevronLeft),
        ("ChevronRight", icondata::LuChevronRight),
        ("ChevronsUp", icondata::LuChevronsUp),
        ("ChevronsDown", icondata::LuChevronsDown),
        ("ChevronsLeft", icondata::LuChevronsLeft),
        ("ChevronsRight", icondata::LuChevronsRight),
        // Actions / UI
        ("Check", icondata::LuCheck),
        ("X", icondata::LuX),
        ("Plus", icondata::LuPlus),
        ("Minus", icondata::LuMinus),
        ("Search", icondata::LuSearch),
        ("Menu", icondata::LuMenu),
        ("Grip", icondata::LuGrip),
        ("Ellipsis", icondata::LuEllipsis),
        ("EllipsisVertical", icondata::LuEllipsisVertical),
        ("RefreshCw", icondata::LuRefreshCw),
        ("RotateCw", icondata::LuRotateCw),
        ("RotateCcw", icondata::LuRotateCcw),
        ("ExternalLink", icondata::LuExternalLink),
        ("Link", icondata::LuLink),
        ("Share", icondata::LuShare),
        ("Copy", icondata::LuCopy),
        ("Clipboard", icondata::LuClipboard),
        ("ClipboardCheck", icondata::LuClipboardCheck),
        ("Download", icondata::LuDownload),
        ("Upload", icondata::LuUpload),
        ("Send", icondata::LuSend),
        ("ListFilter", icondata::LuListFilter),
        ("Maximize", icondata::LuMaximize),
        ("Minimize", icondata::LuMinimize),
        // Navigation / Layout
        ("Home", icondata::LuHouse),
        ("LayoutDashboard", icondata::LuLayoutDashboard),
        ("PanelLeft", icondata::LuPanelLeft),
        ("PanelRight", icondata::LuPanelRight),
        ("List", icondata::LuList),
        ("Grid2x2", icondata::LuGrid2x2),
        ("Grid3x3", icondata::LuGrid3x3),
        ("Columns2", icondata::LuColumns2),
        ("MoveHorizontal", icondata::LuMoveHorizontal),
        // People
        ("User", icondata::LuUser),
        ("Users", icondata::LuUsers),
        // Communication
        ("Mail", icondata::LuMail),
        ("Phone", icondata::LuPhone),
        ("Bell", icondata::LuBell),
        ("BellOff", icondata::LuBellOff),
        ("Globe", icondata::LuGlobe),
        // Files & Folders
        ("File", icondata::LuFile),
        ("FileText", icondata::LuFileText),
        ("FileCode", icondata::LuFileCode),
        ("FilePlus", icondata::LuFilePlus),
        ("FileMinus", icondata::LuFileMinus),
        ("FilePen", icondata::LuFilePen),
        ("Folder", icondata::LuFolder),
        ("Package", icondata::LuPackage),
        ("Save", icondata::LuSave),
        ("Archive", icondata::LuArchive),
        ("Book", icondata::LuBook),
        ("BookOpen", icondata::LuBookOpen),
        ("Bookmark", icondata::LuBookmark),
        ("Tag", icondata::LuTag),
        // Editing
        ("Pencil", icondata::LuPencil),
        ("Pen", icondata::LuPen),
        ("Trash", icondata::LuTrash),
        ("Trash2", icondata::LuTrash2),
        ("Scissors", icondata::LuScissors),
        // Media
        ("Image", icondata::LuImage),
        ("Camera", icondata::LuCamera),
        ("Video", icondata::LuVideo),
        ("Mic", icondata::LuMic),
        ("Play", icondata::LuPlay),
        ("Pause", icondata::LuPause),
        ("Volume", icondata::LuVolume),
        ("Volume2", icondata::LuVolume2),
        ("VolumeX", icondata::LuVolumeX),
        // Status / Feedback
        ("Star", icondata::LuStar),
        ("Heart", icondata::LuHeart),
        ("Flag", icondata::LuFlag),
        ("Zap", icondata::LuZap),
        ("Info", icondata::LuInfo),
        ("Circle", icondata::LuCircle),
        ("Square", icondata::LuSquare),
        ("Loader", icondata::LuLoader),
        ("Shield", icondata::LuShield),
        ("ShieldCheck", icondata::LuShieldCheck),
        // Theme / Settings
        ("Sun", icondata::LuSun),
        ("Moon", icondata::LuMoon),
        ("Cloud", icondata::LuCloud),
        ("Settings", icondata::LuSettings),
        ("SlidersHorizontal", icondata::LuSlidersHorizontal),
        ("Command", icondata::LuCommand),
        ("ToggleLeft", icondata::LuToggleLeft),
        ("ToggleRight", icondata::LuToggleRight),
        ("Eye", icondata::LuEye),
        ("EyeOff", icondata::LuEyeOff),
        ("Lock", icondata::LuLock),
        ("LockOpen", icondata::LuLockOpen),
        ("Key", icondata::LuKey),
        ("LogIn", icondata::LuLogIn),
        ("LogOut", icondata::LuLogOut),
        // Time & Calendar
        ("Calendar", icondata::LuCalendar),
        ("Clock", icondata::LuClock),
        ("Map", icondata::LuMap),
        // Tech
        ("Code", icondata::LuCode),
        ("Terminal", icondata::LuTerminal),
        ("Database", icondata::LuDatabase),
        ("Server", icondata::LuServer),
        ("Cpu", icondata::LuCpu),
        ("Monitor", icondata::LuMonitor),
        ("Laptop", icondata::LuLaptop),
        ("Smartphone", icondata::LuSmartphone),
        ("Tablet", icondata::LuTablet),
        ("HardDrive", icondata::LuHardDrive),
        ("Wifi", icondata::LuWifi),
        ("Bluetooth", icondata::LuBluetooth),
        // Social
        ("Github", icondata::LuGithub),
        ("Twitter", icondata::LuTwitter),
        ("Linkedin", icondata::LuLinkedin),
        ("Instagram", icondata::LuInstagram),
    ]
}

#[component]
pub fn IconsPage() -> impl IntoView {
    let locale = use_context::<RwSignal<Locale>>().expect("locale context");
    let s = move || strings(locale.get());
    let query = RwSignal::new(String::new());

    let icons = all_icons();

    let filtered = move || {
        let q = query.get().to_lowercase();
        icons
            .iter()
            .filter(|(name, _)| q.is_empty() || name.to_lowercase().contains(&q))
            .cloned()
            .collect::<Vec<_>>()
    };

    view! {
        <div class="max-w-5xl space-y-8">
            <div>
                <h1 class="font-heading text-3xl font-bold tracking-tight text-foreground">{move || s().icons_title}</h1>
                <p class="text-sm text-muted-foreground mt-1">{move || s().icons_subtitle}</p>
            </div>

            // Search
            <div class="bg-card border border-border rounded-md p-4 flex items-center gap-3">
                <Icon icon=Signal::derive(|| icondata::LuSearch) width="16" height="16" style="color: var(--color-muted-foreground)" />
                <input
                    type="text"
                    class="flex-1 bg-transparent text-sm text-foreground placeholder:text-muted-foreground outline-none"
                    placeholder=move || s().icons_search_placeholder
                    on:input=move |e| query.set(event_target_value(&e))
                />
            </div>

            // Grid
            <div class="bg-card border border-border rounded-md p-6">
                <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-4">
                    {move || {
                        let items = filtered();
                        if items.is_empty() {
                            view! {
                                <p class="col-span-full text-sm text-muted-foreground text-center py-8">"No icons match your search."</p>
                            }.into_any()
                        } else {
                            items.into_iter().map(|(name, icon)| {
                                view! {
                                    <div class="flex flex-col items-center gap-2 p-3 rounded-md hover:bg-accent transition-colors">
                                        <Icon
                                            icon=Signal::derive(move || icon)
                                            width="24"
                                            height="24"
                                            style="color: var(--color-foreground)"
                                        />
                                        <span class="text-xs text-muted-foreground text-center leading-tight break-all">{name}</span>
                                    </div>
                                }
                            }).collect::<Vec<_>>().into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
