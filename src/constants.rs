use egui::Color32;

pub(crate) const APP_ID: &str = "net.donnybeelo.chromium-profile-picker";
pub(crate) const APP_NAME: &str = "chromium-profile-picker";
pub(crate) const WINDOW_BG: Color32 = Color32::TRANSPARENT;
pub(crate) const PANEL_BG: Color32 = Color32::from_rgb(0x25, 0x25, 0x25);
pub(crate) const CARD_BG_HOVER: Color32 = Color32::from_rgb(0x2b, 0x2b, 0x2b);
pub(crate) const CARD_BG_CLICKED: Color32 = Color32::from_rgb(0x30, 0x30, 0x30);
pub(crate) const BORDER: Color32 = Color32::from_rgb(0x40, 0x40, 0x40);
pub(crate) const TEXT: Color32 = Color32::from_rgb(0xee, 0xee, 0xee);
pub(crate) const MUTED_TEXT: Color32 = Color32::from_rgb(0xf2, 0xf2, 0xf2);
pub(crate) const BADGE_BG: Color32 = Color32::from_rgb(0x4a, 0x4a, 0x4a);
pub(crate) const BADGE_BORDER: Color32 = Color32::from_rgb(0x5a, 0x5a, 0x5a);
pub(crate) const CLOSE_BG: Color32 = Color32::from_rgb(0x33, 0x33, 0x33);
pub(crate) const CLOSE_BG_HOVER: Color32 = Color32::from_rgb(0x44, 0x44, 0x44);
pub(crate) const AVATAR_BG: Color32 = Color32::from_rgb(0x30, 0x30, 0x30);
pub(crate) const CARD_WIDTH: f32 = 170.0;
pub(crate) const CARD_HEIGHT: f32 = 200.0;
pub(crate) const CARD_MARGIN_X: f32 = 15.0;
pub(crate) const PANEL_INNER_PADDING: f32 = 20.0;
pub(crate) const HEADER_HEIGHT: f32 = 40.0;
pub(crate) const URL_ROW_HEIGHT: f32 = 38.0;
pub(crate) const CONTENT_GAP: f32 = 4.0;
pub(crate) const PROFILE_ROW_GAP: f32 = 16.0;

pub(crate) struct Palette {
    pub(crate) panel_bg: Color32,
    pub(crate) panel_border: Color32,
    pub(crate) url_bg: Color32,
    pub(crate) card_bg_hover: Color32,
    pub(crate) card_bg_clicked: Color32,
    pub(crate) border: Color32,
    pub(crate) text: Color32,
    pub(crate) muted_text: Color32,
    pub(crate) badge_bg: Color32,
    pub(crate) badge_border: Color32,
    pub(crate) badge_text: Color32,
    pub(crate) close_bg: Color32,
    pub(crate) close_bg_hover: Color32,
    pub(crate) avatar_bg: Color32,
    pub(crate) error_text: Color32,
}

pub(crate) const DARK: Palette = Palette {
    panel_bg: PANEL_BG,
    panel_border: Color32::from_rgb(0x2f, 0x2f, 0x2f),
    url_bg: Color32::from_rgb(0x30, 0x30, 0x30),
    card_bg_hover: CARD_BG_HOVER,
    card_bg_clicked: CARD_BG_CLICKED,
    border: BORDER,
    text: TEXT,
    muted_text: MUTED_TEXT,
    badge_bg: BADGE_BG,
    badge_border: BADGE_BORDER,
    badge_text: Color32::WHITE,
    close_bg: CLOSE_BG,
    close_bg_hover: CLOSE_BG_HOVER,
    avatar_bg: AVATAR_BG,
    error_text: Color32::from_rgb(0xff, 0x95, 0x95),
};

pub(crate) const LIGHT: Palette = Palette {
    panel_bg: Color32::from_rgb(0xf5, 0xf5, 0xf5),
    panel_border: Color32::from_rgb(0xd6, 0xd6, 0xd6),
    url_bg: Color32::from_rgb(0xe8, 0xe8, 0xe8),
    card_bg_hover: Color32::from_rgb(0xeb, 0xeb, 0xeb),
    card_bg_clicked: Color32::from_rgb(0xe2, 0xe2, 0xe2),
    border: Color32::from_rgb(0xd0, 0xd0, 0xd0),
    text: Color32::from_rgb(0x1f, 0x1f, 0x1f),
    muted_text: Color32::from_rgb(0x1f, 0x1f, 0x1f),
    badge_bg: Color32::from_rgb(0xdc, 0xdc, 0xdc),
    badge_border: Color32::from_rgb(0xc8, 0xc8, 0xc8),
    badge_text: Color32::from_rgb(0x1f, 0x1f, 0x1f),
    close_bg: Color32::from_rgb(0xe6, 0xe6, 0xe6),
    close_bg_hover: Color32::from_rgb(0xd6, 0xd6, 0xd6),
    avatar_bg: Color32::from_rgb(0xe2, 0xe2, 0xe2),
    error_text: Color32::from_rgb(0xc6, 0x28, 0x28),
};

pub(crate) fn palette(ctx: &egui::Context) -> &'static Palette {
    match ctx.theme() {
        egui::Theme::Dark => &DARK,
        egui::Theme::Light => &LIGHT,
    }
}
