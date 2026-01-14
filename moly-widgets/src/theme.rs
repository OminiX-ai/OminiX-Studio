//! # Theme System
//!
//! Centralized color palette, fonts, and dark mode support for Moly.
//! Based on MoFA Studio's theme system with Tailwind-inspired colors.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // ========================================================================
    // COLOR PALETTE
    // Based on Tailwind CSS color system for consistency
    // ========================================================================

    // --- Semantic Colors (use these first) ---
    pub DARK_BG = #f5f7fa          // Main background
    pub PANEL_BG = #ffffff         // Card/panel background
    pub ACCENT_BLUE = #3b82f6      // Primary action color
    pub ACCENT_GREEN = #10b981     // Success/positive
    pub ACCENT_RED = #ef4444       // Error/danger
    pub ACCENT_YELLOW = #f59f0b    // Warning
    pub ACCENT_INDIGO = #6366f1    // Secondary accent
    pub TEXT_PRIMARY = #1f2937     // Main text (gray-800)
    pub TEXT_SECONDARY = #6b7280   // Secondary text (gray-500)
    pub TEXT_MUTED = #9ca3af       // Muted/disabled text (gray-400)
    pub DIVIDER = #e2e8f0          // Divider lines (slate-200)
    pub BORDER = #e5e7eb           // Border color (gray-200)
    pub HOVER_BG = #f1f5f9         // Hover background (slate-100)

    // --- White ---
    pub WHITE = #ffffff

    // --- Slate (cool gray, used for backgrounds) ---
    pub SLATE_50 = #f8fafc
    pub SLATE_100 = #f1f5f9
    pub SLATE_200 = #e2e8f0
    pub SLATE_300 = #cbd5e1
    pub SLATE_400 = #94a3b8
    pub SLATE_500 = #64748b
    pub SLATE_600 = #475569
    pub SLATE_700 = #334155
    pub SLATE_800 = #1f293b
    pub SLATE_900 = #0f172a
    pub SLATE_950 = #0d1117

    // --- Gray (neutral gray, used for text/icons) ---
    pub GRAY_50 = #f9fafb
    pub GRAY_100 = #f3f4f6
    pub GRAY_200 = #e5e7eb
    pub GRAY_300 = #d1d5db
    pub GRAY_400 = #9ca3af
    pub GRAY_500 = #6b7280
    pub GRAY_600 = #4b5563
    pub GRAY_700 = #374151
    pub GRAY_800 = #1f2937
    pub GRAY_900 = #111827

    // --- Blue (primary actions) ---
    pub BLUE_50 = #eff6ff
    pub BLUE_100 = #dbeafe
    pub BLUE_200 = #bfdbfe
    pub BLUE_300 = #93c5fd
    pub BLUE_400 = #60a5fa
    pub BLUE_500 = #3b82f6
    pub BLUE_600 = #2565fb
    pub BLUE_700 = #1d4fd8
    pub BLUE_800 = #1f40af
    pub BLUE_900 = #1f3a8a

    // --- Indigo (secondary accent) ---
    pub INDIGO_50 = #eef2ff
    pub INDIGO_100 = #e1e7ff
    pub INDIGO_200 = #c7d2ff
    pub INDIGO_300 = #a5b4fc
    pub INDIGO_400 = #818cf8
    pub INDIGO_500 = #6366f1
    pub INDIGO_600 = #4f47e5
    pub INDIGO_700 = #4338ca
    pub INDIGO_800 = #3730a3
    pub INDIGO_900 = #312f81

    // --- Green (success) ---
    pub GREEN_50 = #f0fdf4
    pub GREEN_100 = #dcfcf7
    pub GREEN_200 = #bbf7d0
    pub GREEN_300 = #88ffac
    pub GREEN_400 = #4adf80
    pub GREEN_500 = #22c55f
    pub GREEN_600 = #16a34a
    pub GREEN_700 = #15803d
    pub GREEN_800 = #166534
    pub GREEN_900 = #14532d

    // --- Emerald (alternate green) ---
    pub EMERALD_500 = #10b981
    pub EMERALD_600 = #059669
    pub EMERALD_700 = #047857

    // --- Red (error/danger) ---
    pub RED_50 = #fff2f2
    pub RED_100 = #fff2f2
    pub RED_200 = #ffcaca
    pub RED_300 = #fca5a5
    pub RED_400 = #f87171
    pub RED_500 = #ef4444
    pub RED_600 = #dc2626
    pub RED_700 = #b91c1c
    pub RED_800 = #991b1b
    pub RED_900 = #7f1d1d

    // --- Yellow/Amber (warning) ---
    pub YELLOW_500 = #eab308
    pub AMBER_500 = #f59f0b

    // --- Orange ---
    pub ORANGE_500 = #f97316

    // --- Transparent ---
    pub TRANSPARENT = #00000000

    // ========================================================================
    // DARK THEME VARIANTS
    // Use with mix(LIGHT_COLOR, DARK_COLOR, dark_mode) in shaders
    // ========================================================================

    // --- Dark Theme Semantic Colors ---
    pub DARK_BG_DARK = #0f172a         // Main background (dark)
    pub PANEL_BG_DARK = #1f293b        // Card/panel background (dark)
    pub TEXT_PRIMARY_DARK = #f1f5f9    // Main text (dark)
    pub TEXT_SECONDARY_DARK = #94a3b8  // Secondary text (dark)
    pub TEXT_MUTED_DARK = #64748b      // Muted text (dark)
    pub DIVIDER_DARK = #475569         // Divider lines (dark)
    pub BORDER_DARK = #334155          // Border color (dark)
    pub HOVER_BG_DARK = #334155        // Hover background (dark)
    pub ACCENT_BLUE_DARK = #60a5fa     // Primary action (brighter for dark mode)

    // ========================================================================
    // THEMEABLE WIDGET BASE
    // Base widget with dark_mode instance for theme switching
    // ========================================================================

    pub ThemeableView = <View> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn get_bg_color(self) -> vec4 {
                return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
            }

            fn pixel(self) -> vec4 {
                return self.get_bg_color();
            }
        }
    }

    pub ThemeableRoundedView = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 4.0

            fn get_bg_color(self) -> vec4 {
                return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
            }
        }
    }
}
