# swift-runtime-sys

Build native Apple apps from Rust. SwiftUI, RealityKit, Combine, persistence — all with a declarative DSL, reactive state, and 100% pixel parity.

```rust
use swiftui::prelude::*;

fn main() {
    app("My App", 400.0, 300.0, |cx| {
        let count = cx.state(0i32);
        vstack![
            text_fmt!("Count: {count}").bold().size(48.0),
            button("+1", count.increment()),
            button("Reset", count.set_to(0)),
        ].style(Page)
    });
}
```

```bash
git clone https://github.com/eugenehp/swift-runtime-sys
cd swift-runtime-sys
cargo run -p swiftui --example showcase
```

Zero setup — `build.rs` auto-compiles the Swift helper and auto-detects your SDK.

## Crates

### Core

| Crate | Description |
|-------|-------------|
| [`swift-runtime-sys`](crates/swift-runtime-sys) | Raw FFI to the Swift runtime (490+ symbols, arm64 asm thunks) |
| [`swift-runtime`](crates/swift-runtime) | Safe Rust wrappers (Metadata, types, Retained) |
| [`swiftui-sys`](crates/swiftui-sys) | Raw SwiftUI FFI (dlsym function pointers) |
| [`swiftui-macros`](crates/swiftui-macros) | Proc macros (`text_fmt!`, `#[derive(View)]`) |
| [`swiftui`](crates/swiftui) | SwiftUI DSL — 142/142 API coverage |
| [`realitykit-sys`](crates/realitykit-sys) | Raw RealityKit FFI |
| [`realitykit`](crates/realitykit) | RealityKit 3D scene builder |
| [`apple-sys-helpers`](crates/apple-sys-helpers) | Shared dlsym helpers and `apple_framework!` macro |
| [`swift-bridge-gen`](crates/swift-bridge-gen) | Auto-generate bindings for any Apple framework |

### Apple Frameworks (158 crates)

Every Apple framework with a Swift module is covered. Crates marked ⚡ have API beyond availability checking.

<details>
<summary>Full framework list</summary>

