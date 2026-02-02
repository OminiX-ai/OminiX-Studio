//! Models Screen UI Design

use makepad_widgets::*;

use super::ModelsApp;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    // Search input style
    SearchInput = <TextInput> {
        width: Fill, height: 44
        padding: {left: 40, right: 12, top: 10, bottom: 10}
        empty_text: "Search models..."

        draw_bg: {
            instance radius: 8.0
            instance border_width: 1.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                sdf.box(1.0, 1.0, sz.x, sz.y, max(1.0, self.radius - self.border_width));

                let bg = mix(#ffffff, #1e293b, self.dark_mode);
                let border = mix(#d1d5db, #475569, self.dark_mode);
                sdf.fill(bg);
                sdf.stroke(border, self.border_width);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#1f2937, #f1f5f9, self.dark_mode);
            }
            text_style: <FONT_REGULAR>{ font_size: 13.0 }
        }
    }

    // Model card component
    ModelCard = <View> {
        width: Fill, height: Fit
        padding: 16
        margin: {bottom: 12}
        show_bg: true
        flow: Down
        spacing: 12

        draw_bg: {
            instance radius: 8.0
            instance dark_mode: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);

                let bg = mix(#ffffff, #1e293b, self.dark_mode);
                let hover_bg = mix(#f8fafc, #334155, self.dark_mode);
                let border = mix(#e5e7eb, #374151, self.dark_mode);

                sdf.fill(mix(bg, hover_bg, self.hover));
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }

        // Header row with name and stats
        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            spacing: 8

            model_name = <Label> {
                width: Fit
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                }
            }

            model_size = <Label> {
                width: Fit
                margin: {left: 8}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                }
            }

            <View> { width: Fill } // Spacer

            download_count = <Label> {
                width: Fit
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }

            like_count = <Label> {
                width: Fit
                margin: {left: 12}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }
        }

        // Summary
        model_summary = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#4b5563, #cbd5e1, self.dark_mode);
                }
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                wrap: Word
            }
        }

        // Info row
        info_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 16

            architecture = <Label> {
                width: Fit
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }

            author = <Label> {
                width: Fit
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#3b82f6, #60a5fa, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }
        }

        // Files section with download button
        files_section = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12
            margin: {top: 8}
            padding: {top: 8}
            align: {y: 0.5}

            files_label = <Label> {
                width: Fill
                text: "1 file(s) available"
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }

            download_btn = <Button> {
                width: Fit, height: 32
                padding: {left: 16, right: 16}

                draw_bg: {
                    instance hover: 0.0
                    instance pressed: 0.0
                    instance radius: 6.0
                    instance dark_mode: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let sz = self.rect_size - 2.0;
                        // Blue colors: #3b82f6, #2563eb, #1d4ed8
                        let light_base = vec4(0.231, 0.510, 0.965, 1.0);
                        let dark_base = vec4(0.145, 0.388, 0.922, 1.0);
                        let light_hover = vec4(0.145, 0.388, 0.922, 1.0);
                        let dark_hover = vec4(0.114, 0.306, 0.847, 1.0);
                        let base_color = mix(light_base, dark_base, self.dark_mode);
                        let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                        let color = mix(base_color, hover_color, self.hover);
                        sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                        sdf.fill(color);
                        return sdf.result;
                    }
                }

                draw_text: {
                    color: #ffffff
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }

                text: "Download"
            }
        }
    }

    // File item in model card
    FileItem = <View> {
        width: Fill, height: Fit
        padding: {left: 8, right: 8, top: 6, bottom: 6}
        flow: Right
        align: {y: 0.5}
        spacing: 8
        show_bg: true

        draw_bg: {
            instance radius: 4.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                let bg = mix(#f3f4f6, #0f172a, self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        file_name = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#1f2937, #f1f5f9, self.dark_mode);
                }
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
            }
        }

        file_size = <Label> {
            width: Fit
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#6b7280, #94a3b8, self.dark_mode);
                }
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
            }
        }

        file_quant = <Label> {
            width: Fit
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#8b5cf6, #a78bfa, self.dark_mode);
                }
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
            }
        }

        download_btn = <Button> {
            width: Fit, height: 24
            padding: {left: 10, right: 10}

            draw_bg: {
                instance hover: 0.0
                instance pressed: 0.0
                instance radius: 4.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let sz = self.rect_size - 2.0;
                    let base_color = vec4(0.231, 0.510, 0.965, 1.0);
                    let hover_color = vec4(0.145, 0.388, 0.922, 1.0);
                    let color = mix(base_color, hover_color, self.hover);
                    sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                    sdf.fill(color);
                    return sdf.result;
                }
            }

            draw_text: {
                color: #ffffff
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
            }

            text: "Download"
        }
    }

    // Download progress item
    DownloadItem = <View> {
        width: Fill, height: Fit
        padding: 12
        show_bg: true
        flow: Down
        spacing: 8

        draw_bg: {
            instance radius: 6.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                let bg = mix(#f0fdf4, #14532d, self.dark_mode);
                let border = mix(#bbf7d0, #166534, self.dark_mode);
                sdf.fill(bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }

        // File name and progress text
        download_header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            download_name = <Label> {
                width: Fill
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        // #166534 = rgb(22, 101, 52), #86efac = rgb(134, 239, 172)
                        let light = vec4(0.086, 0.396, 0.204, 1.0);
                        let dark = vec4(0.525, 0.937, 0.675, 1.0);
                        return mix(light, dark, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                }
            }

            download_progress_text = <Label> {
                width: Fit
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        // #15803d = rgb(21, 128, 61), #4ade80 = rgb(74, 222, 128)
                        let light = vec4(0.082, 0.502, 0.239, 1.0);
                        let dark = vec4(0.290, 0.871, 0.502, 1.0);
                        return mix(light, dark, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }
        }

        // Progress bar
        progress_bar_bg = <View> {
            width: Fill, height: 6
            show_bg: true

            draw_bg: {
                instance radius: 3.0
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let sz = self.rect_size - 2.0;
                    sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                    let bg = mix(#dcfce7, #052e16, self.dark_mode);
                    sdf.fill(bg);
                    return sdf.result;
                }
            }

            progress_bar_fill = <View> {
                width: 0, height: Fill
                show_bg: true

                draw_bg: {
                    instance radius: 3.0
                    instance progress: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let sz = self.rect_size - 2.0;
                        sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                        // Green gradient for progress
                        let color = vec4(0.133, 0.545, 0.133, 1.0); // #22c55e
                        sdf.fill(color);
                        return sdf.result;
                    }
                }
            }
        }
    }

    // Connection status badge
    StatusBadge = <View> {
        width: Fit, height: Fit
        padding: {left: 8, right: 8, top: 4, bottom: 4}
        show_bg: true

        draw_bg: {
            instance radius: 4.0
            instance status: 0.0  // 0=disconnected, 1=connecting, 2=connected, 3=error

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);

                // Colors: gray, blue, green, red
                let disconnected = #9ca3af;
                let connecting = #3b82f6;
                let connected = #22c55e;
                let error = #ef4444;

                let color = mix(
                    mix(disconnected, connecting, clamp(self.status, 0.0, 1.0)),
                    mix(connected, error, clamp(self.status - 2.0, 0.0, 1.0)),
                    step(1.5, self.status)
                );

                sdf.fill(color);
                return sdf.result;
            }
        }

        status_text = <Label> {
            draw_text: {
                color: #ffffff
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
            }
        }
    }

    pub ModelsApp = {{ModelsApp}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true

        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f5f7fa, #0f172a, self.dark_mode);
            }
        }

        // Header with search
        header = <View> {
            width: Fill, height: Fit
            padding: 20
            flow: Down
            spacing: 16

            // Title row
            title_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}

                title_label = <Label> {
                    text: "Model Discovery"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#1f2937, #f1f5f9, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 24.0 }
                    }
                }

                <View> { width: Fill } // Spacer

                status_badge = <StatusBadge> {
                    status_text = { text: "Disconnected" }
                }
            }

            // Search bar
            search_section = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 12

                search_container = <View> {
                    width: Fill, height: Fit

                    search_input = <SearchInput> {}
                }

                refresh_btn = <Button> {
                    width: 44, height: 44
                    padding: 0

                    draw_bg: {
                        instance hover: 0.0
                        instance pressed: 0.0
                        instance radius: 8.0
                        instance dark_mode: 0.0

                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let sz = self.rect_size - 2.0;
                            let bg = mix(#ffffff, #1e293b, self.dark_mode);
                            let hover_bg = mix(#f3f4f6, #334155, self.dark_mode);
                            let border = mix(#d1d5db, #475569, self.dark_mode);
                            sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                            sdf.fill(mix(bg, hover_bg, self.hover));
                            sdf.stroke(border, 1.0);
                            return sdf.result;
                        }
                    }

                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#374151, #e2e8f0, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    }

                    text: "R"
                }
            }
        }

        // Active downloads section
        downloads_section = <View> {
            width: Fill, height: Fit
            flow: Down
            padding: {left: 20, right: 20, bottom: 12}
            visible: false

            downloads_header = <Label> {
                text: "Active Downloads"
                margin: {bottom: 8}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                }
            }

            downloads_list = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8
            }
        }

        // Results info
        results_info = <View> {
            width: Fill, height: Fit
            padding: {left: 20, right: 20, bottom: 12}

            results_label = <Label> {
                text: "Featured Models"
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                }
            }
        }

        // Model list
        models_scroll = <View> {
            width: Fill, height: Fill
            flow: Down
            padding: {left: 20, right: 20}

            models_list = <PortalList> {
                width: Fill, height: Fill
                drag_scrolling: true

                ModelCardItem = <ModelCard> {}
            }
        }

        // Empty state / loading / error
        empty_state = <View> {
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}
            visible: false

            empty_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                }
            }
        }
    }
}
