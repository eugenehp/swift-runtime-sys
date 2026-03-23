#!/usr/bin/env python3
"""Generate Rust crate boilerplate for all Apple Swift frameworks."""

import os
import textwrap

# ── Framework definitions ──
# (crate_dir, lib_name, pkg_name, description, platforms, min_versions, extra_doc)
# platforms: dict of os -> bool
# min_versions: dict of os -> version string

FRAMEWORKS = [
    # ── Media & AV ──
    ("avfaudio", "avfaudio", "avfaudio-rs",
     "Apple AVFAudio — audio playback, recording, and processing from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "13", "tvos": "13", "xros": "1", "watchos": "7"},
     "Wraps AVFAudio for audio engine, players, recorders, and audio sessions."),

    ("avfoundation", "avfoundation", "avfoundation-rs",
     "Apple AVFoundation — media capture, playback, and editing from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.15", "ios": "13", "tvos": "13", "xros": "1"},
     "Wraps AVFoundation for camera capture, video playback, and media composition."),

    ("avkit", "avkit", "avkit-rs",
     "Apple AVKit — media playback UI from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.15", "ios": "13", "tvos": "13", "xros": "1"},
     "Wraps AVKit for AVPlayerViewController and picture-in-picture."),

    ("mediaplayer", "mediaplayer", "mediaplayer-rs",
     "Apple MediaPlayer — music and media playback from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": False, "watchos": True},
     {"macos": "10.12", "ios": "3", "tvos": "14", "watchos": "5"},
     "Wraps MediaPlayer for system music player and Now Playing info."),

    ("musickit", "musickit", "musickit-rs",
     "Apple MusicKit — Apple Music integration from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": False, "watchos": True},
     {"macos": "12", "ios": "15", "tvos": "15", "watchos": "8"},
     "Wraps MusicKit for Apple Music catalog search, playback, and library access."),

    # ── Vision & ML ──
    ("coreml", "coreml", "coreml-rs",
     "Apple Core ML — on-device machine learning from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.13", "ios": "11", "tvos": "11", "xros": "1", "watchos": "4"},
     "Wraps Core ML for loading and running .mlmodel inference on-device."),

    ("vision", "vision", "vision-rs",
     "Apple Vision framework — image analysis and computer vision from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.13", "ios": "11", "tvos": "11", "xros": "1"},
     "Wraps Vision for face detection, text recognition, image classification, and more."),

    ("visionkit", "visionkit", "visionkit-rs",
     "Apple VisionKit — document scanning and visual lookup from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "13", "ios": "13", "xros": "1"},
     "Wraps VisionKit for document camera, data scanner, and Live Text."),

    ("createml", "createml", "createml-rs",
     "Apple Create ML — train machine learning models from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "10.15", "ios": "15"},
     "Wraps Create ML for training image classifiers, text classifiers, and more."),

    ("naturallanguage", "naturallanguage", "naturallanguage-rs",
     "Apple NaturalLanguage — text processing and NLP from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.14", "ios": "12", "tvos": "12", "xros": "1", "watchos": "5"},
     "Wraps NaturalLanguage for language detection, tokenization, and sentiment analysis."),

    ("soundanalysis", "soundanalysis", "soundanalysis-rs",
     "Apple SoundAnalysis — audio classification from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "13", "tvos": "13", "xros": "1", "watchos": "6"},
     "Wraps SoundAnalysis for classifying sounds (speech, music, environment)."),

    ("speech", "speech", "speech-rs",
     "Apple Speech — speech recognition from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.15", "ios": "10", "xros": "1"},
     "Wraps Speech framework for on-device and server-based speech recognition."),

    # ── Data & Cloud ──
    ("cloudkit", "cloudkit", "cloudkit-rs",
     "Apple CloudKit — iCloud database from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.10", "ios": "8", "tvos": "9", "xros": "1", "watchos": "3"},
     "Wraps CloudKit for iCloud public/private database, records, and subscriptions."),

    ("coredata", "coredata", "coredata-rs",
     "Apple Core Data — persistent object graph from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.4", "ios": "3", "tvos": "9", "xros": "1", "watchos": "2"},
     "Wraps Core Data for managed object models, persistent stores, and fetch requests."),

    ("swiftdata", "swiftdata", "swiftdata-rs",
     "Apple SwiftData — modern persistence framework from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "14", "ios": "17", "tvos": "17", "xros": "1", "watchos": "10"},
     "Wraps SwiftData for declarative data modeling and persistence. Note: The existing `swift-data` crate provides UserDefaults. This crate wraps the SwiftData framework."),

    ("tabulardata", "tabulardata", "tabulardata-rs",
     "Apple TabularData — data tables and CSV from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "12", "ios": "15", "tvos": "15", "xros": "1", "watchos": "8"},
     "Wraps TabularData for DataFrame, columns, and CSV/JSON import."),

    # ── UI & Graphics ──
    ("coregraphics", "coregraphics", "coregraphics-rs",
     "Apple Core Graphics — 2D drawing from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.0", "ios": "2", "tvos": "9", "xros": "1", "watchos": "2"},
     "Wraps Core Graphics (Quartz 2D) for paths, colors, images, and PDF generation."),

    ("coreimage", "coreimage", "coreimage-rs",
     "Apple Core Image — image processing and filters from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.4", "ios": "5", "tvos": "9", "xros": "1"},
     "Wraps Core Image for image filters, face detection, and GPU-accelerated processing."),

    ("metal", "metal", "metal-rs",
     "Apple Metal — GPU programming from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.11", "ios": "8", "tvos": "9", "xros": "1"},
     "Wraps Metal for GPU compute, rendering, and shader compilation."),

    ("metalkit", "metalkit", "metalkit-rs",
     "Apple MetalKit — Metal utilities from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.11", "ios": "9", "tvos": "9", "xros": "1"},
     "Wraps MetalKit for MTKView, texture loading, and model I/O integration."),

    ("scenekit", "scenekit", "scenekit-rs",
     "Apple SceneKit — 3D rendering from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.8", "ios": "8", "tvos": "9", "xros": "1", "watchos": "3"},
     "Wraps SceneKit for 3D scene graphs, physics, and rendering."),

    ("spritekit", "spritekit", "spritekit-rs",
     "Apple SpriteKit — 2D game engine from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.9", "ios": "7", "tvos": "9", "xros": "1", "watchos": "3"},
     "Wraps SpriteKit for 2D sprites, physics, and particle systems."),

    ("modelio", "modelio", "modelio-rs",
     "Apple Model I/O — 3D model import/export from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.11", "ios": "9", "tvos": "9", "xros": "1"},
     "Wraps Model I/O for loading 3D assets (USD, OBJ, etc.) and voxelization."),

    ("pencilkit", "pencilkit", "pencilkit-rs",
     "Apple PencilKit — drawing and handwriting from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.15", "ios": "13", "xros": "1"},
     "Wraps PencilKit for canvas drawing, stroke recognition, and Apple Pencil input."),

    ("imageplayground", "imageplayground", "imageplayground-rs",
     "Apple Image Playground — AI image generation from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "15.2", "ios": "18.2"},
     "Wraps Image Playground for on-device AI image generation."),

    ("pdfkit", "pdfkit", "pdfkit-rs",
     "Apple PDFKit — PDF viewing and annotation from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.4", "ios": "11", "xros": "1"},
     "Wraps PDFKit for displaying, searching, and annotating PDF documents."),

    # ── Networking & Communication ──
    ("network", "network", "network-rs",
     "Apple Network framework — modern networking from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.14", "ios": "12", "tvos": "12", "xros": "1", "watchos": "6"},
     "Wraps Network.framework for TCP/UDP/QUIC connections, listeners, and path monitoring."),

    ("networkextension", "networkextension", "networkextension-rs",
     "Apple NetworkExtension — VPN and content filtering from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.11", "ios": "8", "tvos": "17", "xros": "1"},
     "Wraps NetworkExtension for VPN, DNS proxy, and content filter providers."),

    ("multipeerconnectivity", "multipeerconnectivity", "multipeerconnectivity-rs",
     "Apple MultipeerConnectivity — peer-to-peer networking from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.10", "ios": "7", "tvos": "10", "xros": "1"},
     "Wraps MultipeerConnectivity for discovering and communicating with nearby devices."),

    ("nearbyinteraction", "nearbyinteraction", "nearbyinteraction-rs",
     "Apple NearbyInteraction — UWB ranging from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "12", "ios": "14", "xros": "1", "watchos": "8"},
     "Wraps NearbyInteraction for ultra-wideband (U1 chip) spatial awareness."),

    ("pushkit", "pushkit", "pushkit-rs",
     "Apple PushKit — VoIP and complication push notifications from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "8", "xros": "1", "watchos": "6"},
     "Wraps PushKit for receiving VoIP pushes and complication updates."),

    # ── Security & Authentication ──
    ("cryptokit", "cryptokit", "cryptokit-rs",
     "Apple CryptoKit — cryptography from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "13", "tvos": "13", "xros": "1", "watchos": "6"},
     "Wraps CryptoKit for hashing, signing, encryption, and key agreement."),

    ("localauthentication", "localauthentication", "localauthentication-rs",
     "Apple LocalAuthentication — biometric auth from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "10.12", "ios": "8", "xros": "1", "watchos": "3"},
     "Wraps LocalAuthentication for Face ID, Touch ID, and password authentication."),

    ("authenticationservices", "authenticationservices", "authenticationservices-rs",
     "Apple AuthenticationServices — Sign in with Apple from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "12", "tvos": "16", "xros": "1", "watchos": "6"},
     "Wraps AuthenticationServices for Sign in with Apple, passkeys, and web authentication."),

    # ── Location & Maps ──
    ("corelocation", "corelocation", "corelocation-rs",
     "Apple Core Location — GPS and location services from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.6", "ios": "2", "tvos": "9", "xros": "1", "watchos": "2"},
     "Wraps Core Location for GPS, geofencing, beacon ranging, and heading updates."),

    ("mapkit", "mapkit", "mapkit-rs",
     "Apple MapKit — maps and directions from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.9", "ios": "3", "tvos": "9", "xros": "1", "watchos": "2"},
     "Wraps MapKit for map views, annotations, overlays, and directions."),

    # ── Contacts & Calendar ──
    ("contacts", "contacts", "contacts-rs",
     "Apple Contacts — address book access from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "10.11", "ios": "9", "xros": "1", "watchos": "2"},
     "Wraps Contacts for reading and writing contact records."),

    ("eventkit", "eventkit", "eventkit-rs",
     "Apple EventKit — calendar and reminders from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "10.8", "ios": "4", "xros": "1", "watchos": "2"},
     "Wraps EventKit for calendar events, reminders, and alarms."),

    # ── Photos ──
    ("photos", "photos", "photos-rs",
     "Apple Photos — photo library access from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.13", "ios": "8", "tvos": "10", "xros": "1"},
     "Wraps Photos for fetching, caching, and editing photo assets and albums."),

    ("photosui", "photosui", "photosui-rs",
     "Apple PhotosUI — photo picker from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "13", "ios": "14", "xros": "1", "watchos": "9"},
     "Wraps PhotosUI for PHPickerViewController and editing extensions."),

    # ── Games ──
    ("gamecontroller", "gamecontroller", "gamecontroller-rs",
     "Apple GameController — controller input from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.9", "ios": "7", "tvos": "9", "xros": "1"},
     "Wraps GameController for MFi gamepads, keyboard, mouse, and racing wheel input."),

    ("gamekit", "gamekit", "gamekit-rs",
     "Apple GameKit — Game Center from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.8", "ios": "4", "tvos": "9", "xros": "1"},
     "Wraps GameKit for leaderboards, achievements, matchmaking, and Game Center."),

    ("gameplaykit", "gameplaykit", "gameplaykit-rs",
     "Apple GameplayKit — game logic from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.11", "ios": "9", "tvos": "9", "xros": "1"},
     "Wraps GameplayKit for pathfinding, AI state machines, and random sources."),

    # ── Health & Fitness ──
    ("healthkit", "healthkit", "healthkit-rs",
     "Apple HealthKit — health and fitness data from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "13", "ios": "8", "xros": "1", "watchos": "2"},
     "Wraps HealthKit for reading/writing health samples, workouts, and statistics."),

    ("workoutkit", "workoutkit", "workoutkit-rs",
     "Apple WorkoutKit — workout composition from Rust",
     {"macos": False, "ios": True, "tvos": False, "xros": False, "watchos": True},
     {"ios": "17", "watchos": "10"},
     "Wraps WorkoutKit for building custom workout plans and intervals."),

    # ── Payments & Commerce ──
    ("storekit", "storekit", "storekit-rs",
     "Apple StoreKit — in-app purchases and subscriptions from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.7", "ios": "3", "tvos": "9", "xros": "1", "watchos": "6"},
     "Wraps StoreKit 2 for products, transactions, subscriptions, and App Store receipt validation."),

    ("passkit", "passkit", "passkit-rs",
     "Apple PassKit — Wallet and Apple Pay from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"macos": "10.12", "ios": "6", "xros": "1", "watchos": "2"},
     "Wraps PassKit for Apple Pay, Wallet passes, and payment sheets."),

    ("financekit", "financekit", "financekit-rs",
     "Apple FinanceKit — financial data from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "15", "ios": "17.4"},
     "Wraps FinanceKit for reading Apple Card and Apple Cash transaction history."),

    # ── Notifications ──
    ("usernotifications", "usernotifications", "usernotifications-rs",
     "Apple UserNotifications — push and local notifications from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.14", "ios": "10", "tvos": "10", "xros": "1", "watchos": "3"},
     "Wraps UserNotifications for scheduling local notifications and handling remote push."),

    # ── Sharing & Social ──
    ("groupactivities", "groupactivities", "groupactivities-rs",
     "Apple GroupActivities — SharePlay from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "12", "ios": "15", "tvos": "15", "xros": "1"},
     "Wraps GroupActivities for SharePlay sessions and shared experiences."),

    ("sharedwithyou", "sharedwithyou", "sharedwithyou-rs",
     "Apple SharedWithYou — Messages collaboration from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "13", "ios": "16", "tvos": "16", "xros": "1"},
     "Wraps SharedWithYou for surfacing content shared via Messages."),

    ("linkpresentation", "linkpresentation", "linkpresentation-rs",
     "Apple LinkPresentation — URL previews from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.15", "ios": "13", "xros": "1"},
     "Wraps LinkPresentation for fetching rich URL metadata and preview views."),

    # ── Search & Spotlight ──
    ("corespotlight", "corespotlight", "corespotlight-rs",
     "Apple Core Spotlight — search indexing from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.13", "ios": "9", "xros": "1"},
     "Wraps Core Spotlight for indexing app content for system search."),

    # ── Sensors & Motion ──
    ("coremotion", "coremotion", "coremotion-rs",
     "Apple Core Motion — accelerometer and gyroscope from Rust",
     {"macos": False, "ios": True, "tvos": False, "xros": True, "watchos": True},
     {"ios": "4", "xros": "1", "watchos": "2"},
     "Wraps Core Motion for accelerometer, gyroscope, pedometer, and activity recognition."),

    ("corehaptics", "corehaptics", "corehaptics-rs",
     "Apple Core Haptics — haptic feedback from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "10.15", "ios": "13"},
     "Wraps Core Haptics for custom haptic patterns and audio-haptic experiences."),

    ("sensorkit", "sensorkit", "sensorkit-rs",
     "Apple SensorKit — research sensor data from Rust",
     {"macos": False, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"ios": "14"},
     "Wraps SensorKit for ambient light, accelerometer, and keyboard metrics (research use)."),

    # ── Bluetooth & Accessories ──
    ("corebluetooth", "corebluetooth", "corebluetooth-rs",
     "Apple Core Bluetooth — BLE from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.10", "ios": "5", "tvos": "9", "xros": "1", "watchos": "2"},
     "Wraps Core Bluetooth for BLE central/peripheral, service discovery, and characteristics."),

    ("accessorysetupkit", "accessorysetupkit", "accessorysetupkit-rs",
     "Apple AccessorySetupKit — accessory pairing from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "15", "ios": "18"},
     "Wraps AccessorySetupKit for discovering and pairing Bluetooth/Wi-Fi accessories."),

    ("dockkit", "dockkit", "dockkit-rs",
     "Apple DockKit — motorized stand control from Rust",
     {"macos": False, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"ios": "17"},
     "Wraps DockKit for controlling motorized camera stands and tracking."),

    # ── Text & Documents ──
    ("coretext", "coretext", "coretext-rs",
     "Apple Core Text — text layout and font handling from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.5", "ios": "3.2", "tvos": "9", "xros": "1", "watchos": "2"},
     "Wraps Core Text for advanced text layout, font enumeration, and glyph rendering."),

    ("datadetection", "datadetection", "datadetection-rs",
     "Apple DataDetection — structured data extraction from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "13", "ios": "16", "xros": "1"},
     "Wraps DataDetection for extracting dates, addresses, links, and phone numbers from text."),

    # ── Web ──
    ("webkit", "webkit", "webkit-rs",
     "Apple WebKit — web views from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.10", "ios": "8", "xros": "1"},
     "Wraps WebKit/WKWebView for embedding web content, navigation, and JavaScript evaluation."),

    # ── Background & Extensions ──
    ("backgroundtasks", "backgroundtasks", "backgroundtasks-rs",
     "Apple BackgroundTasks — background work scheduling from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "13", "tvos": "13", "xros": "1", "watchos": "7"},
     "Wraps BackgroundTasks for scheduling app refresh and processing tasks."),

    ("extensionkit", "extensionkit", "extensionkit-rs",
     "Apple ExtensionKit — app extensions from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "13", "ios": "16", "tvos": "16", "xros": "1", "watchos": "9"},
     "Wraps ExtensionKit and ExtensionFoundation for building and hosting app extensions."),

    # ── File & Storage ──
    ("fileprovider", "fileprovider", "fileprovider-rs",
     "Apple FileProvider — file sync from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "11", "ios": "11", "xros": "1"},
     "Wraps FileProvider for cloud file sync, enumeration, and materialization."),

    ("quicklook", "quicklook", "quicklook-rs",
     "Apple QuickLook — file previews from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.5", "ios": "4", "xros": "1"},
     "Wraps QuickLook for previewing documents, images, and 3D models."),

    ("uniformtypeidentifiers", "uniformtypeidentifiers", "uniformtypeidentifiers-rs",
     "Apple UniformTypeIdentifiers — UTI system from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "11", "ios": "14", "tvos": "14", "xros": "1", "watchos": "7"},
     "Wraps UniformTypeIdentifiers for declaring and querying file types."),

    # ── Weather ──
    ("weatherkit", "weatherkit", "weatherkit-rs",
     "Apple WeatherKit — weather data from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "13", "ios": "16", "tvos": "16", "xros": "1", "watchos": "9"},
     "Wraps WeatherKit for current conditions, forecasts, and weather alerts."),

    # ── AR & Immersive ──
    ("arkit", "arkit", "arkit-rs",
     "Apple ARKit — augmented reality from Rust",
     {"macos": False, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"ios": "11", "xros": "1"},
     "Wraps ARKit for world tracking, plane detection, face tracking, and body tracking."),

    ("compositorservices", "compositorservices", "compositorservices-rs",
     "Apple CompositorServices — visionOS rendering from Rust",
     {"macos": False, "ios": False, "tvos": False, "xros": True, "watchos": False},
     {"xros": "1"},
     "Wraps CompositorServices for low-level visionOS rendering with Metal."),

    # ── Accessibility ──
    ("accessibility", "accessibility", "accessibility-rs",
     "Apple Accessibility — assistive technology support from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "12", "ios": "15", "tvos": "15", "xros": "1", "watchos": "8"},
     "Wraps Accessibility for VoiceOver, Switch Control, and assistive technology attributes."),

    # ── Device Management ──
    ("devicecheck", "devicecheck", "devicecheck-rs",
     "Apple DeviceCheck — device attestation from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "11", "tvos": "11", "xros": "1", "watchos": "9"},
     "Wraps DeviceCheck and App Attest for device-level fraud prevention."),

    ("deviceactivity", "deviceactivity", "deviceactivity-rs",
     "Apple DeviceActivity — Screen Time monitoring from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "14", "ios": "16"},
     "Wraps DeviceActivity for monitoring app and website usage (Screen Time API)."),

    ("familycontrols", "familycontrols", "familycontrols-rs",
     "Apple FamilyControls — parental controls from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "14", "ios": "16"},
     "Wraps FamilyControls for requesting Screen Time authorization and app restrictions."),

    ("managedsettings", "managedsettings", "managedsettings-rs",
     "Apple ManagedSettings — device restrictions from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "14", "ios": "16"},
     "Wraps ManagedSettings for applying Screen Time shields and restrictions."),

    # ── Analytics & Logging ──
    ("oslog", "oslog", "oslog-rs",
     "Apple OSLog — unified logging from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "10.15", "ios": "15", "tvos": "15", "xros": "1", "watchos": "8"},
     "Wraps OSLog for structured logging with the unified logging system."),

    ("metrickit", "metrickit", "metrickit-rs",
     "Apple MetricKit — app diagnostics from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "12", "ios": "13", "xros": "1"},
     "Wraps MetricKit for receiving aggregated app performance and diagnostic data."),

    # ── Suggestions & Tips ──
    ("tipkit", "tipkit", "tipkit-rs",
     "Apple TipKit — in-app tips and hints from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "14", "ios": "17", "tvos": "17", "xros": "1", "watchos": "10"},
     "Wraps TipKit for displaying contextual tips, feature discovery, and onboarding hints."),

    # ── Screen & Display ──
    ("screencapturekit", "screencapturekit", "screencapturekit-rs",
     "Apple ScreenCaptureKit — screen recording from Rust",
     {"macos": True, "ios": False, "tvos": False, "xros": False, "watchos": False},
     {"macos": "12.3"},
     "Wraps ScreenCaptureKit for capturing screen content, windows, and apps."),

    # ── Content Safety ──
    ("sensitivecontentanalysis", "sensitivecontentanalysis", "sensitivecontentanalysis-rs",
     "Apple SensitiveContentAnalysis — CSAM/nudity detection from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "14", "ios": "17", "xros": "1"},
     "Wraps SensitiveContentAnalysis for detecting sensitive imagery."),

    # ── Music Recognition ──
    ("shazamkit", "shazamkit", "shazamkit-rs",
     "Apple ShazamKit — music recognition from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "12", "ios": "15", "tvos": "15", "xros": "1", "watchos": "8"},
     "Wraps ShazamKit for identifying songs from audio and building custom catalogs."),

    # ── Virtualization (macOS-only) ──
    ("virtualization", "virtualization", "virtualization-rs",
     "Apple Virtualization — virtual machines from Rust",
     {"macos": True, "ios": False, "tvos": False, "xros": False, "watchos": False},
     {"macos": "11"},
     "Wraps Virtualization.framework for running Linux and macOS VMs."),

    # ── Core Transferable ──
    ("coretransferable", "coretransferable", "coretransferable-rs",
     "Apple CoreTransferable — drag-and-drop and sharing from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "13", "ios": "16", "tvos": "16", "xros": "1", "watchos": "9"},
     "Wraps CoreTransferable for the Transferable protocol, drag-and-drop, and copy/paste."),

    # ── Cinematic ──
    ("cinematic", "cinematic", "cinematic-rs",
     "Apple Cinematic — cinematic video processing from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "14", "ios": "17"},
     "Wraps Cinematic for processing Cinematic mode video and adjusting depth of field."),

    # ── Matter ──
    ("matter", "matter", "matter-rs",
     "Apple Matter — smart home connectivity from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": False, "watchos": False},
     {"macos": "14", "ios": "16", "tvos": "16"},
     "Wraps Matter/MatterSupport for commissioning and controlling Matter smart home devices."),

    # ── Call & Communication ──
    ("callkit", "callkit", "callkit-rs",
     "Apple CallKit — VoIP call integration from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": True},
     {"macos": "13", "ios": "10", "watchos": "9"},
     "Wraps CallKit for VoIP call UI, call directory, and blocking/identification."),

    ("livecommunicationkit", "livecommunicationkit", "livecommunicationkit-rs",
     "Apple LiveCommunicationKit — live calling from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "14", "ios": "17"},
     "Wraps LiveCommunicationKit for VoIP calling with modern async API."),

    # ── Symbols ──
    ("symbols", "symbols", "symbols-rs",
     "Apple Symbols — SF Symbols metadata from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": True},
     {"macos": "14", "ios": "17", "tvos": "17", "xros": "1", "watchos": "10"},
     "Wraps Symbols for SF Symbol effects, variable color, and symbol images."),

    # ── Replay & Broadcasting ──
    ("replaykit", "replaykit", "replaykit-rs",
     "Apple ReplayKit — screen recording and broadcasting from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": False, "watchos": False},
     {"macos": "11", "ios": "9", "tvos": "10"},
     "Wraps ReplayKit for in-app screen recording and live broadcasting."),

    # ── Ad & Attribution ──
    ("adservices", "adservices", "adservices-rs",
     "Apple AdServices — ad attribution from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": False, "watchos": False},
     {"macos": "14", "ios": "14.3"},
     "Wraps AdServices for Apple Search Ads attribution tokens."),

    ("adsupport", "adsupport", "adsupport-rs",
     "Apple AdSupport — advertising identifier from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "10.14", "ios": "6", "tvos": "9", "xros": "1"},
     "Wraps AdSupport for reading the IDFA advertising identifier."),

    ("apptrackingtransparency", "apptrackingtransparency", "apptrackingtransparency-rs",
     "Apple AppTrackingTransparency — tracking permission from Rust",
     {"macos": True, "ios": True, "tvos": True, "xros": True, "watchos": False},
     {"macos": "11", "ios": "14", "tvos": "14", "xros": "1"},
     "Wraps AppTrackingTransparency for requesting user permission to track."),

    # ── Safari ──
    ("safariservices", "safariservices", "safariservices-rs",
     "Apple SafariServices — in-app browser from Rust",
     {"macos": True, "ios": True, "tvos": False, "xros": True, "watchos": False},
     {"macos": "10.12", "ios": "7", "xros": "1"},
     "Wraps SafariServices for SFSafariViewController, content blockers, and web extensions."),
]