| Crate | Description |
|-------|-------------|
| [`accessibility`](crates/accessibility) | Accessibility — assistive technology support from Rust |
| [`accessorysetupkit`](crates/accessorysetupkit) | AccessorySetupKit — accessory pairing from Rust |
| [`accessorytransportextension`](crates/accessorytransportextension) | AccessoryTransportExtension — accessory transport from Rust |
| [`activitykit`](crates/activitykit) | ActivityKit — Live Activities and Dynamic Island from Rust |
| [`adattributionkit`](crates/adattributionkit) | AdAttributionKit — ad attribution from Rust |
| [`adservices`](crates/adservices) | AdServices — ad attribution from Rust |
| [`adsupport`](crates/adsupport) | AdSupport — advertising identifier from Rust |
| [`alarmkit`](crates/alarmkit) | AlarmKit — alarm management from Rust |
| [`appintents`](crates/appintents) | AppIntents — Siri shortcuts and Spotlight integration from Rust |
| [`appmigrationkit`](crates/appmigrationkit) | AppMigrationKit — app migration utilities from Rust |
| [`apptrackingtransparency`](crates/apptrackingtransparency) | AppTrackingTransparency — tracking permission from Rust |
| [`arkit`](crates/arkit) | ARKit — augmented reality from Rust |
| [`assignables`](crates/assignables) | Assignables — education assignment management from Rust |
| [`authenticationservices`](crates/authenticationservices) | AuthenticationServices — Sign in with Apple from Rust |
| [`automateddeviceenrollment`](crates/automateddeviceenrollment) | AutomatedDeviceEnrollment — MDM enrollment from Rust |
| [`automaticassessmentconfiguration`](crates/automaticassessmentconfiguration) | AutomaticAssessmentConfiguration — exam lockdown from Rust |
| [`avfaudio`](crates/avfaudio) | AVFAudio — audio playback, recording, and processing from Rust |
| [`avfoundation`](crates/avfoundation) | AVFoundation — media capture, playback, and editing from Rust |
| [`avkit`](crates/avkit) | AVKit — media playback UI from Rust |
| [`backgroundassets`](crates/backgroundassets) | BackgroundAssets — background asset downloads from Rust |
| [`backgroundtasks`](crates/backgroundtasks) | BackgroundTasks — background work scheduling from Rust |
| [`browserenginecore`](crates/browserenginecore) | BrowserEngineCore — browser engine hosting from Rust |
| [`browserenginekit`](crates/browserenginekit) | BrowserEngineKit — browser engine integration from Rust |
| [`browserkit`](crates/browserkit) | BrowserKit — browser kit from Rust |
| [`callkit`](crates/callkit) | CallKit — VoIP call integration from Rust |
| [`carkey`](crates/carkey) | CarKey — digital car keys from Rust |
| [`charts`](crates/charts) | Swift Charts — data visualization from Rust ⚡ |
| [`cinematic`](crates/cinematic) | Cinematic — cinematic video processing from Rust |
| [`clockkit`](crates/clockkit) | ClockKit — watchOS complications from Rust (deprecated) |
| [`cloudkit`](crates/cloudkit) | CloudKit — iCloud database from Rust |
| [`combine`](crates/combine) | Combine publisher/subscriber bridge from Rust ⚡ |
| [`compositorservices`](crates/compositorservices) | CompositorServices — visionOS rendering from Rust |
| [`contactprovider`](crates/contactprovider) | ContactProvider — contact provider extensions from Rust |
| [`contacts`](crates/contacts) | Contacts — address book access from Rust |
| [`corebluetooth`](crates/corebluetooth) | Core Bluetooth — BLE from Rust |
| [`coredata`](crates/coredata) | Core Data — persistent object graph from Rust |
| [`coregraphics`](crates/coregraphics) | Core Graphics — 2D drawing from Rust |
| [`corehaptics`](crates/corehaptics) | Core Haptics — haptic feedback from Rust |
| [`corehid`](crates/corehid) | CoreHID — USB and Bluetooth HID devices from Rust |
| [`coreimage`](crates/coreimage) | Core Image — image processing and filters from Rust |
| [`corelocation`](crates/corelocation) | Core Location — GPS and location services from Rust |
| [`coreml`](crates/coreml) | Core ML — on-device machine learning from Rust |
| [`coremotion`](crates/coremotion) | Core Motion — accelerometer and gyroscope from Rust |
| [`corenfc`](crates/corenfc) | Core NFC — NFC tag reading and writing from Rust |
| [`corespotlight`](crates/corespotlight) | Core Spotlight — search indexing from Rust |
| [`coretext`](crates/coretext) | Core Text — text layout and font handling from Rust |
| [`coretransferable`](crates/coretransferable) | CoreTransferable — drag-and-drop and sharing from Rust |
| [`createml`](crates/createml) | Create ML — train machine learning models from Rust |
| [`cryptokit`](crates/cryptokit) | CryptoKit — cryptography from Rust |
| [`cryptotokenkit`](crates/cryptotokenkit) | CryptoTokenKit — smart cards and crypto tokens from Rust |
| [`datadetection`](crates/datadetection) | DataDetection — structured data extraction from Rust |
| [`declaredagerange`](crates/declaredagerange) | DeclaredAgeRange — age range declaration from Rust |
| [`deviceactivity`](crates/deviceactivity) | DeviceActivity — Screen Time monitoring from Rust |
| [`devicecheck`](crates/devicecheck) | DeviceCheck — device attestation from Rust |
| [`devicediscoveryextension`](crates/devicediscoveryextension) | DeviceDiscoveryExtension — streaming device discovery from Rust |
| [`dockkit`](crates/dockkit) | DockKit — motorized stand control from Rust |
| [`energykit`](crates/energykit) | EnergyKit — energy usage information from Rust |
| [`eventkit`](crates/eventkit) | EventKit — calendar and reminders from Rust |
| [`extensionkit`](crates/extensionkit) | ExtensionKit — app extensions from Rust |
| [`familycontrols`](crates/familycontrols) | FamilyControls — parental controls from Rust |
| [`fileprovider`](crates/fileprovider) | FileProvider — file sync from Rust |
| [`financekit`](crates/financekit) | FinanceKit — financial data from Rust |
| [`foundation-models`](crates/foundation-models) | Intelligence on-device LLM from Rust ⚡ |
| [`fskit`](crates/fskit) | FSKit — file system extensions from Rust |
| [`gamecontroller`](crates/gamecontroller) | GameController — controller input from Rust |
| [`gamekit`](crates/gamekit) | GameKit — Game Center from Rust |
| [`gameplaykit`](crates/gameplaykit) | GameplayKit — game logic from Rust |
| [`gamesave`](crates/gamesave) | GameSave — game save management from Rust |
| [`geotoolbox`](crates/geotoolbox) | GeoToolbox — geographic utilities from Rust |
| [`groupactivities`](crates/groupactivities) | GroupActivities — SharePlay from Rust |
| [`healthkit`](crates/healthkit) | HealthKit — health and fitness data from Rust |
| [`homekit`](crates/homekit) | HomeKit — smart home control from Rust |
| [`identitylookup`](crates/identitylookup) | IdentityLookup — caller ID and message filtering from Rust |
| [`imageplayground`](crates/imageplayground) | Image Playground — AI image generation from Rust |
| [`immersivemediasupport`](crates/immersivemediasupport) | ImmersiveMediaSupport — immersive media playback from Rust |
| [`intents`](crates/intents) | Intents — Siri intents and shortcuts from Rust |
| [`ituneslibrary`](crates/ituneslibrary) | iTunesLibrary — Music library access from Rust |
| [`journalingsuggestions`](crates/journalingsuggestions) | JournalingSuggestions — journaling suggestion picker from Rust |
| [`linkpresentation`](crates/linkpresentation) | LinkPresentation — URL previews from Rust |
| [`livecommunicationkit`](crates/livecommunicationkit) | LiveCommunicationKit — live calling from Rust |
| [`localauthentication`](crates/localauthentication) | LocalAuthentication — biometric auth from Rust |
| [`lockedcameracapture`](crates/lockedcameracapture) | LockedCameraCapture — locked screen camera capture from Rust |
| [`managedapp`](crates/managedapp) | ManagedApp — managed app configuration from Rust |
| [`managedappdistribution`](crates/managedappdistribution) | ManagedAppDistribution — enterprise app distribution from Rust |
| [`managedsettings`](crates/managedsettings) | ManagedSettings — device restrictions from Rust |
| [`mapkit`](crates/mapkit) | MapKit — maps and directions from Rust |
| [`marketplacekit`](crates/marketplacekit) | MarketplaceKit — alternative app marketplace from Rust |
| [`matter`](crates/matter) | Matter — smart home connectivity from Rust |
| [`mediaaccessibility`](crates/mediaaccessibility) | MediaAccessibility — closed captions and audio descriptions from Rust |
| [`mediaextension`](crates/mediaextension) | MediaExtension — media codec extensions from Rust |
| [`mediaplayer`](crates/mediaplayer) | MediaPlayer — music and media playback from Rust |
| [`messages`](crates/messages) | Messages — iMessage app extensions from Rust |
| [`messageui`](crates/messageui) | MessageUI — in-app email and SMS compose from Rust |
| [`metal`](crates/metal) | Metal — GPU programming from Rust ⚡ |
| [`metalkit`](crates/metalkit) | MetalKit — Metal utilities from Rust |
| [`metalperformanceshadersgraph`](crates/metalperformanceshadersgraph) | MPS Graph — GPU ML graph operations from Rust |
| [`metrickit`](crates/metrickit) | MetricKit — app diagnostics from Rust |
| [`mlcompute`](crates/mlcompute) | MLCompute — ML compute operations from Rust (deprecated) |
| [`modelio`](crates/modelio) | Model I/O — 3D model import/export from Rust |
| [`multipeerconnectivity`](crates/multipeerconnectivity) | MultipeerConnectivity — peer-to-peer networking from Rust |
| [`musickit`](crates/musickit) | MusicKit — Apple Music integration from Rust |
| [`naturallanguage`](crates/naturallanguage) | NaturalLanguage — text processing and NLP from Rust |
| [`nearbyinteraction`](crates/nearbyinteraction) | NearbyInteraction — UWB ranging from Rust |
| [`network`](crates/network) | Network framework — modern networking from Rust |
| [`networkextension`](crates/networkextension) | NetworkExtension — VPN and content filtering from Rust |
| [`oslog`](crates/oslog) | OSLog — unified logging from Rust |
| [`paperkit`](crates/paperkit) | PaperKit — paper detection and interaction from Rust |
| [`passkit`](crates/passkit) | PassKit — Wallet and Apple Pay from Rust |
| [`pdfkit`](crates/pdfkit) | PDFKit — PDF viewing and annotation from Rust |
| [`pencilkit`](crates/pencilkit) | PencilKit — drawing and handwriting from Rust |
| [`permissionkit`](crates/permissionkit) | PermissionKit — permission management from Rust |
| [`photos`](crates/photos) | Photos — photo library access from Rust |
| [`photosui`](crates/photosui) | PhotosUI — photo picker from Rust |
| [`proximityreader`](crates/proximityreader) | ProximityReader — Tap to Pay on iPhone from Rust |
| [`pushkit`](crates/pushkit) | PushKit — VoIP and complication push notifications from Rust |
| [`quicklook`](crates/quicklook) | QuickLook — file previews from Rust |
| [`quicklookthumbnailing`](crates/quicklookthumbnailing) | QuickLook Thumbnailing — file thumbnail generation from Rust |
| [`quicklookui`](crates/quicklookui) | QuickLookUI — QuickLook preview panel from Rust |
| [`realityfoundation`](crates/realityfoundation) | RealityFoundation — RealityKit foundation types from Rust |
| [`relevancekit`](crates/relevancekit) | RelevanceKit — relevance engine from Rust |
| [`replaykit`](crates/replaykit) | ReplayKit — screen recording and broadcasting from Rust |
| [`roomplan`](crates/roomplan) | RoomPlan — 3D room scanning with LiDAR from Rust |
| [`safariservices`](crates/safariservices) | SafariServices — in-app browser from Rust |
| [`scenekit`](crates/scenekit) | SceneKit — 3D rendering from Rust |
| [`screencapturekit`](crates/screencapturekit) | ScreenCaptureKit — screen recording from Rust |
| [`secureelementcredential`](crates/secureelementcredential) | SecureElementCredential — secure element credentials from Rust |
| [`sensitivecontentanalysis`](crates/sensitivecontentanalysis) | SensitiveContentAnalysis — CSAM/nudity detection from Rust |
| [`sensorkit`](crates/sensorkit) | SensorKit — research sensor data from Rust |
| [`servicesaccountlinking`](crates/servicesaccountlinking) | ServicesAccountLinking — service account linking from Rust |
| [`sharedwithyou`](crates/sharedwithyou) | SharedWithYou — Messages collaboration from Rust |
| [`shazamkit`](crates/shazamkit) | ShazamKit — music recognition from Rust |
| [`soundanalysis`](crates/soundanalysis) | SoundAnalysis — audio classification from Rust |
| [`spatial`](crates/spatial) | Spatial framework — 3D math types from Rust ⚡ |
| [`speech`](crates/speech) | Speech — speech recognition from Rust |
| [`spritekit`](crates/spritekit) | SpriteKit — 2D game engine from Rust |
| [`stickerkit`](crates/stickerkit) | StickerKit — iMessage sticker packs from Rust |
| [`storekit`](crates/storekit) | StoreKit — in-app purchases and subscriptions from Rust |
| [`swift-data`](crates/swift-data) | Persistent key-value store backed by UserDefaults ⚡ |
| [`swiftdata`](crates/swiftdata) | SwiftData — modern persistence framework from Rust |
| [`symbols`](crates/symbols) | Symbols — SF Symbols metadata from Rust |
| [`tabulardata`](crates/tabulardata) | TabularData — data tables and CSV from Rust |
| [`telephonymessagingkit`](crates/telephonymessagingkit) | TelephonyMessagingKit — telephony messaging from Rust |
| [`tipkit`](crates/tipkit) | TipKit — in-app tips and hints from Rust |
| [`translation`](crates/translation) | Translation framework — on-device text translation from Rust |
| [`uniformtypeidentifiers`](crates/uniformtypeidentifiers) | UniformTypeIdentifiers — UTI system from Rust |
| [`usernotifications`](crates/usernotifications) | UserNotifications — push and local notifications from Rust |
| [`videosubscriberaccount`](crates/videosubscriberaccount) | VideoSubscriberAccount — TV provider authentication from Rust |
| [`virtualization`](crates/virtualization) | Virtualization — virtual machines from Rust |
| [`vision`](crates/vision) | Vision framework — image analysis and computer vision from Rust |
| [`visionkit`](crates/visionkit) | VisionKit — document scanning and visual lookup from Rust |
| [`visualintelligence`](crates/visualintelligence) | VisualIntelligence — visual lookup and intelligence from Rust |
| [`weatherkit`](crates/weatherkit) | WeatherKit — weather data from Rust |
| [`webkit`](crates/webkit) | WebKit — web views from Rust |
| [`widgetkit`](crates/widgetkit) | WidgetKit — reload widget timelines from Rust ⚡ |
| [`wifiaware`](crates/wifiaware) | WiFiAware — WiFi Aware networking from Rust |
| [`wifiinfrastructure`](crates/wifiinfrastructure) | WiFiInfrastructure — WiFi infrastructure management from Rust |
| [`wirelessinsights`](crates/wirelessinsights) | WirelessInsights — wireless diagnostics from Rust |
| [`workoutkit`](crates/workoutkit) | WorkoutKit — workout composition from Rust |

