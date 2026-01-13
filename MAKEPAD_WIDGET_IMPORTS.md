# Makepad Widget Import Limitations

## The Problem

When trying to import custom widgets from separate crates in Makepad, you may encounter the error:
```
Error parsing live file: Can't find live definition
```

This happens because **Makepad's `live_design!` macro does not support importing widgets from external crates** using standard Rust import patterns.

## What Doesn't Work

### ❌ Separate Crates with Standard Imports
```rust
// In moly-shell/src/app.rs - THIS DOESN'T WORK
live_design! {
    use moly_chat::*;  // ❌ Error: expected ident, unexpected token *
    use moly_chat::ChatApp;  // ❌ Error: expected ident
}
```

### ❌ Import Keyword
```rust
live_design! {
    import moly_chat;  // ❌ Error: Unexpected assign_type
    import moly_chat::ChatApp;  // ❌ Error: Unexpected assign_type
}
```

### ❌ Fully Qualified Paths
```rust
live_design! {
    App = <View> {
        chat = <moly_chat::ChatApp> {}  // ❌ Error: expected > unexpected token ::
    }
}
```

## What Works

### ✅ Internal Modules Pattern

The **only supported approach** is to use internal modules within the same crate:

```
moly-shell/
├── src/
│   ├── main.rs
│   ├── app.rs
│   └── apps/
│       ├── mod.rs
│       ├── chat.rs      // Widget defined here
│       ├── models.rs
│       ├── settings.rs
│       └── mcp.rs
└── Cargo.toml
```

In your code:

```rust
// In moly-shell/src/main.rs
mod app;
mod apps;  // Register the apps module

// In moly-shell/src/apps/mod.rs
pub mod chat;
pub mod models;
pub mod settings;
pub mod mcp;

// In moly-shell/src/apps/chat.rs
use makepad_widgets::*;

live_design! {
    pub ChatApp = {{ChatApp}} {
        // Widget definition
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatApp {
    #[deref]
    pub view: View,
}

// In moly-shell/src/app.rs
use makepad_widgets::*;

live_design! {
    // This works because it's within the same crate
    use crate::apps::chat::*;     // ✅
    use crate::apps::models::*;   // ✅
    use crate::apps::settings::*; // ✅
    use crate::apps::mcp::*;      // ✅

    App = <View> {
        // Can now use widgets directly
        chat_app = <ChatApp> {}       // ✅
        models_app = <ModelsApp> {}   // ✅
    }
}

// Don't forget to register in LiveRegister
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::apps::chat::live_design(cx);
        crate::apps::models::live_design(cx);
        crate::apps::settings::live_design(cx);
        crate::apps::mcp::live_design(cx);
    }
}
```

## Why This Limitation Exists

The `live_design!` macro is a custom DSL (Domain Specific Language) that gets processed by Makepad's live compiler before Rust compilation. It has its own parser that doesn't understand Rust's full module system or path resolution. The DSL only supports:

1. `use` statements for items **within the same crate** (`use crate::...`)
2. `use` statements for external crate **theme/shader/widget** exports that have been properly registered
3. Direct widget references (like `<Button>`) without namespace qualifiers

## Alternative: Keep Separate Crates for Code Organization

If you want to maintain separate crates for organizational purposes (e.g., for testing, documentation, or potential future use), you can:

1. Keep the separate crates in `apps/moly-chat`, `apps/moly-models`, etc.
2. **Also** maintain copies as internal modules in `moly-shell/src/apps/`
3. Use symbolic links or build scripts to sync the code

However, this adds complexity and is generally not recommended unless you have a specific reason.

## Best Practice

**Use internal modules for all Makepad widgets within your application.** Only create separate crates for:
- Shared utility code (doesn't use `live_design!`)
- Theme definitions
- Non-widget components

## See Also

- Makepad Skills: /makepad-evolution (self-improving skill system)
- Implementation Plan: [Implementation_Plan.md](./Implementation_Plan.md)
- Development Guide: [CLAUDE.md](./CLAUDE.md)
