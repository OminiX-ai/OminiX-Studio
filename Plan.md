# Implementation Plan: Moly with MoFA Studio Look & Feel

## Overview
Create a new makepad project combining Moly-AI's functionality with MoFA Studio's UI/UX design. The project will be built incrementally, starting with a simple runnable shell, then adding features progressively.

## Project Architecture

### Project Name
**moly** - Matches the source project name for consistency.

### Recommended Structure
```
moly/
├── Cargo.toml (workspace)
├── moly-shell/              # Main application (like mofa-studio-shell)
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs           # App shell with header, sidebar, navigation
│   │   └── data/            # Shared Store for app communication
│   └── resources/           # Icons, fonts, images (copied from mofa-studio)
├── moly-widgets/            # Shared widget library (from mofa-widgets + moly-kit)
│   └── src/
│       ├── theme.rs         # Color system, fonts
│       └── ...              # Reusable widgets
└── apps/
    ├── moly-chat/           # Chat interface app
    ├── moly-models/         # Model discovery/download app
    ├── moly-mcp/            # MCP configuration app (desktop only)
    └── moly-settings/       # Provider settings app
```

### Key Dependencies

**From moly-ai:**
- `moly-protocol` - git: https://github.com/moly-ai/moly-local, rev: 788cac14d
- `aitk` (AI Toolkit) - git: https://github.com/moly-ai/aitk, rev: 7c20045b
- `moly-kit` - Reference from ../moly-ai/moly-kit (local path)
- `makepad-widgets` - wyeworks/makepad, rev: 53b2e5c84

**From mofa-studio:**
- Resources (fonts, icons) - Will be copied to new project
- Theme system and UI patterns

**Version Resolution:**
- Use moly-ai's makepad revision (53b2e5c84) as it's the working version for Moly functionality
- mofa-studio uses b8b65f4fa but we prioritize functional compatibility with moly-ai

### User Decisions Summary
✅ **Project Name**: moly (matches moly-ai)
✅ **Resources**: Copy files to new project (self-contained)
✅ **Features**: Include all features (full parity)
✅ **Architecture**: Hybrid approach (plugin system + shared Store)

### Key Architectural Decisions

**1. Plugin System vs Integrated Approach**
- **Decision**: Use MoFA's plugin system (MolyApp trait) but with a **shared Store** (user approved)
- **Rationale**:
  - Apps can remain mostly independent (MoFA pattern)
  - Store acts as message bus for cross-app communication
  - Shell owns Store, passes references to apps via custom events
- **Trade-off**: Not pure black-box, but necessary for Moly's architecture and user-approved

**2. Widget Libraries**
- **Decision**: Create `moly-widgets` based on `mofa-widgets`, add Moly-specific widgets as needed
- **Rationale**:
  - Reuse MoFA's theme system, base widgets
  - Add moly-kit widgets (ChatView, MessageList, etc.) to moly-widgets
  - Single unified widget library
- **Trade-off**: Some duplication from moly-kit, but better integration

**3. Navigation Pattern**
- **Decision**: Use MoFA's sidebar + app switching, but apps manage their own sub-navigation
- **Rationale**:
  - Sidebar shows: Chat, Models, MCP (desktop), Settings
  - Each app handles internal routing if needed
  - Consistent with both source projects

**4. Dependencies**
- **Decision**: Import moly-protocol, aitk as dependencies in apps
- **Rationale**: These are core to Moly functionality
- **Approach**: Reference via path or git if not published

## Phase 1: Runnable Shell (Basic Structure) ✅ COMPLETED

### Goal
Create a minimal runnable application with MoFA Studio's look and feel, no AI functionality yet.

### Status
✅ **COMPLETED** - Basic shell with all core functionality working.

### Steps

**1.1 Project Setup** ✅ COMPLETED
- ✅ Create workspace Cargo.toml
- ✅ Create moly-shell/ package
- ✅ Create moly-widgets/ package
- ✅ Add makepad-widgets dependencies

