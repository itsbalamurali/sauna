use dxf::entities::*;
use dxf::{Drawing, Point};

use crate::constants::*;
use crate::helpers::*;
use crate::layers::*;

pub fn create_section_cut(drawing: &mut Drawing, offset_x: f64, offset_y: f64) {
    // Ground
    let ground = Line {
        p1: Point::new(offset_x - 0.50, offset_y, 0.0),
        p2: Point::new(offset_x + SAUNA_WIDTH + 0.50, offset_y, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ground));
    entity.common.layer = LAYER_SECTION.to_string();
    drawing.add_entity(entity);

    // Deck
    let deck = polyline_from_points(vec![
        Point::new(offset_x - 0.30, offset_y, 0.0),
        Point::new(offset_x - 0.30, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + SAUNA_WIDTH + 0.30, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + SAUNA_WIDTH + 0.30, offset_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(deck));
    entity.common.layer = LAYER_DECK.to_string();
    drawing.add_entity(entity);

    // Left wall section (cut through, shown solid)
    let left_wall = polyline_from_points(vec![
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(
            offset_x + WALL_THICKNESS,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT,
            0.0,
        ),
        Point::new(offset_x + WALL_THICKNESS, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x, offset_y + DECK_ELEV_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(left_wall));
    entity.common.layer = LAYER_SECTION.to_string();
    drawing.add_entity(entity);

    // Right wall section
    let right_wall = polyline_from_points(vec![
        Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(
            offset_x + SAUNA_WIDTH - WALL_THICKNESS,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT,
            0.0,
        ),
        Point::new(
            offset_x + SAUNA_WIDTH,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT,
            0.0,
        ),
        Point::new(offset_x + SAUNA_WIDTH, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS, offset_y + DECK_ELEV_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(right_wall));
    entity.common.layer = LAYER_SECTION.to_string();
    drawing.add_entity(entity);

    // Lower bench
    let bench_lower = polyline_from_points(vec![
        Point::new(offset_x + WALL_THICKNESS + 0.10, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_LOWER, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10 + BENCH_DEPTH, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_LOWER, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10 + BENCH_DEPTH, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_LOWER + 0.05, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_LOWER + 0.05, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_LOWER, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(bench_lower));
    entity.common.layer = LAYER_FIXTURES.to_string();
    drawing.add_entity(entity);

    // Upper bench
    let bench_upper = polyline_from_points(vec![
        Point::new(offset_x + WALL_THICKNESS + 0.10, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_UPPER, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10 + BENCH_DEPTH, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_UPPER, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10 + BENCH_DEPTH, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_UPPER + 0.05, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_UPPER + 0.05, 0.0),
        Point::new(offset_x + WALL_THICKNESS + 0.10, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_UPPER, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(bench_upper));
    entity.common.layer = LAYER_FIXTURES.to_string();
    drawing.add_entity(entity);

    // Heater
    let heater = polyline_from_points(vec![
        Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS - 0.60, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS - 0.60, offset_y + DECK_ELEV_HEIGHT + 0.70, 0.0),
        Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS - 0.30, offset_y + DECK_ELEV_HEIGHT + 0.70, 0.0),
        Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS - 0.30, offset_y + DECK_ELEV_HEIGHT, 0.0),
        Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS - 0.60, offset_y + DECK_ELEV_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(heater));
    entity.common.layer = LAYER_FIXTURES.to_string();
    drawing.add_entity(entity);

    // Flat roof
    let roof = polyline_from_points(vec![
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(
            offset_x - ROOF_OVERHANG,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS,
            0.0,
        ),
        Point::new(
            offset_x + SAUNA_WIDTH + ROOF_OVERHANG,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS,
            0.0,
        ),
        Point::new(offset_x + SAUNA_WIDTH + ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        Point::new(offset_x - ROOF_OVERHANG, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(roof));
    entity.common.layer = LAYER_ROOF.to_string();
    drawing.add_entity(entity);

    // Ceiling line
    let ceiling = Line {
        p1: Point::new(offset_x + WALL_THICKNESS, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        p2: Point::new(offset_x + SAUNA_WIDTH - WALL_THICKNESS, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ceiling));
    entity.common.layer = LAYER_SECTION.to_string();
    drawing.add_entity(entity);

    // Dimensions
    draw_dim_horizontal(drawing, offset_x, offset_x + SAUNA_WIDTH, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS, 0.30, "");
    draw_dim_vertical(drawing, offset_x - 0.50, offset_y + DECK_ELEV_HEIGHT, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_LOWER, -0.20, "");
    draw_dim_vertical(drawing, offset_x - 0.50, offset_y + DECK_ELEV_HEIGHT, offset_y + DECK_ELEV_HEIGHT + BENCH_HEIGHT_UPPER, -0.40, "");
    draw_dim_vertical(drawing, offset_x + SAUNA_WIDTH, offset_y + DECK_ELEV_HEIGHT, offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT, 0.30, "");

    // Label
    let label = Text {
        location: Point::new(
            offset_x + 0.30,
            offset_y + DECK_ELEV_HEIGHT + BUILDING_HEIGHT + ROOF_THICKNESS + 0.50,
            0.0,
        ),
        text_height: LABEL_TEXT_HEIGHT,
        value: "SECTION A-A".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(label));
    entity.common.layer = LAYER_TEXT.to_string();
    drawing.add_entity(entity);
}