</details>

## SwiftUI (142/142 APIs)

### Views

```rust
text("Hello")                 image("star.fill")        label("Settings", "gear")
button("Click", || {})        toggle("On", true)        slider(0.5, 0.0, 1.0)
textfield("Name", "")         secure_field("Pass", "")  text_editor("Long text")
stepper("Qty", 1, 0, 10)      progress(0.7, 1.0)        link("Rust", "https://...")
color(RED)                    spacer()                  divider()
async_image("https://...")    photos_picker("Photo", |d| {})
map(37.7, -122.4, 0.1, 0.1)  video_player("https://...")
share_link("Share", "url")   empty_view()              content_unavailable(...)
group_box("Title", content)  disclosure_group(...)      labeled_content(...)
date_picker("Date")          color_picker("Color")
```

### Stacks & Layout

```rust
vstack![a, b, c]     hstack![a, b]     zstack![bg, fg]
grid(3, children)    hgrid(2, children)
form(children)       section("Title", children)
list![items]         tabview(vec![Tab::new("Home", "house", view)])
navigation_split_view(sidebar, detail)
```

### Modifiers

```rust
.padding(16.0)    .frame(200.0, 100.0)    .bg(DARK)         .foreground(BLUE)
.rounded(12.0)    .opacity(0.5)           .shadow(...)       .border(GRAY, 1.0)
.offset(10.0, 5.0) .scale(1.5)           .rotation(45.0)    .blur(3.0)
.brightness(0.2)  .saturation(0.5)        .grayscale(1.0)    .color_invert()
.clip_circle()    .clipped()              .mask(view)        .blend_mode(1)
.hidden()         .disabled(true)         .overlay(view)     .overlay_aligned(v, 2)
.font(18.0, Bold) .bold_mod()             .italic_mod()      .line_limit(2)
.truncation_mode(0) .minimum_scale_factor(0.5)               .fixed_size_mod()
.aspect_ratio(1.0, true)                  .tint(BLUE)        .badge(5)
.help("tooltip")  .keyboard_shortcut("r") .focusable()       .drawing_group()
.allows_hit_testing(true)                 .content_shape(0)
.navigation_title("Title")               .navigation_stack() .toolbar(content)
.context_menu(m)  .popover(v, shown)      .sheet(v, shown)   .alert(t, m, shown)
.confirmation_dialog(t, shown, acts)      .searchable(|q| {}) .refreshable(|| {})
.swipe_delete(|| {})                      .swipe_actions(v, leading)
.on_tap(|| {})    .on_long_press(|| {})   .on_drag(|x,y| {}) .on_magnify(|s| {})
.on_rotate(|d| {})                        .on_appear(|| {})   .on_disappear(|| {})
.task(|| {})      .scroll()               .scroll_id("id")
.preferred_color_scheme(true)             .ignores_safe_area()
.safe_area_inset_bottom(v)                .list_row_background(v)
.list_row_separator(false)                .container_relative_frame(0)
.matched_geometry("id")                   .symbol_bounce()    .symbol_pulse()
.animated()       .spring()               .bouncy()           .ease_in(0.3)
.ease_out(0.3)    .ease_in_out(0.3)       .linear(0.5)        .bezier(...)
.spring_params(0.5, 0.3)                  .transition_opacity() .transition_slide()
.phase_animate(3) .phase_animate_scale(&[...])                .keyframe(&[...], t)
.accessibility_label("x") .accessibility_hint("x")            .accessibility_hidden(t)
.style(Elevated)  .styles(&[Title, CardDark])
```

