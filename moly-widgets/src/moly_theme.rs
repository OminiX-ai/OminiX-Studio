//! # MolyTheme - Runtime Theme with Animation
//!
//! Provides runtime theme state with smooth dark mode transitions.
//! Based on mofa-studio's two-tier theme system:
//! - Static: Color constants in live_design! (theme.rs)
//! - Runtime: MolyTheme struct with animation state

use makepad_widgets::Cx;

/// Duration in seconds for theme transition animation
pub const THEME_TRANSITION_DURATION: f64 = 0.25;

/// Animation speed factor (higher = faster)
const ANIMATION_SPEED: f64 = 0.15;

/// Threshold for considering animation complete
const ANIMATION_THRESHOLD: f64 = 0.01;

/// Runtime theme state with animation support
///
/// Use `dark_mode_anim` in shaders for smooth transitions:
/// ```ignore
/// fn get_bg_color(self) -> vec4 {
///     return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct MolyTheme {
    /// Current dark mode state (true = dark, false = light)
    pub dark_mode: bool,
    /// Animated value for smooth transitions (0.0 = light, 1.0 = dark)
    /// Use this value in shaders via `instance dark_mode`
    pub dark_mode_anim: f64,
}

impl Default for MolyTheme {
    fn default() -> Self {
        Self {
            dark_mode: false,
            dark_mode_anim: 0.0,
        }
    }
}

impl MolyTheme {
    /// Create a new theme with specified dark mode
    pub fn new(dark_mode: bool) -> Self {
        let anim = if dark_mode { 1.0 } else { 0.0 };
        Self {
            dark_mode,
            dark_mode_anim: anim,
        }
    }

    /// Toggle dark mode and start animation
    pub fn toggle_dark_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
    }

    /// Set dark mode state
    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.dark_mode = dark_mode;
    }

    /// Step the animation forward
    ///
    /// Call this in your NextFrame handler. Returns true if animation
    /// is still in progress and needs another frame.
    ///
    /// # Example
    /// ```ignore
    /// Event::NextFrame(_) => {
    ///     if self.theme.animate_step(cx) {
    ///         self.ui.redraw(cx);
    ///     }
    /// }
    /// ```
    pub fn animate_step(&mut self, cx: &mut Cx) -> bool {
        let target = if self.dark_mode { 1.0 } else { 0.0 };
        let diff = target - self.dark_mode_anim;

        if diff.abs() < ANIMATION_THRESHOLD {
            // Animation complete
            self.dark_mode_anim = target;
            false
        } else {
            // Continue animating
            self.dark_mode_anim += diff * ANIMATION_SPEED;
            cx.new_next_frame();
            true
        }
    }

    /// Check if animation is currently in progress
    pub fn is_animating(&self) -> bool {
        let target = if self.dark_mode { 1.0 } else { 0.0 };
        (target - self.dark_mode_anim).abs() >= ANIMATION_THRESHOLD
    }

    /// Get the current animation value (0.0 to 1.0)
    pub fn anim_value(&self) -> f64 {
        self.dark_mode_anim
    }

    /// Instantly snap to target state without animation
    pub fn snap_to_target(&mut self) {
        self.dark_mode_anim = if self.dark_mode { 1.0 } else { 0.0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_default() {
        let theme = MolyTheme::default();
        assert!(!theme.dark_mode);
        assert_eq!(theme.dark_mode_anim, 0.0);
    }

    #[test]
    fn test_theme_toggle() {
        let mut theme = MolyTheme::default();
        theme.toggle_dark_mode();
        assert!(theme.dark_mode);
        // Animation value hasn't changed yet (needs animate_step)
        assert_eq!(theme.dark_mode_anim, 0.0);
    }

    #[test]
    fn test_theme_snap() {
        let mut theme = MolyTheme::default();
        theme.toggle_dark_mode();
        theme.snap_to_target();
        assert_eq!(theme.dark_mode_anim, 1.0);
    }
}
