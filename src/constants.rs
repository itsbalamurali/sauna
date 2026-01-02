// ============================================================================
// SAUNA BUILDING - DIMENSIONAL CONSTANTS
// All dimensions in METERS
// ============================================================================

// Room dimensions (exterior widths including walls)
pub const SAUNA_WIDTH: f64 = 2.26;         // Sauna room width
pub const WASH_WIDTH: f64 = 1.20;          // Wash/shower room width
pub const SEATING_WIDTH: f64 = 1.50;       // Changing/seating area width
pub const OUTDOOR_SEATING_WIDTH: f64 = 3.65;  // Covered deck area

// Overall building dimensions (computed)
pub const ENCLOSED_WIDTH: f64 = SAUNA_WIDTH + WASH_WIDTH + SEATING_WIDTH;
pub const TOTAL_WIDTH: f64 = ENCLOSED_WIDTH + OUTDOOR_SEATING_WIDTH;
pub const BUILDING_DEPTH: f64 = 3.00;      // Depth of enclosed building section
pub const DECK_DEPTH: f64 = 1.00;          // Depth of deck/terrace strip
pub const WALL_THICKNESS: f64 = 0.15;      // 150mm timber frame walls

// Door dimensions
pub const DOOR_WIDTH: f64 = 0.80;          // Standard door width (800mm)
pub const DOOR_HEIGHT: f64 = 2.10;         // Standard door height (2100mm)
pub const DOOR_MARGIN: f64 = 0.30;         // Wall margin before door opening

// Window dimensions
pub const WINDOW_WIDTH: f64 = 0.60;        // Window width
pub const WINDOW_HEIGHT: f64 = 0.80;       // Window height
pub const WINDOW_SILL_HEIGHT: f64 = 1.00;  // Height from floor to window sill

// Hot tub dimensions
pub const HOT_TUB_DIAMETER: f64 = 2.00;    // Hot tub diameter
pub const HOT_TUB_RADIUS: f64 = HOT_TUB_DIAMETER / 2.0;
pub const HOT_TUB_OFFSET_X: f64 = 1.50;    // Hot tub center offset from right edge
pub const HOT_TUB_OFFSET_Y: f64 = 0.20;    // Hot tub overlap with deck edge

// Elevation dimensions
pub const BUILDING_HEIGHT: f64 = 2.50;     // Floor to ceiling height
pub const DECK_ELEV_HEIGHT: f64 = 0.40;    // Deck height above ground
pub const ROOF_THICKNESS: f64 = 0.20;      // Flat roof slab thickness
pub const ROOF_OVERHANG: f64 = 0.30;       // Roof overhang

// Fixture dimensions
pub const HEATER_DIAMETER: f64 = 0.50;     // Sauna heater diameter
pub const BENCH_DEPTH: f64 = 0.60;         // Sauna bench depth
pub const BENCH_HEIGHT_LOWER: f64 = 0.45;  // Lower bench height
pub const BENCH_HEIGHT_UPPER: f64 = 0.90;  // Upper bench height

// Structural elements
pub const POST_SIZE: f64 = 0.15;           // Support post dimensions (150mm)
pub const POST_INSET: f64 = 0.30;          // Post inset from edges

// Deck pattern
pub const DECK_BOARD_WIDTH: f64 = 0.15;    // 150mm deck boards
pub const DECK_BOARD_GAP: f64 = 0.01;      // 10mm gap between boards

// Drawing annotation settings
pub const DIM_TEXT_HEIGHT: f64 = 0.10;     // Dimension text height
pub const LABEL_TEXT_HEIGHT: f64 = 0.15;   // Room label text height
pub const TITLE_TEXT_HEIGHT: f64 = 0.25;   // Title text height
pub const DIM_OFFSET: f64 = 0.30;          // Offset for dimension lines
pub const DIM_TICK_SIZE: f64 = 0.05;       // Dimension tick mark size

// Scale
pub const DRAWING_SCALE: &str = "1:50";

// Room areas (calculated)
pub fn sauna_area() -> f64 {
    let inner_width = SAUNA_WIDTH - 2.0 * WALL_THICKNESS;
    let inner_depth = BUILDING_DEPTH - 2.0 * WALL_THICKNESS;
    inner_width * inner_depth
}

pub fn wash_area() -> f64 {
    let inner_width = WASH_WIDTH - WALL_THICKNESS; // Shared wall with sauna
    let inner_depth = BUILDING_DEPTH - 2.0 * WALL_THICKNESS;
    inner_width * inner_depth
}

pub fn seating_area() -> f64 {
    let inner_width = SEATING_WIDTH - WALL_THICKNESS;
    let inner_depth = BUILDING_DEPTH - 2.0 * WALL_THICKNESS;
    inner_width * inner_depth
}

pub fn outdoor_area() -> f64 {
    OUTDOOR_SEATING_WIDTH * BUILDING_DEPTH
}

pub fn deck_area() -> f64 {
    TOTAL_WIDTH * DECK_DEPTH
}

pub fn total_floor_area() -> f64 {
    sauna_area() + wash_area() + seating_area() + outdoor_area() + deck_area()
}
