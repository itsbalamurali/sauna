use dxf::tables::Layer;
use dxf::{Color, Drawing};

// Layer names following AIA naming convention
pub const LAYER_WALLS: &str = "A-WALL";
pub const LAYER_WALLS_INNER: &str = "A-WALL-INTR";
pub const LAYER_DOORS: &str = "A-DOOR";
pub const LAYER_WINDOWS: &str = "A-GLAZ";
pub const LAYER_DIMENSIONS: &str = "A-DIMS";
pub const LAYER_TEXT: &str = "A-TEXT";
pub const LAYER_ANNO: &str = "A-ANNO";
pub const LAYER_DECK: &str = "A-DECK";
pub const LAYER_DECK_PATTERN: &str = "A-DECK-PATT";
pub const LAYER_HOT_TUB: &str = "A-PLMB-FIXT";
pub const LAYER_FIXTURES: &str = "A-FIXT";
pub const LAYER_TITLE: &str = "A-ANNO-TTLB";
pub const LAYER_ELEV: &str = "A-ELEV";
pub const LAYER_SECTION: &str = "A-SECT";
pub const LAYER_ROOF: &str = "A-ROOF";

fn create_layer(name: &str, color_index: u8) -> Layer {
    Layer {
        name: name.to_string(),
        color: Color::from_index(color_index),
        ..Default::default()
    }
}

pub fn setup_layers(drawing: &mut Drawing) {
    // Walls - main structural walls (White)
    drawing.add_layer(create_layer(LAYER_WALLS, 7));

    // Inner walls (Gray)
    drawing.add_layer(create_layer(LAYER_WALLS_INNER, 8));

    // Doors (Green)
    drawing.add_layer(create_layer(LAYER_DOORS, 3));

    // Windows (Cyan)
    drawing.add_layer(create_layer(LAYER_WINDOWS, 4));

    // Dimensions (Yellow)
    drawing.add_layer(create_layer(LAYER_DIMENSIONS, 2));

    // Text labels (White)
    drawing.add_layer(create_layer(LAYER_TEXT, 7));

    // Annotations (White)
    drawing.add_layer(create_layer(LAYER_ANNO, 7));

    // Deck structure (Brown)
    drawing.add_layer(create_layer(LAYER_DECK, 30));

    // Deck pattern/planking (Gray)
    drawing.add_layer(create_layer(LAYER_DECK_PATTERN, 8));

    // Hot tub / plumbing fixtures (Cyan)
    drawing.add_layer(create_layer(LAYER_HOT_TUB, 4));

    // Fixtures - heater, benches (Red)
    drawing.add_layer(create_layer(LAYER_FIXTURES, 1));

    // Title block (White)
    drawing.add_layer(create_layer(LAYER_TITLE, 7));

    // Elevations (White)
    drawing.add_layer(create_layer(LAYER_ELEV, 7));

    // Sections (White)
    drawing.add_layer(create_layer(LAYER_SECTION, 7));

    // Roof (Red)
    drawing.add_layer(create_layer(LAYER_ROOF, 1));
}
