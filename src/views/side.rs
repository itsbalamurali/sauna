use dxf::entities::*;
use dxf::{Drawing, Point};

use crate::constants::*;
use crate::helpers::*;
use crate::layers::*;

pub fn create_side_elevation(drawing: &mut Drawing, offset_x: f64, offset_y: f64) {
    // Ground line
    let ground = Line {
        p1: Point::new(offset_x - 0.50, offset_y, 0.0),
        p2: Point::new(offset_x + BUILDING_DEPTH + DECK_DEPTH + 0.50, offset_y, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ground));
    entity.common.layer = LAYER_ELEV.to_string();
    drawing.add_entity(entity);

    // Deck (extends in front of building)
    let deck = polyline_from_points(vec![
        Point::new(offset_x - DECK_DEPTH, offset_y, 0.0),
        Point::new(offset_x - DECK_DEPTH, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH, offset_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(deck));
    entity.common.layer = LAYER_DECK.to_string();
    drawing.add_entity(entity);

    // Wall (full depth)
    let wall = polyline_from_points(vec![
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(wall));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // Flat roof (with overhang)
    let roof = polyline_from_points(vec![
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.0),
        Point::new(offset_x + BUILDING_DEPTH + ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.0),
        Point::new(offset_x + BUILDING_DEPTH + ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(roof));
    entity.common.layer = LAYER_ROOF.to_string();
    drawing.add_entity(entity);

    // Window (on north wall - visible from side)
    let window = polyline_from_points(vec![
        Point::new(offset_x + BUILDING_DEPTH - WALL_THICKNESS - 0.05, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH - WALL_THICKNESS - 0.05, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT + WINDOW_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT + WINDOW_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT, 0.0),
        Point::new(offset_x + BUILDING_DEPTH - WALL_THICKNESS - 0.05, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(window));
    entity.common.layer = LAYER_WINDOWS.to_string();
    drawing.add_entity(entity);

    // Dimensions
    draw_dim_horizontal(drawing, offset_x, offset_x + BUILDING_DEPTH, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.30, "");
    draw_dim_vertical(drawing, offset_x + BUILDING_DEPTH, offset_y, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.30, "");

    // Label
    let label = Text {
        location: Point::new(
            offset_x + 0.20,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS + 0.50,
            0.0,
        ),
        text_height: LABEL_TEXT_HEIGHT,
        value: "SIDE ELEVATION".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(label));
    entity.common.layer = LAYER_TEXT.to_string();
    drawing.add_entity(entity);
}