**1.2 Theme System (moly-widgets/src/theme.rs)** ✅ COMPLETED
- ✅ Copy color definitions from mofa-widgets (200+ Tailwind colors)
- ✅ Copy font definitions (THEME_FONT_REGULAR, MEDIUM, SEMIBOLD, BOLD)
- ✅ Copy resources (fonts: Regular, Medium, SemiBold, Bold)
- ✅ Create all icons (hamburger.svg, sun.svg, moon.svg, chat.svg, settings.svg, app.svg)

**1.3 Basic Shell (moly-shell/src/)** ✅ COMPLETED
- ✅ **main.rs**: Entry point, platform initialization, logger
- ✅ **app.rs**: Shell structure with:
  - ✅ Window (1400x900, "Moly")
  - ✅ Header with hamburger icon, "Moly" title, and theme toggle (sun/moon icon)
  - ✅ Theme toggle button (fully wired with dark mode switching)
  - ✅ Sidebar with navigation buttons (Chat, Models, Settings)
  - ✅ Content area with three views (Chat, Models, Settings)
  - ✅ Dark mode support with shader-based color mixing
  - ✅ Navigation logic (view switching works)

**1.4 Verification** ✅ COMPLETED
- ✅ Run: `cargo run` - Application runs successfully
- ✅ MoFA-styled window displays correctly
- ✅ Theme toggle switches between light and dark modes
- ✅ Navigation switches between Chat/Models/Settings views
- ✅ All icons display correctly

### Implementation Notes
- Used Icon widgets wrapped in Views for clickable icons
- SVG files converted to use `<path>` tags (Makepad requirement)
- Font naming follows Makepad convention: `THEME_FONT_*`
- Dark mode uses shader instance variables for smooth color transitions
- Event handling uses `MatchEvent` trait with `handle_actions()`
- Widget references use `ids!()` macro (plural)

### Files to Create (Phase 1)

```
moly/
├── Cargo.toml                                 # Workspace
├── moly-shell/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                            # Entry point
│   │   └── app.rs                             # Shell UI
│   └── resources/
│       ├── fonts/                             # Manrope fonts (copy from mofa-studio)
│       │   ├── Manrope-Regular.ttf
│       │   ├── Manrope-Medium.ttf
│       │   ├── Manrope-SemiBold.ttf
│       │   └── Manrope-Bold.ttf
│       └── icons/                             # SVG icons (copy from mofa-studio)
│           ├── hamburger.svg
│           ├── sun.svg
│           ├── moon.svg
│           ├── chat.svg
│           ├── app.svg
│           └── settings.svg
└── moly-widgets/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        └── theme.rs                           # Color system
```

### Phase 1 Implementation Details

**Workspace Cargo.toml:**
- Define workspace members: moly-shell, moly-widgets
- Set workspace dependencies: makepad-widgets (rev: 53b2e5c84), common crates

**moly-shell/Cargo.toml:**
- Dependencies: makepad-widgets, moly-widgets (path)
- Platform-specific deps: tokio (native), wasm-bindgen (wasm)

**moly-shell/src/main.rs:**
- Platform initialization (from moly-ai/src/main.rs)
- Set working directory for macOS app bundle support
- Logger initialization
- Launch app_main()

**moly-shell/src/app.rs:**
- Shell struct with Header, Sidebar, ContentArea
- Header: Logo (40x40), title "Moly", hamburger button, theme toggle
- Sidebar: Hover/pin modes with animation, menu items (Chat, Models, Settings)
- Dark mode state and animation system (from mofa-studio patterns)
- Event handling for navigation, theme toggle
- Use moly-widgets theme colors

**moly-widgets/src/theme.rs:**
- Copy color definitions from mofa-studio/mofa-widgets/src/theme.rs
- Full Tailwind palette (SLATE, GRAY, BLUE, GREEN, RED, etc.)
- Light/dark mode colors
- Font definitions for Manrope family

**moly-widgets/src/lib.rs:**
- Re-export theme module
- Setup for future widgets

**Resources:**
- Copy fonts from: ../mofa-studio/mofa-studio-shell/resources/ OR ../mofa-studio/mofa-widgets/resources/
- Copy icons from: ../mofa-studio/mofa-studio-shell/resources/icons/
- May need to create chat.svg, models.svg if not in mofa-studio