### Reactive State

```rust
app("App", 400.0, 300.0, |cx| {
    let count = cx.state(0i32);
    let name = cx.state("World".into());
    let items = cx.state(vec!["a".into(), "b".into()]);

    vstack![
        text_fmt!("Hello {name}! Count: {count}").style(Title),
        button("+1", count.increment()),
        button("-1", count.decrement()),
        button("Reset", count.set_to(0)),
        button("×2", count.bind(|n| n * 2)),
        bound_textfield("Name", &name),          // two-way binding
        bound_toggle("Agree", &agreed),
        bound_slider(&volume, 0.0, 1.0),
        bound_picker("Sort", &["Name","Date"], &sort),
        bound_color_picker("Color", &r, &g, &b),
        bound_date_picker("Date", &timestamp),
    ]
});

// Animated state changes
animate(|| count.set(0));
animate_spring(|| offset.set(100.0));
with_animation(AnimCurve::EaseInOut, 0.3, || { ... });

// Vec state helpers
items.push("new".into());
items.remove(0);
items.update_at(1, |s| s.to_uppercase());
items.clear();

// Persistence
app_storage_set("theme", "dark");
app_storage_get("theme"); // Some("dark")

// Focus management
let focus = FocusManager::new();
focusable_textfield("Email", &email, "email", &focus);
focus.focus("email");

// Timer
let _timer = RustTimer::start(1.0, || println!("tick"));
```