def platform_list(platforms):
    """Build a human-readable platform support string."""
    parts = []
    for os_name, label in [("macos", "macOS"), ("ios", "iOS"), ("tvos", "tvOS"),
                            ("xros", "visionOS"), ("watchos", "watchOS")]:
        if platforms.get(os_name):
            parts.append(label)
    return ", ".join(parts)


def unavailable_list(platforms):
    """OS names where the framework is NOT available."""
    out = []
    for os_name in ["macos", "ios", "tvos", "xros", "watchos"]:
        if not platforms.get(os_name):
            out.append(os_name)
    return out


def version_string(platforms, min_versions):
    """Build a version string like 'macOS 10.15+, iOS 13+, ...'."""
    parts = []
    labels = {"macos": "macOS", "ios": "iOS", "tvos": "tvOS", "xros": "visionOS", "watchos": "watchOS"}
    for os_name in ["macos", "ios", "tvos", "xros", "watchos"]:
        if platforms.get(os_name) and os_name in min_versions:
            parts.append(f"{labels[os_name]} {min_versions[os_name]}+")
    return ", ".join(parts)


def cfg_supported(platforms):
    """Build a cfg() expression for supported platforms."""
    supported = [k for k, v in platforms.items() if v]
    if len(supported) == 5:
        return None  # all platforms
    parts = [f'target_os = "{os}"' for os in supported]
    if len(parts) == 1:
        return parts[0]
    return f"any({', '.join(parts)})"


