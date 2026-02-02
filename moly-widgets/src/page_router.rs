//! # PageRouter - Centralized Navigation
//!
//! Provides centralized page navigation with history support.
//! Based on mofa-studio's PageRouter pattern.
//!
//! ## Features
//! - Navigation history with back support
//! - Returns pages to hide for TimerControl integration
//! - Page stack for nested navigation

use makepad_widgets::LiveId;

/// Centralized page navigation manager
///
/// Tracks current page and navigation history, returns pages that need
/// to be hidden when navigating (for TimerControl integration).
#[derive(Clone, Debug)]
pub struct PageRouter {
    /// Currently visible page
    current_page: LiveId,
    /// Navigation history stack
    page_stack: Vec<LiveId>,
    /// All registered pages (for calculating pages_to_hide)
    all_pages: Vec<LiveId>,
}

impl Default for PageRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl PageRouter {
    /// Create a new empty router
    pub fn new() -> Self {
        Self {
            current_page: LiveId::empty(),
            page_stack: Vec::new(),
            all_pages: Vec::new(),
        }
    }

    /// Create a router with initial page and all available pages
    pub fn with_pages(initial_page: LiveId, all_pages: Vec<LiveId>) -> Self {
        Self {
            current_page: initial_page,
            page_stack: vec![initial_page],
            all_pages,
        }
    }

    /// Register a page with the router
    pub fn register_page(&mut self, page_id: LiveId) {
        if !self.all_pages.contains(&page_id) {
            self.all_pages.push(page_id);
        }
    }

    /// Set the initial page (without adding to history)
    pub fn set_initial_page(&mut self, page_id: LiveId) {
        self.current_page = page_id;
        self.page_stack.clear();
        self.page_stack.push(page_id);
    }

    /// Navigate to a new page
    ///
    /// Returns the list of pages that should be hidden (for TimerControl).
    pub fn navigate_to(&mut self, page_id: LiveId) -> Vec<LiveId> {
        if self.current_page == page_id {
            return Vec::new(); // Already on this page
        }

        let pages_to_hide = self.pages_to_hide(page_id);
        self.page_stack.push(page_id);
        self.current_page = page_id;
        pages_to_hide
    }

    /// Navigate back to the previous page
    ///
    /// Returns Some(previous_page, pages_to_hide) if there's history,
    /// or None if at the root.
    pub fn navigate_back(&mut self) -> Option<(LiveId, Vec<LiveId>)> {
        if self.page_stack.len() <= 1 {
            return None; // Can't go back from root
        }

        self.page_stack.pop(); // Remove current
        let previous = *self.page_stack.last()?;
        let pages_to_hide = self.pages_to_hide(previous);
        self.current_page = previous;
        Some((previous, pages_to_hide))
    }

    /// Get pages that should be hidden when navigating to a new page
    pub fn pages_to_hide(&self, new_page: LiveId) -> Vec<LiveId> {
        self.all_pages
            .iter()
            .filter(|&&p| p != new_page)
            .copied()
            .collect()
    }

    /// Get the current page
    pub fn current_page(&self) -> LiveId {
        self.current_page
    }

    /// Check if we can navigate back
    pub fn can_go_back(&self) -> bool {
        self.page_stack.len() > 1
    }

    /// Get the navigation history depth
    pub fn history_depth(&self) -> usize {
        self.page_stack.len()
    }

    /// Clear navigation history (keep current page)
    pub fn clear_history(&mut self) {
        let current = self.current_page;
        self.page_stack.clear();
        self.page_stack.push(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_page(name: &str) -> LiveId {
        LiveId::from_str_with_lut(name).unwrap()
    }

    #[test]
    fn test_navigation() {
        let chat = test_page("chat");
        let settings = test_page("settings");
        let models = test_page("models");

        let mut router = PageRouter::with_pages(chat, vec![chat, settings, models]);

        assert_eq!(router.current_page(), chat);
        assert!(!router.can_go_back());

        // Navigate to settings
        let hidden = router.navigate_to(settings);
        assert_eq!(router.current_page(), settings);
        assert!(hidden.contains(&chat));
        assert!(hidden.contains(&models));
        assert!(!hidden.contains(&settings));

        // Navigate back
        let (prev, _) = router.navigate_back().unwrap();
        assert_eq!(prev, chat);
        assert_eq!(router.current_page(), chat);
    }

    #[test]
    fn test_same_page_navigation() {
        let chat = test_page("chat");
        let mut router = PageRouter::with_pages(chat, vec![chat]);

        let hidden = router.navigate_to(chat);
        assert!(hidden.is_empty()); // No pages hidden when navigating to same page
    }
}