### Navigation

```rust
// State-based
navigator(&screen, |s| match s { ... })
nav_button("Go", &screen, Screen::Detail(1))
back_button(&screen, Screen::Home)

// SwiftUI NavigationStack
view.navigation_title("Title").navigation_stack()
navigation_link("Detail", detail_view)
```

### App Configuration

```rust
app("Simple", 400.0, 300.0, |cx| { ... });           // simple
App::new("Configured", 800.0, 600.0)                   // configured
    .borderless().min_size(400.0, 300.0).run(|cx| { ... });
SceneApp::new()                                         // scene-based
    .window("main", "App", 800.0, 600.0, |cx| { ... })
    .settings("Prefs", |cx| { ... })
    .launch();
```

## RealityKit

```rust
let rk = RealityKit::new()?;
let sphere = rk.sphere(0.5).at(0.0, 0.5, 0.0);
let floor = rk.plane(10.0, 10.0);
let light = rk.point_light().at(2.0, 3.0, 2.0);
rk.anchor(0.0, 0.0, -3.0).add(&floor).add(&sphere).add(&light);
```

## Combine

```rust
let subject = combine_rs::Subject::new();
let _sub = subject.subscribe(|v| println!("Got: {v}"));
subject.send(42);

let current = combine_rs::CurrentValue::new(0);
current.set(10);
assert_eq!(current.get(), 10);
```

## Persistence

```rust
let db = swift_data::Store::new();
db.set("users", "name", "Alice");
db.set_int("stats", "launches", 5);
assert_eq!(db.get("users", "name"), Some("Alice".into()));
assert_eq!(db.get_int("stats", "launches"), 5);
```

## Bridge Generator

```bash
xcrun swift-api-digester -dump-sdk -module Foundation \
  -target arm64-apple-macosx26.0 -sdk $(xcrun -sdk macosx --show-sdk-path) \
  -o api.json
cargo run -p swift-bridge-gen -- api.json --types URL,UUID,Date
```

## Platform Support

| | macOS | iOS | visionOS |
|--|-------|-----|----------|
| SwiftUI | ✅ | ✅ | future |
| RealityKit | ✅ | ✅ | future |
| Combine | ✅ | ✅ | ✅ |
| Persistence | ✅ | ✅ | ✅ |

Auto-detects SDK version. Pin with `features = ["macos-26"]` or `SWIFTUI_MACOS_VERSION=15.0`.

## Tests

```bash
cargo test --workspace -- --test-threads=1  # 91 tests, 100% pixel parity
```

## License

Apache-2.0
