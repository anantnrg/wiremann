use gpui::Rgba;

pub struct Theme {
    /* ===== Backgrounds ===== */
    pub bg_app: Rgba,      // App root
    pub bg_titlebar: Rgba, // Titlebar
    pub bg_panel: Rgba,    // Sidebars, queue, playlist panels

    /* ===== Text ===== */
    pub text_primary: Rgba,   // Main text
    pub text_secondary: Rgba, // Artist names, counts
    pub text_muted: Rgba,     // Timestamps, icons
    pub text_accent: Rgba,    // Active track title, active tab text

    /* ===== Accent ===== */
    pub accent: Rgba,         // Primary accent (buttons, progress, underline)
    pub accent_bg: Rgba,      // Selected row / playlist background
    pub accent_bg_soft: Rgba, // Floating queue button background
    pub accent_border: Rgba,  // Active item border

    /* ===== UI Chrome ===== */
    pub border: Rgba,          // Panel dividers
    pub hover_bg: Rgba,        // Hover row background
    pub slider_inactive: Rgba, // Slider track remainder
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            /* ===== Backgrounds ===== */
            // #0A070F
            bg_app: Rgba::new(10.0 / 255.0, 7.0 / 255.0, 15.0 / 255.0, 1.0),

            // #0A0515
            bg_titlebar: Rgba::new(10.0 / 255.0, 5.0 / 255.0, 21.0 / 255.0, 1.0),

            // #0F0C17
            bg_panel: Rgba::new(15.0 / 255.0, 12.0 / 255.0, 23.0 / 255.0, 1.0),

            /* ===== Text ===== */
            // #FFFFFF
            text_primary: Rgba::new(1.0, 1.0, 1.0, 1.0),

            // #6B6B7B
            text_secondary: Rgba::new(107.0 / 255.0, 107.0 / 255.0, 123.0 / 255.0, 1.0),

            // #5A5A6B
            text_muted: Rgba::new(90.0 / 255.0, 90.0 / 255.0, 107.0 / 255.0, 1.0),

            // #8B7BF7
            text_accent: Rgba::new(139.0 / 255.0, 123.0 / 255.0, 247.0 / 255.0, 1.0),

            /* ===== Accent ===== */
            // #8B7BF7
            accent: Rgba::new(139.0 / 255.0, 123.0 / 255.0, 247.0 / 255.0, 1.0),

            // rgba(139,123,247,0.10)
            accent_bg: Rgba::new(139.0 / 255.0, 123.0 / 255.0, 247.0 / 255.0, 0.10),

            // rgba(139,123,247,0.15)
            accent_bg_soft: Rgba::new(139.0 / 255.0, 123.0 / 255.0, 247.0 / 255.0, 0.15),

            // rgba(139,123,247,0.30)
            accent_border: Rgba::new(139.0 / 255.0, 123.0 / 255.0, 247.0 / 255.0, 0.30),

            /* ===== UI Chrome ===== */
            // rgba(255,255,255,0.05)
            border: Rgba::new(1.0, 1.0, 1.0, 0.05),

            // rgba(255,255,255,0.05)
            hover_bg: Rgba::new(1.0, 1.0, 1.0, 0.05),

            // rgba(255,255,255,0.08)
            slider_inactive: Rgba::new(1.0, 1.0, 1.0, 0.08),
        }
    }
}

impl gpui::Global for Theme {}
