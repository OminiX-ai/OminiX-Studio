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

## Phase 2: App Plugin System

### Goal
Implement MoFA's plugin architecture and create empty app containers.

### Steps

**2.1 Plugin Trait (moly-shell/src/app.rs)**
```rust
pub trait MolyApp {
    fn app_id(&self) -> &str;
    fn app_name(&self) -> &str;
    fn app_icon(&self) -> LiveDependency;
    fn handle_event(&mut self, cx: &mut Cx, event: &Event);
    fn draw(&mut self, cx: &mut Cx2d, scope: &mut Scope) -> DrawStep;
}
```

**2.2 App Containers**
- Create apps/moly-chat/ (empty placeholder)
- Create apps/moly-models/ (empty placeholder)
- Create apps/moly-settings/ (empty placeholder)
- Create apps/moly-mcp/ (empty placeholder, desktop only)

**2.3 Shell Integration**
- Load apps in shell
- Wire sidebar to switch apps
- Content area displays active app

**2.4 Verification**
- Sidebar navigation switches between empty app screens
- Each app shows placeholder text

### Files to Create (Phase 2)

```
apps/
├── moly-chat/
│   ├── Cargo.toml
│   └── src/lib.rs                             # Empty MolyApp impl
├── moly-models/
│   ├── Cargo.toml
│   └── src/lib.rs
├── moly-settings/
│   ├── Cargo.toml
│   └── src/lib.rs
└── moly-mcp/
    ├── Cargo.toml
    └── src/lib.rs

moly-shell/src/
└── app.rs                                     # Updated with plugin loading
```

## Phase 3: Shared State & Store

### Goal
Implement Moly's Store pattern in the shell, enable app communication.

### Steps

**3.1 Store Implementation (moly-shell/src/data/)**
- Copy Store structure from moly-ai
- Implement async loading
- Add preferences persistence
- Create StoreAction enum

**3.2 Store Integration**
- Shell owns Store instance
- Apps receive Store reference via custom events
- Apps post actions to modify Store
- Shell propagates Store updates to apps

**3.3 Verification**
- Store loads preferences on startup
- Apps can read/write Store
- Preferences persist across restarts

### Files to Create (Phase 3)

```
moly-shell/src/
├── data/
│   ├── mod.rs
│   ├── store.rs                               # Store pattern
│   ├── preferences.rs                         # User preferences
│   └── actions.rs                             # StoreAction enum
└── app.rs                                     # Updated with Store
```

## Phase 4: Chat Feature

### Goal
Implement full chat functionality in moly-chat app.

### Steps

**4.1 Dependencies**
- Add aitk, moly-kit to moly-chat
- Add moly-protocol if needed

**4.2 Chat UI (apps/moly-chat/src/)**
- Port ChatScreen from moly-ai
- Adapt to MoFA styling (use moly-widgets theme)
- Integrate with Store for chat persistence
- Add ChatController

**4.3 BotContext**
- Port bot_context module
- Implement in Store
- Wire to chat app

**4.4 Verification**
- Create new chats
- Send messages (with provider configured)
- Chat history persists
- Multiple chats work

### Files to Create/Modify (Phase 4)

```
apps/moly-chat/
├── Cargo.toml                                 # Add aitk, moly-kit
└── src/
    ├── lib.rs                                 # MolyApp impl
    ├── chat_screen.rs                         # Main chat UI
    └── ...                                    # Other chat modules

moly-shell/src/data/
├── store.rs                                   # Add chats, bot_context
└── chat.rs                                    # Chat models
```

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

## Phase 6: Model Discovery & Downloads

### Goal
Implement MolyServer integration for model discovery/download.

### Steps

**6.1 MolyClient**
- Port MolyClient from moly-ai
- Add to Store
- Implement connection checking

**6.2 Models UI**
- Port landing screen from moly-ai
- Model discovery, search, filtering
- Download management UI
- Progress tracking

**6.3 Verification**
- Browse featured models
- Search models
- Download models
- Track progress

### Files to Create/Modify (Phase 6)

```
apps/moly-models/
└── src/
    ├── lib.rs
    ├── discovery_screen.rs
    ├── downloads_screen.rs
    └── ...

moly-shell/src/data/
├── store.rs                                   # Add search, downloads
├── moly_client.rs                             # HTTP client
└── models.rs                                  # Model data structures
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
