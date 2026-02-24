use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::*;

    // ========================================================================
    // SHARED TEXT STYLES
    // Reusable label templates with consistent typography
    // ========================================================================

    // Section title: semibold, 16px, primary color
    pub SectionTitle = <Label> {
        draw_text: {
            color: (TEXT_PRIMARY)
            text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
        }
    }

    // Body text: regular, 11px, secondary gray
    pub BodyText = <Label> {
        draw_text: {
            color: (GRAY_700)
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
        }
    }

    // Hint/muted text: regular, 10px, muted gray
    pub HintText = <Label> {
        draw_text: {
            color: (TEXT_MUTED)
            text_style: <FONT_REGULAR>{ font_size: 10.0 }
        }
    }

    // ========================================================================
    // CHAT LIST ITEM
    // Sidebar chat history item with hover and selected states
    // ========================================================================

    pub ChatListItem = <View> {
        width: Fill, height: 32
        padding: {left: 8, right: 8}
        align: {y: 0.5}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let base = mix((WHITE), #0f172a, self.dark_mode);
                let hover_color = mix((HOVER_BG), #1a2332, self.dark_mode);
                let selected_color = mix((BLUE_100), #1a2845, self.dark_mode);
                return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
            }
        }
        title = <Label> {
            width: Fill
            draw_text: {
                color: (GRAY_700)
                text_style: { font_size: 11.0 }
                wrap: Ellipsis
            }
        }
    }
}
