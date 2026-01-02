use dxf::entities::*;
use dxf::{Drawing, Point};

use crate::constants::*;
use crate::helpers::*;
use crate::layers::*;

pub fn create_front_elevation(drawing: &mut Drawing, offset_x: f64, offset_y: f64) {
    // Ground line
    let ground = Line {
        p1: Point::new(offset_x - 0.50, offset_y, 0.0),
        p2: Point::new(offset_x + TOTAL_WIDTH + 0.50, offset_y, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ground));
    entity.common.layer = LAYER_ELEV.to_string();
    drawing.add_entity(entity);

    // Deck platform (full width)
    let deck = polyline_from_points(vec![
        Point::new(offset_x, offset_y, 0.0),
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + TOTAL_WIDTH, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + TOTAL_WIDTH, offset_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(deck));
    entity.common.layer = LAYER_DECK.to_string();
    drawing.add_entity(entity);

    // Enclosed building section
    let building_rect = polyline_from_points(vec![
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x + ENCLOSED_WIDTH, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x + ENCLOSED_WIDTH, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(building_rect));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // Covered deck posts
    let post_x1 = offset_x + ENCLOSED_WIDTH;
    let post_x2 = offset_x + TOTAL_WIDTH - POST_INSET;
    for post_x in [post_x1, post_x2] {
        let post = polyline_from_points(vec![
            Point::new(post_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
            Point::new(post_x, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
            Point::new(post_x + POST_SIZE, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
            Point::new(post_x + POST_SIZE, offset_y + DECK_ELEV_HEIGHT, 0.0),
            Point::new(post_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
        ]);
        let mut entity = Entity::new(EntityType::LwPolyline(post));
        entity.common.layer = LAYER_WALLS.to_string();
        drawing.add_entity(entity);
    }

    // Flat roof (full width with overhang)
    let roof = polyline_from_points(vec![
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.0),
        Point::new(offset_x + TOTAL_WIDTH + ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.0),
        Point::new(offset_x + TOTAL_WIDTH + ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(roof));
    entity.common.layer = LAYER_ROOF.to_string();
    drawing.add_entity(entity);

    // Door (in wash room area)
    let door_x = offset_x + SAUNA_WIDTH + 0.20;
    let door = polyline_from_points(vec![
        Point::new(door_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(door_x, offset_y + DECK_ELEV_HEIGHT + DOOR_HEIGHT, 0.0),
        Point::new(door_x + DOOR_WIDTH, offset_y + DECK_ELEV_HEIGHT + DOOR_HEIGHT, 0.0),
        Point::new(door_x + DOOR_WIDTH, offset_y + DECK_ELEV_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(door));
    entity.common.layer = LAYER_DOORS.to_string();
    drawing.add_entity(entity);

    // Window in sauna (west wall visible from front)
    let window_x = offset_x + SAUNA_WIDTH / 2.0 - WINDOW_WIDTH / 2.0;
    let window = polyline_from_points(vec![
        Point::new(window_x, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT, 0.0),
        Point::new(window_x, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT + WINDOW_HEIGHT, 0.0),
        Point::new(window_x + WINDOW_WIDTH, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT + WINDOW_HEIGHT, 0.0),
        Point::new(window_x + WINDOW_WIDTH, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT, 0.0),
        Point::new(window_x, offset_y + DECK_ELEV_HEIGHT + WINDOW_SILL_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(window));
    entity.common.layer = LAYER_WINDOWS.to_string();
    drawing.add_entity(entity);

    // Dimensions
    draw_dim_horizontal(drawing, offset_x, offset_x + TOTAL_WIDTH, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.30, "");
    draw_dim_vertical(drawing, offset_x + TOTAL_WIDTH, offset_y, offset_y + DECK_ELEV_HEIGHT, 0.30, "");
    draw_dim_vertical(drawing, offset_x + TOTAL_WIDTH, offset_y + DECK_ELEV_HEIGHT, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.50, "");

    // Label
    let label = Text {
        location: Point::new(
            offset_x + 0.50,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS + 0.50,
            0.0,
        ),
        text_height: LABEL_TEXT_HEIGHT,
        value: "FRONT ELEVATION".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(label));
    entity.common.layer = LAYER_TEXT.to_string();
    drawing.add_entity(entity);
}