def cfg_unsupported(platforms):
    """Build a cfg() expression for UNsupported platforms."""
    unsupported = [k for k, v in platforms.items() if not v]
    if not unsupported:
        return None
    parts = [f'target_os = "{os}"' for os in unsupported]
    if len(parts) == 1:
        return parts[0]
    return f"any({', '.join(parts)})"


def gen_cargo_toml(crate_dir, lib_name, pkg_name, description):
    return f'''[package]
name = "{pkg_name}"
description = "{description}"
version = "0.0.1"
edition = "2021"
license = "Apache-2.0"

[lib]
name = "{lib_name}"

[lints.rust]
unexpected_cfgs = {{ level = "allow", check-cfg = ['cfg(target_os, values("xros", "tvos", "watchos"))', 'cfg(target_abi, values("sim"))'] }}
'''


def gen_lib_rs(lib_name, description, platforms, min_versions, extra_doc):
    ver_str = version_string(platforms, min_versions)
    plat_str = platform_list(platforms)

    cfg_sup = cfg_supported(platforms)
    cfg_unsup = cfg_unsupported(platforms)

    lines = []
    lines.append(f"//! {description}.")
    lines.append(f"//!")
    lines.append(f"//! **Platform support:** {ver_str}.")
    lines.append(f"//!")
    lines.append(f"//! {extra_doc}")
    lines.append(f"//!")
    lines.append(f"//! ```ignore")
    lines.append(f"//! assert!({lib_name}::is_available());")
    lines.append(f"//! ```")
    lines.append("")

    # Real implementation
    if cfg_sup:
        lines.append(f"#[cfg({cfg_sup})]")
        lines.append("mod real {")
    else:
        lines.append("mod real {")

    lines.append("    use core::ffi::{c_char, c_void};")
    lines.append("")
    lines.append("    unsafe extern \"C\" {")
    lines.append("        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;")
    lines.append("    }")
    lines.append("")
    lines.append("    fn sym(name: &core::ffi::CStr) -> *const c_void {")
    lines.append("        unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }")
    lines.append("    }")
    lines.append("")
    lines.append(f"    /// Check if the framework is available at runtime.")
    lines.append(f"    pub fn is_available() -> bool {{")
    lines.append(f"        let f = sym(c\"{lib_name}_available\");")
    lines.append(f"        if f.is_null() {{")
    lines.append(f"            return false;")
    lines.append(f"        }}")
    lines.append(f"        type F = unsafe extern \"C\" fn() -> bool;")
    lines.append(f"        unsafe {{ (std::mem::transmute::<_, F>(f))() }}")
    lines.append(f"    }}")
    lines.append("}")
    lines.append("")

    if cfg_sup:
        lines.append(f"#[cfg({cfg_sup})]")
    lines.append("pub use real::*;")
    lines.append("")

    # Stub for unsupported platforms
    if cfg_unsup:
        lines.append(f"#[cfg({cfg_unsup})]")
        lines.append("mod stub {")
        lines.append(f"    /// This framework is not available on this platform. Always returns `false`.")
        lines.append(f"    pub fn is_available() -> bool {{")
        lines.append(f"        false")
        lines.append(f"    }}")
        lines.append("}")
        lines.append("")
        lines.append(f"#[cfg({cfg_unsup})]")
        lines.append("pub use stub::*;")
        lines.append("")

    return "\n".join(lines)