## Phase 2: App Plugin System ✅ COMPLETED

### Goal
Implement MoFA's plugin architecture and create empty app containers.

### Status
✅ **COMPLETED** - Multi-app system working with internal module structure.

### Steps

**2.1 Plugin Trait (moly-shell/src/app.rs)** ✅ COMPLETED
- Decision: Simplified to use Makepad Widget pattern instead of custom trait
- Apps are Makepad widgets using `#[derive(Live, LiveHook, Widget)]`

**2.2 App Containers** ✅ COMPLETED
- ✅ Created moly-shell/src/apps/chat.rs (ChatApp widget)
- ✅ Created moly-shell/src/apps/models.rs (ModelsApp widget)
- ✅ Created moly-shell/src/apps/settings.rs (SettingsApp widget)
- ✅ Created moly-shell/src/apps/mcp.rs (McpApp widget)

**2.3 Shell Integration** ✅ COMPLETED
- ✅ Apps load via live_design! and live_register()
- ✅ Sidebar wired to switch apps via NavigationTarget enum
- ✅ Content area displays active app with visibility toggling
- ✅ Dark mode propagates to all apps

**2.4 Verification** ✅ COMPLETED
- ✅ Sidebar navigation switches between app screens
- ✅ Each app shows placeholder text
- ✅ Dark mode applies to all apps
- ✅ Navigation state persists while switching

