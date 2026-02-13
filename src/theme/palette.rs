use bevy::prelude::*;

/// #ddd369
pub const LABEL_TEXT: Color = COL_LIGHT;

/// #fcfbcc
pub const HEADER_TEXT: Color = COL_PURPLE;

/// #ececec
pub const BUTTON_TEXT: Color = COL_LIGHT;
/// #4666bf
pub const BUTTON_BACKGROUND: Color = COL_PURPLE;
/// #6299d1
pub const BUTTON_FOCUSED_BACKGROUND: Color = COL_PURPLE_DARK;
/// #3d4999
pub const BUTTON_PRESSED_BACKGROUND: Color = COL_PURPLE_DARKER;

// most of the palette:
// https://lospec.com/palette-list/chasm
pub const COL_SNOW: Color = Color::srgb_u8(133, 218, 235);
pub const COL_CYAN: Color = Color::srgb_u8(95, 201, 231);
pub const COL_BLUE_LIGHT: Color = Color::srgb_u8(95, 161, 231);
pub const COL_BLUEISH_PURPLE: Color = Color::srgb_u8(95, 110, 231);
pub const COL_BLUE: Color = Color::srgb_u8(76, 96, 170);
pub const COL_BLUE_DARK: Color = Color::srgb_u8(68, 71, 116);
pub const COL_DARK: Color = Color::srgb_u8(50, 49, 59);
pub const COL_PURPLE_DARKER: Color = Color::srgb_u8(70, 60, 94);
pub const COL_PURPLE_DARK: Color = Color::srgb_u8(93, 71, 118);
pub const COL_PURPLE: Color = Color::srgb_u8(133, 83, 149);
pub const COL_PURPLE_LIGHT: Color = Color::srgb_u8(171, 88, 168);
pub const COL_PINK: Color = Color::srgb_u8(202, 96, 174);
pub const COL_ORANGE: Color = Color::srgb_u8(243, 167, 135);
pub const COL_YELLOW: Color = Color::srgb_u8(245, 218, 167);
pub const COL_GREEN_LIGHT: Color = Color::srgb_u8(141, 216, 148);
pub const COL_GREEN: Color = Color::srgb_u8(93, 193, 144);
pub const COL_GREEN_DARK: Color = Color::srgb_u8(74, 185, 163);
pub const COL_TURQUOISE_DARK: Color = Color::srgb_u8(69, 147, 165);
pub const COL_CYAN_NEON: Color = Color::srgb_u8(94, 253, 247);
pub const COL_PINK_NEON: Color = Color::srgb_u8(255, 93, 204);
pub const COL_YELLOW_NEON: Color = Color::srgb_u8(253, 254, 137);
pub const COL_LIGHT: Color = Color::srgb_u8(255, 255, 255);

// reds from
// https://lospec.com/palette-list/cyberpunk-neons
pub const COL_RED_NEON: Color = Color::srgb_u8(193, 17, 90);
pub const COL_RED: Color = Color::srgb_u8(225, 58, 106);
pub const COL_RED_LIGHT: Color = Color::srgb_u8(228, 106, 135);

// cards
pub const COL_CARD: Color = COL_LIGHT;
pub const COL_CARD_OUTLINE_ACTION_CARD: Color = COL_PURPLE_DARKER;
pub const COL_CARD_OUTLINE_TILE_CARD: Color = COL_CARD;
pub const COL_CARD_BORDER: Color = COL_PURPLE_DARKER;
pub const COL_CARD_BORDER_FOCUS: Color = COL_ORANGE;
pub const COL_CARD_BORDER_DISCARD: Color = COL_RED;
pub const COL_CARD_TEMP_COST_BG: Color = COL_PURPLE_DARKER;
pub const COL_CARD_CENTER_TILE: Color = COL_CARD_BORDER;
pub const COL_CARD_INVALID_TILE: Color = Color::srgb_u8(172, 172, 200);

// tiles
pub const COL_TILE_VALID: Color = COL_GREEN;
pub const COL_TILE_VALID_HOVER: Color = COL_GREEN_DARK;