def gen_build_rs(lib_name, platforms):
    """Generate build.rs that links framework on supported platforms."""
    # Map crate name to framework name
    framework_map = {
        "avfaudio": "AVFAudio",
        "avfoundation": "AVFoundation",
        "avkit": "AVKit",
        "mediaplayer": "MediaPlayer",
        "musickit": "MusicKit",
        "coreml": "CoreML",
        "vision": "Vision",
        "visionkit": "VisionKit",
        "createml": "CreateML",
        "naturallanguage": "NaturalLanguage",
        "soundanalysis": "SoundAnalysis",
        "speech": "Speech",
        "cloudkit": "CloudKit",
        "coredata": "CoreData",
        "swiftdata": "SwiftData",
        "tabulardata": "TabularData",
        "coregraphics": "CoreGraphics",
        "coreimage": "CoreImage",
        "metal": "Metal",
        "metalkit": "MetalKit",
        "scenekit": "SceneKit",
        "spritekit": "SpriteKit",
        "modelio": "ModelIO",
        "pencilkit": "PencilKit",
        "imageplayground": "ImagePlayground",
        "pdfkit": "PDFKit",
        "network": "Network",
        "networkextension": "NetworkExtension",
        "multipeerconnectivity": "MultipeerConnectivity",
        "nearbyinteraction": "NearbyInteraction",
        "pushkit": "PushKit",
        "cryptokit": "CryptoKit",
        "localauthentication": "LocalAuthentication",
        "authenticationservices": "AuthenticationServices",
        "corelocation": "CoreLocation",
        "mapkit": "MapKit",
        "contacts": "Contacts",
        "eventkit": "EventKit",
        "photos": "Photos",
        "photosui": "PhotosUI",
        "gamecontroller": "GameController",
        "gamekit": "GameKit",
        "gameplaykit": "GameplayKit",
        "healthkit": "HealthKit",
        "workoutkit": "WorkoutKit",
        "storekit": "StoreKit",
        "passkit": "PassKit",
        "financekit": "FinanceKit",
        "usernotifications": "UserNotifications",
        "groupactivities": "GroupActivities",
        "sharedwithyou": "SharedWithYou",
        "linkpresentation": "LinkPresentation",
        "corespotlight": "CoreSpotlight",
        "coremotion": "CoreMotion",
        "corehaptics": "CoreHaptics",
        "sensorkit": "SensorKit",
        "corebluetooth": "CoreBluetooth",
        "accessorysetupkit": "AccessorySetupKit",
        "dockkit": "DockKit",
        "coretext": "CoreText",
        "datadetection": "DataDetection",
        "webkit": "WebKit",
        "backgroundtasks": "BackgroundTasks",
        "extensionkit": "ExtensionKit",
        "fileprovider": "FileProvider",
        "quicklook": "QuickLook",
        "uniformtypeidentifiers": "UniformTypeIdentifiers",
        "weatherkit": "WeatherKit",
        "arkit": "ARKit",
        "compositorservices": "CompositorServices",
        "accessibility": "Accessibility",
        "devicecheck": "DeviceCheck",
        "deviceactivity": "DeviceActivity",
        "familycontrols": "FamilyControls",
        "managedsettings": "ManagedSettings",
        "oslog": "OSLog",
        "metrickit": "MetricKit",
        "tipkit": "TipKit",
        "screencapturekit": "ScreenCaptureKit",
        "sensitivecontentanalysis": "SensitiveContentAnalysis",
        "shazamkit": "ShazamKit",
        "virtualization": "Virtualization",
        "coretransferable": "CoreTransferable",
        "cinematic": "Cinematic",
        "matter": "Matter",
        "callkit": "CallKit",
        "livecommunicationkit": "LiveCommunicationKit",
        "symbols": "Symbols",
        "replaykit": "ReplayKit",
        "adservices": "AdServices",
        "adsupport": "AdSupport",
        "apptrackingtransparency": "AppTrackingTransparency",
        "safariservices": "SafariServices",
    }

    fw_name = framework_map.get(lib_name, lib_name)
    supported = [k for k, v in platforms.items() if v]
    
    lines = []
    lines.append("fn main() {")
    lines.append("    let os = std::env::var(\"CARGO_CFG_TARGET_OS\").unwrap_or_default();")
    quoted = ', '.join('"' + s + '"' for s in supported)
    lines.append(f"    let supported = [{quoted}];")
    lines.append("    if supported.contains(&os.as_str()) {")
    lines.append(f"        // Framework is loaded at runtime via dlsym — no link-time dependency needed.")
    lines.append(f"        // If you need link-time binding, uncomment:")
    lines.append(f"        // println!(\"cargo:rustc-link-lib=framework={fw_name}\");")
    lines.append(f"        println!(\"cargo:warning={fw_name} framework available\");")
    lines.append("    } else {")
    lines.append(f"        println!(\"cargo:warning={fw_name} framework not available on {{}}\", os);")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