**2.5 Enhancements** ✅ COMPLETED
- ✅ Hamburger menu functionality (collapse/expand sidebar)
- ✅ Collapsible sidebar (250px expanded → 60px collapsed)
- ✅ Icon-based navigation (visible when collapsed)
- ✅ Colored icons for better UX:
  - Chat: Blue (#3b82f6) - Communication
  - Models: Purple (#8b5cf6) - Tech/AI
  - Settings: Amber (#f59e0b) - Tools
- ✅ Logo added to header (moly-logo.png)
- ✅ Icons adjust colors for dark mode

### Implementation Notes
- **Architecture Decision**: Used internal modules (moly-shell/src/apps/) instead of separate crates
- **Rationale**: Makepad's `live_design!` DSL has limitations with cross-crate widget imports
- **Pattern**: Each app is a Makepad Widget in its own module file
- **Future**: Can refactor to separate crates once cross-crate pattern is better understood
- Apps register via `live_design(cx)` calls in the shell's `LiveRegister` implementation
- Navigation uses visibility toggling rather than dynamic widget instantiation
- **Sidebar Toggle**: Uses `set_visible()` on labels to show/hide text while keeping icons
- **Color Scheme**: Icons use semantic colors that adapt to dark mode for accessibility

### Files Created (Phase 2)

```
moly-shell/src/
├── apps/
│   ├── mod.rs                                 # Module exports
│   ├── chat.rs                                # ChatApp widget
│   ├── models.rs                              # ModelsApp widget
│   ├── settings.rs                            # SettingsApp widget
│   └── mcp.rs                                 # McpApp widget
├── main.rs                                    # Updated with apps module
└── app.rs                                     # Updated with app switching logic
```

Note: External app crates (apps/moly-*/) exist but are not currently used.

## Phase 3: Shared State & Store ✅ COMPLETED

### Goal
Implement Moly's Store pattern in the shell, enable app communication.

### Status
✅ **COMPLETED** - Store pattern implemented following moly-ai's architecture.

### Steps

**3.1 Store Implementation (moly-shell/src/data/)** ✅ COMPLETED
- ✅ Created Store struct with Preferences
- ✅ Synchronous loading (simplified from moly-ai's async pattern)
- ✅ Preferences persistence to JSON file
- ✅ Created StoreAction enum for future action handling

**3.2 Store Integration** ✅ COMPLETED
- ✅ Shell owns Store instance
- ✅ Apps receive Store reference via Makepad's Scope mechanism
- ✅ Apps read Store state in draw_walk()
- ✅ Shell updates Store and triggers redraws

**3.3 Verification** ✅ COMPLETED
- ✅ Store loads preferences on startup
- ✅ Apps can read Store via `scope.data.get::<Store>()`
- ✅ Preferences persist across restarts (dark_mode, sidebar_expanded, current_view)

### Implementation Notes

**Architecture Decision**: Used Makepad's native Scope pattern instead of custom events:
```rust
// In App's handle_event
let scope = &mut Scope::with_data(&mut self.store);
self.ui.handle_event(cx, event, scope);

// In child widgets' draw_walk
if let Some(store) = scope.data.get::<Store>() {
    let dark_mode = store.is_dark_mode();
    // Apply state
}
```

**Preferences Persistence**:
- File location: `<executable_dir>/preferences/preferences.json`
- Format: JSON with serde serialization
- Auto-saves on state changes

### Files Created (Phase 3)

```
moly-shell/src/
├── data/
│   ├── mod.rs                                 # Module exports
│   ├── store.rs                               # Store struct & StoreAction
│   └── preferences.rs                         # Preferences with persistence
└── app.rs                                     # Updated with Store integration
```

## Phase 4: Chat Feature ✅ COMPLETED

### Goal
Implement full chat functionality in moly-chat app.

### Status
✅ **COMPLETED** - Full chat functionality with multi-provider support, persistence, and chat history UI.

### Steps

**4.1 Dependencies** ✅ COMPLETED
- ✅ Added moly-kit (local path from ../moly-ai/moly-kit)
- ✅ Added aitk (git: moly-ai/aitk, rev: 7c20045b)
- ✅ Added moly-protocol (git: moly-ai/moly-local, rev: 788cac14d)
- ✅ Added uuid, chrono, futures dependencies

**4.2 Chat UI** ✅ COMPLETED
- ✅ Integrated moly-kit's Chat widget into ChatApp
- ✅ Messages widget working (displays message history)
- ✅ PromptInput widget working (text input with send button)
- ✅ ChatController initialized with basic spawner
- ✅ Registered moly-kit widgets via live_design(cx)

**4.3 Chat Data Models** ✅ COMPLETED
- ✅ Created moly-shell/src/data/chats.rs with:
  - ChatId type
  - ChatData struct (id, title, bot_id, messages, timestamps)
  - Chats manager struct
- ✅ Integrated Chats into Store
- ✅ ChatController stored in Store (for future sharing)

**4.4 BotContext/Provider Integration** ✅ COMPLETED
- ✅ ProvidersManager configured with OpenAI and Gemini support
- ✅ BotClient created per provider with API credentials
- ✅ Model dropdown shows 164+ models from all enabled providers
- ✅ Bot selection working with `switch_to_provider_for_bot()`
- ✅ Provider switching preserves model list (re-dispatch bots after set_client)
- ✅ Selected model persists via Preferences

**4.5 View State Management** ✅ COMPLETED
- ✅ Models persist when switching between views (Chat ↔ Models ↔ Settings)
- ✅ `on_become_visible()` method resets controller when returning to Chat
- ✅ Force re-set controller pattern to bypass early return checks

**4.6 Chat Persistence** ✅ COMPLETED
- ✅ Chats saved to `~/.moly/chats/{id}.chat.json`
- ✅ Messages synced to disk when count changes
- ✅ Chats loaded on app startup
- ✅ Auto-generated title from first user message

**4.7 Chat History UI** ✅ COMPLETED
- ✅ Chat history sidebar panel with list of saved chats
- ✅ "New Chat" button to create new chat sessions
- ✅ Click handling on chat history items (ChatHistoryItem Widget)
- ✅ Chat switching with proper scroll position reset
- ✅ Message sync tracking (detects streaming completion)
- ✅ Content persistence (saves streaming content properly)

**4.8 Verification** ✅ COMPLETED
- ✅ Model dropdown shows all available models
- ✅ Model selection changes provider correctly
- ✅ Selected model persists across restarts
- ✅ Chat history persistence (messages saved to disk)
- ✅ Multiple chat sessions with chat list and switching
- ✅ Chat history items clickable and functional
- ✅ Streaming messages persist correctly after completion

### Implementation Notes

**Architecture Decision**: Used moly-kit's Chat widget directly instead of porting individual components:
- Chat widget includes Messages + PromptInput + voice modal
- ChatController manages state, streaming, and plugins
- Used `after_new_from_doc` LiveHook to initialize controller before draw

**Key Pattern**: Initialize ChatController with default value to avoid "no chat controller set" panic:
```rust
#[rust(ChatController::new_arc())]
chat_controller: Arc<Mutex<ChatController>>,

impl LiveHook for ChatApp {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.chat_controller.lock().unwrap().set_basic_spawner();
        self.view.chat(ids!(chat))
            .write()
            .set_chat_controller(cx, Some(self.chat_controller.clone()));
    }
}
```

**Key Fix**: Provider Switching Preserves Bots
The `set_client()` method on ChatController clears all bots. Solution:
```rust
fn switch_to_provider_for_bot(&mut self, bot_id: &BotId, scope: &mut Scope) {
    // Save all bots before switching
    let all_bots = store.providers_manager.get_all_bots().to_vec();

    // set_client() clears bots
    ctrl.set_client(Some(Box::new(client)));

    // Re-dispatch bots after set_client
    ctrl.dispatch_mutation(VecMutation::Set(all_bots));
}
```

**Key Fix**: View Switching Preserves Model List
When navigating away and back to Chat, the controller needs to be re-set:
```rust
pub fn on_become_visible(&mut self) {
    self.needs_controller_reset = true;
}

// In handle_event, when needs_controller_reset is true:
// 1. Force re-set controller (set to None, then back to Some)
// 2. Re-dispatch bots mutation
// 3. Trigger redraw
```

**Key Pattern**: ChatHistoryItem Widget for Click Handling
Converted from View template to proper Widget struct to enable click detection:
```rust
#[derive(Live, LiveHook, Widget)]
pub struct ChatHistoryItem {
    #[deref]
    view: View,
    #[rust]
    chat_id: Option<ChatId>,
}

impl ChatHistoryItem {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.view.widget_uid()) {
            if let ViewAction::FingerDown(fd) = item.cast() {
                return fd.tap_count == 1;
            }
        }
        false
    }
}
```

**Key Fix**: Message Sync Tracking for Streaming Content
Track multiple conditions to properly persist streaming messages:
```rust
#[rust]
last_synced_message_count: usize,
#[rust]
had_writing_message: bool,
#[rust]
last_synced_content_len: usize,

fn sync_messages_to_persistence(&mut self, scope: &mut Scope) {
    let count_changed = message_count != self.last_synced_message_count;
    let writing_finished = self.had_writing_message && !has_writing_message;
    let content_changed = last_msg_content_len != self.last_synced_content_len;

    if !count_changed && !writing_finished && !content_changed {
        return;
    }
    // ... sync to store
}
```

**Key Fix**: PortalList Scroll Position Reset
When switching chats, reset scroll position to avoid "first_id > range_end" errors:
```rust
pub fn switch_to_chat(&mut self, cx: &mut Cx, scope: &mut Scope, chat_id: ChatId) {
    // ... load messages ...

    // Reset scroll position to avoid PortalList errors
    self.view.chat(ids!(chat)).write().messages_ref().write().instant_scroll_to_bottom(cx);

    // Reset sync tracking state
    self.last_synced_message_count = message_count;
    self.had_writing_message = false;
    self.last_synced_content_len = last_content_len;
}
```

### Files Created/Modified (Phase 4)

```
moly-shell/src/
├── data/
│   ├── mod.rs                                 # Added chats, providers_manager exports
│   ├── chats.rs                               # Chat data models
│   ├── preferences.rs                         # Added current_chat_model persistence
│   ├── providers_manager.rs                   # NEW: Multi-provider bot management
│   └── store.rs                               # Added ProvidersManager integration
├── apps/
│   ├── chat.rs                                # Multi-provider support, view state fixes
│   └── settings.rs                            # Provider configuration UI
└── app.rs                                     # View visibility notifications

Cargo.toml (workspace)                         # Added moly-kit, aitk, moly-protocol deps
moly-shell/Cargo.toml                          # Added workspace dependencies
```

### Phase 4 Complete
All chat functionality implemented:
- ✅ Chat history persistence
- ✅ Multiple chat sessions UI (chat list, new chat button, chat switching)
- ✅ End-to-end chat flow with message streaming
- ✅ Proper sync of streaming content to disk

## Phase 5: Provider Settings

### Goal
Implement provider configuration in moly-settings app.

### Steps

**5.1 Provider UI**
- Port settings screens from moly-ai
- Adapt to MoFA styling
- Provider list panel
- Provider detail view
- Add provider modal

**5.2 Provider Logic**
- Port provider models to Store
- Implement connection testing
- Fetch models from providers
- Store provider preferences

**5.3 Verification**
- Add custom provider
- Test connection
- See available models
- Enable/disable providers

### Files to Create/Modify (Phase 5)

```
apps/moly-settings/
└── src/
    ├── lib.rs
    ├── providers_screen.rs
    └── ...

moly-shell/src/data/
├── store.rs                                   # Add providers
├── provider.rs                                # Provider models
└── supported_providers.json                   # Provider whitelist
```

## Phase 6: Model Discovery & Downloads ✅ COMPLETED

### Goal
Implement MolyServer integration for model discovery/download.

### Status
✅ **COMPLETED** - Full model discovery and download management implemented.

### Steps

**6.1 MolyClient** ✅ COMPLETED
- ✅ Created MolyClient in moly-data crate
- ✅ Added to Store (moly-data/src/store.rs)
- ✅ Connection status tracking (Disconnected/Connecting/Connected/Error)
- ✅ Methods: test_connection, get_featured_models, search_models, get_downloaded_files, get_pending_downloads, download_file, pause_download, cancel_download, delete_file

**6.2 Models UI** ✅ COMPLETED
- ✅ Header with "Model Discovery" title and connection status badge
- ✅ Search bar with refresh button
- ✅ ModelCard component with:
  - Model name, size, download count, like count
  - Summary (truncated to 200 chars)
  - Architecture and author info
  - Files section with download button
- ✅ DownloadItem component with progress bar (green theme)
- ✅ Empty/loading/error states
- ✅ Full dark mode support

**6.3 Download Management** ✅ COMPLETED
- ✅ Download button on each model card
- ✅ Async task handling with std::thread + tokio runtime
- ✅ Progress tracking via polling (500ms interval)
- ✅ Active downloads section showing current progress
- ✅ Download status display (Initializing/Downloading %/Paused/Error)

**6.4 Verification** ✅ COMPLETED
- ✅ Browse featured models (when Moly Server is running)
- ✅ Search models by query
- ✅ Start downloads via download button
- ✅ Track progress in active downloads section

### Implementation Notes

**Architecture Decision**: MolyClient lives in moly-data crate (not moly-shell) for shared access:
```rust
// moly-data/src/store.rs
pub struct Store {
    pub moly_client: MolyClient,
    // ... other fields
}
```

**Async Pattern**: Uses std::thread with tokio runtime (same as moly-ai):
```rust
std::thread::spawn(move || {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        // async operations
    });
});
```

**Task Result Pattern**: Shared Arc<Mutex<Option<TaskResult>>> for async communication:
```rust
type TaskResultState = Arc<Mutex<Option<ModelsTaskResult>>>;

enum ModelsTaskResult {
    ConnectionResult(Result<(), String>),
    ModelsResult(Result<Vec<Model>, String>),
    DownloadStarted(Result<FileId, String>),
    DownloadsUpdate(Result<Vec<PendingDownload>, String>),
}
```

**Hex Color Limitation**: Colors with 'e' followed by digits (e.g., #2563eb) are interpreted as scientific notation in live_design!. Use vec4() instead:
```rust
// Instead of: let color = mix(#3b82f6, #2563eb, self.dark_mode);
let light = vec4(0.231, 0.510, 0.965, 1.0);  // #3b82f6
let dark = vec4(0.145, 0.388, 0.922, 1.0);   // #2563eb
let color = mix(light, dark, self.dark_mode);
```

### Files Created/Modified (Phase 6)

```
moly-data/
├── Cargo.toml                                 # Added moly-protocol, reqwest, futures
├── src/
│   ├── lib.rs                                 # Added moly_client module, Model/File exports
│   ├── moly_client.rs                         # NEW: HTTP client for Moly Server
│   └── store.rs                               # Added MolyClient field

apps/moly-models/
├── Cargo.toml                                 # Added moly-protocol, tokio, chrono
└── src/
    ├── lib.rs                                 # Screen module export
    └── screen/
        ├── mod.rs                             # ModelsApp widget implementation
        └── design.rs                          # UI: ModelCard, DownloadItem, StatusBadge
```

## Phase 7: MCP Integration

### Goal
Implement Model Context Protocol support (desktop only).

### Steps

**7.1 MCP Configuration**
- Port MCP settings from moly-ai
- Server configuration UI
- Environment variables
- Enable/disable servers

**7.2 MCP Tool Manager**
- Port tool manager from aitk
- Integrate with BotContext
- Tool execution in chats

**7.3 Verification**
- Configure MCP servers
- See available tools in chat
- Execute tool calls

### Files to Create/Modify (Phase 7)

```
apps/moly-mcp/
└── src/
    ├── lib.rs
    ├── mcp_screen.rs
    └── ...

moly-shell/src/data/
├── store.rs                                   # Add mcp_config
└── mcp.rs                                     # MCP models
```

## Critical Files Summary

### Phase 1 (Shell) - ~10 files
- Cargo.toml (workspace)
- moly-shell/Cargo.toml, main.rs, app.rs
- moly-widgets/Cargo.toml, lib.rs, theme.rs
- Resources (fonts, icons)

### Phase 2 (Plugins) - ~10 files
- 4 app Cargo.toml files
- 4 app lib.rs files
- Updated shell app.rs
- Plugin trait definition

### Phase 3 (Store) - ~6 files
- data/ module with store, preferences, actions
- Updated app.rs

### Phase 4 (Chat) - ~15 files
- chat app modules
- Store updates for chats, bot_context
- Chat models

### Phase 5 (Settings) - ~10 files
- settings app modules
- Store updates for providers
- Provider models

### Phase 6 (Models) - ~12 files
- models app modules
- Store updates for search, downloads
- MolyClient

### Phase 7 (MCP) - ~8 files
- mcp app modules
- Store updates for mcp config

**Total: ~70-80 files across all phases**

## Key Implementation Notes

### Dark Mode
- Use shader-based mixing like MoFA (instance dark_mode: 0.0)
- Animate transitions (300ms cubic ease-out)
- Propagate from shell to all apps/widgets

### Async Pattern
- Use aitk's spawn for cross-platform async
- Deferred UI updates via app_runner().defer()
- Store methods are async where needed

### State Synchronization
- Shell posts StoreUpdate actions after Store changes
- Apps listen for StoreUpdate, redraw on changes
- Prevents stale UI state

### Resource Management
- Copy fonts from mofa-studio
- Create new icons for Chat, Models, MCP
- Reuse moly-ai icons where applicable

## Testing Strategy

Each phase should be verified before moving to next:
1. **Visual Testing**: Run app, test UI interactions
2. **Dark Mode**: Toggle theme, verify all elements update
3. **Navigation**: Switch between apps, verify state persists
4. **Functionality**: Test core features (chat, downloads, etc.)
5. **Persistence**: Restart app, verify data loads correctly

## Risk Mitigation

### High-Risk Areas
1. **Store sharing between apps**: May need custom event types
2. **Async runtime**: Ensure spawn works on all platforms
3. **MolyServer connection**: Handle server not running gracefully
4. **MCP desktop-only**: Conditional compilation may be tricky

### Mitigation Strategies
1. Start with Store in shell, pass via scope parameter
2. Use aitk's spawn abstraction from day 1
3. Add connection status UI, retry logic
4. Use #[cfg(not(target_arch = "wasm32"))] early

## Timeline Estimate (Not Strict)

- Phase 1: Runnable shell - Foundation work
- Phase 2: Plugin system - Architecture setup
- Phase 3: Store pattern - Core infrastructure
- Phase 4: Chat feature - Major feature
- Phase 5: Provider settings - Medium feature
- Phase 6: Model discovery - Major feature
- Phase 7: MCP integration - Medium feature

Recommend completing Phases 1-3 before asking user for feedback on architecture, then continuing with features.