def main():
    base = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    crates_dir = os.path.join(base, "crates")

    created = []
    for (crate_dir, lib_name, pkg_name, desc, platforms, min_versions, extra_doc) in FRAMEWORKS:
        crate_path = os.path.join(crates_dir, crate_dir)
        if os.path.exists(crate_path):
            print(f"  SKIP {crate_dir} (already exists)")
            continue

        os.makedirs(os.path.join(crate_path, "src"), exist_ok=True)

        cargo = gen_cargo_toml(crate_dir, lib_name, pkg_name, desc)
        with open(os.path.join(crate_path, "Cargo.toml"), "w") as f:
            f.write(cargo)

        lib_rs = gen_lib_rs(lib_name, desc, platforms, min_versions, extra_doc)
        with open(os.path.join(crate_path, "src", "lib.rs"), "w") as f:
            f.write(lib_rs)

        build_rs = gen_build_rs(lib_name, platforms)
        with open(os.path.join(crate_path, "build.rs"), "w") as f:
            f.write(build_rs)

        created.append(crate_dir)
        print(f"  CREATE {crate_dir}")

    print(f"\nCreated {len(created)} crates.")
    if created:
        print("\nAdd to workspace Cargo.toml members:")
        for c in sorted(created):
            print(f'    "crates/{c}",')


if __name__ == "__main__":
    main()
