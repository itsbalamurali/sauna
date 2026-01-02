use dxf::entities::*;
use dxf::{Drawing, Point};

use crate::constants::*;
use crate::helpers::*;
use crate::layers::*;

/// Draw all floor plan elements
pub fn draw_floor_plan(drawing: &mut Drawing) {
    draw_walls(drawing);
    draw_doors(drawing);
    draw_windows(drawing);
    draw_deck(drawing);
    draw_hot_tub(drawing);
    draw_fixtures(drawing);
    draw_room_labels(drawing);
    draw_dimensions(drawing);
    draw_annotations(drawing);
}

fn draw_walls(drawing: &mut Drawing) {
    let deck_y = DECK_DEPTH;
    let building_top = deck_y + BUILDING_DEPTH;

    // =========================================================================
    // EXTERIOR WALLS
    // =========================================================================

    // Bottom wall (south) - full length of enclosed area
    let south_wall = polyline_from_points(vec![
        Point::new(0.0, deck_y, 0.0),
        Point::new(ENCLOSED_WIDTH, deck_y, 0.0),
        Point::new(ENCLOSED_WIDTH, deck_y + WALL_THICKNESS, 0.0),
        Point::new(0.0, deck_y + WALL_THICKNESS, 0.0),
        Point::new(0.0, deck_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(south_wall));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // Top wall (north) - full length of enclosed area
    let north_wall = polyline_from_points(vec![
        Point::new(0.0, building_top - WALL_THICKNESS, 0.0),
        Point::new(ENCLOSED_WIDTH, building_top - WALL_THICKNESS, 0.0),
        Point::new(ENCLOSED_WIDTH, building_top, 0.0),
        Point::new(0.0, building_top, 0.0),
        Point::new(0.0, building_top - WALL_THICKNESS, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(north_wall));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // Left wall (west) - sauna exterior
    let west_wall = polyline_from_points(vec![
        Point::new(0.0, deck_y, 0.0),
        Point::new(WALL_THICKNESS, deck_y, 0.0),
        Point::new(WALL_THICKNESS, building_top, 0.0),
        Point::new(0.0, building_top, 0.0),
        Point::new(0.0, deck_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(west_wall));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // Right wall of enclosed area (east side of seating)
    let east_wall = polyline_from_points(vec![
        Point::new(ENCLOSED_WIDTH - WALL_THICKNESS, deck_y, 0.0),
        Point::new(ENCLOSED_WIDTH, deck_y, 0.0),
        Point::new(ENCLOSED_WIDTH, building_top, 0.0),
        Point::new(ENCLOSED_WIDTH - WALL_THICKNESS, building_top, 0.0),
        Point::new(ENCLOSED_WIDTH - WALL_THICKNESS, deck_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(east_wall));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // =========================================================================
    // INTERIOR PARTITION WALLS
    // =========================================================================

    // Wall between sauna and wash (with door opening)
    let sauna_wash_wall_bottom = polyline_from_points(vec![
        Point::new(SAUNA_WIDTH - WALL_THICKNESS, deck_y + WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH, deck_y + WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH, deck_y + WALL_THICKNESS + DOOR_MARGIN, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS, deck_y + WALL_THICKNESS + DOOR_MARGIN, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS, deck_y + WALL_THICKNESS, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(sauna_wash_wall_bottom));
    entity.common.layer = LAYER_WALLS_INNER.to_string();
    drawing.add_entity(entity);

    // Wall segment after door opening
    let sauna_wash_wall_top = polyline_from_points(vec![
        Point::new(SAUNA_WIDTH - WALL_THICKNESS, deck_y + WALL_THICKNESS + DOOR_MARGIN + DOOR_WIDTH, 0.0),
        Point::new(SAUNA_WIDTH, deck_y + WALL_THICKNESS + DOOR_MARGIN + DOOR_WIDTH, 0.0),
        Point::new(SAUNA_WIDTH, building_top - WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS, building_top - WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS, deck_y + WALL_THICKNESS + DOOR_MARGIN + DOOR_WIDTH, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(sauna_wash_wall_top));
    entity.common.layer = LAYER_WALLS_INNER.to_string();
    drawing.add_entity(entity);

    // Wall between wash and seating (with door opening)
    let wash_seat_wall_bottom = polyline_from_points(vec![
        Point::new(SAUNA_WIDTH + WASH_WIDTH - WALL_THICKNESS, deck_y + WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH, deck_y + WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH, building_top - WALL_THICKNESS - DOOR_WIDTH - DOOR_MARGIN, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH - WALL_THICKNESS, building_top - WALL_THICKNESS - DOOR_WIDTH - DOOR_MARGIN, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH - WALL_THICKNESS, deck_y + WALL_THICKNESS, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(wash_seat_wall_bottom));
    entity.common.layer = LAYER_WALLS_INNER.to_string();
    drawing.add_entity(entity);

    // Wall segment after door
    let wash_seat_wall_top = polyline_from_points(vec![
        Point::new(SAUNA_WIDTH + WASH_WIDTH - WALL_THICKNESS, building_top - WALL_THICKNESS - DOOR_MARGIN, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH, building_top - WALL_THICKNESS - DOOR_MARGIN, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH, building_top - WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH - WALL_THICKNESS, building_top - WALL_THICKNESS, 0.0),
        Point::new(SAUNA_WIDTH + WASH_WIDTH - WALL_THICKNESS, building_top - WALL_THICKNESS - DOOR_MARGIN, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(wash_seat_wall_top));
    entity.common.layer = LAYER_WALLS_INNER.to_string();
    drawing.add_entity(entity);

    // =========================================================================
    // COVERED DECK AREA BOUNDARY
    // =========================================================================

    let outdoor_boundary = polyline_from_points(vec![
        Point::new(ENCLOSED_WIDTH, deck_y, 0.0),
        Point::new(TOTAL_WIDTH, deck_y, 0.0),
        Point::new(TOTAL_WIDTH, building_top, 0.0),
        Point::new(ENCLOSED_WIDTH, building_top, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(outdoor_boundary));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // Support posts for covered deck
    let posts = [
        (TOTAL_WIDTH - POST_INSET, deck_y + POST_INSET),
        (TOTAL_WIDTH - POST_INSET, building_top - POST_INSET - POST_SIZE),
    ];

    for (px, py) in posts {
        let post = polyline_from_points(vec![
            Point::new(px, py, 0.0),
            Point::new(px + POST_SIZE, py, 0.0),
            Point::new(px + POST_SIZE, py + POST_SIZE, 0.0),
            Point::new(px, py + POST_SIZE, 0.0),
            Point::new(px, py, 0.0),
        ]);
        let mut entity = Entity::new(EntityType::LwPolyline(post));
        entity.common.layer = LAYER_WALLS.to_string();
        drawing.add_entity(entity);
    }
}

fn draw_doors(drawing: &mut Drawing) {
    let deck_y = DECK_DEPTH;
    let building_top = deck_y + BUILDING_DEPTH;

    // Door from wash to deck (south wall) - swings into wash room
    let wash_door_x = SAUNA_WIDTH + 0.20;
    draw_door(drawing, wash_door_x, deck_y + WALL_THICKNESS, DOOR_WIDTH, 90.0, false);

    // Door from sauna to wash (internal) - swings into wash
    let sauna_door_y = deck_y + WALL_THICKNESS + DOOR_MARGIN;
    draw_door(drawing, SAUNA_WIDTH - WALL_THICKNESS, sauna_door_y, DOOR_WIDTH, 0.0, false);

    // Door from wash to seating (internal) - swings into seating
    let wash_seat_door_y = building_top - WALL_THICKNESS - DOOR_MARGIN - DOOR_WIDTH;
    draw_door(drawing, SAUNA_WIDTH + WASH_WIDTH, wash_seat_door_y + DOOR_WIDTH, DOOR_WIDTH, 180.0, false);
}

fn draw_windows(drawing: &mut Drawing) {
    let deck_y = DECK_DEPTH;
    let building_top = deck_y + BUILDING_DEPTH;

    // Window in sauna - west wall (looking out)
    draw_window(drawing, 0.0, deck_y + BUILDING_DEPTH / 2.0 - WINDOW_WIDTH / 2.0, WINDOW_WIDTH, WALL_THICKNESS, false);

    // Window in sauna - north wall
    draw_window(drawing, SAUNA_WIDTH / 2.0 - WINDOW_WIDTH / 2.0, building_top - WALL_THICKNESS, WINDOW_WIDTH, WALL_THICKNESS, true);

    // Window in seating area - north wall
    let seating_x = SAUNA_WIDTH + WASH_WIDTH;
    draw_window(drawing, seating_x + SEATING_WIDTH / 2.0 - WINDOW_WIDTH / 2.0, building_top - WALL_THICKNESS, WINDOW_WIDTH, WALL_THICKNESS, true);
}

fn draw_deck(drawing: &mut Drawing) {
    // Deck outline
    let deck_outline = polyline_from_points(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(TOTAL_WIDTH, 0.0, 0.0),
        Point::new(TOTAL_WIDTH, DECK_DEPTH, 0.0),
        Point::new(0.0, DECK_DEPTH, 0.0),
        Point::new(0.0, 0.0, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(deck_outline));
    entity.common.layer = LAYER_DECK.to_string();
    drawing.add_entity(entity);

    // Deck planking pattern
    draw_deck_pattern(drawing, 0.0, 0.0, TOTAL_WIDTH, DECK_DEPTH);
}

fn draw_hot_tub(drawing: &mut Drawing) {
    let hot_tub_x = TOTAL_WIDTH - HOT_TUB_OFFSET_X;
    let hot_tub_y = -HOT_TUB_RADIUS + HOT_TUB_OFFSET_Y;

    // Hot tub enclosure on deck
    let enclosure = polyline_from_points(vec![
        Point::new(hot_tub_x - HOT_TUB_RADIUS - 0.10, DECK_DEPTH, 0.0),
        Point::new(hot_tub_x + HOT_TUB_RADIUS + 0.10, DECK_DEPTH, 0.0),
        Point::new(hot_tub_x + HOT_TUB_RADIUS + 0.10, 0.0, 0.0),
        Point::new(hot_tub_x - HOT_TUB_RADIUS - 0.10, 0.0, 0.0),
        Point::new(hot_tub_x - HOT_TUB_RADIUS - 0.10, DECK_DEPTH, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(enclosure));
    entity.common.layer = LAYER_HOT_TUB.to_string();
    drawing.add_entity(entity);

    // Hot tub outer circle
    let hot_tub_outer = Circle {
        center: Point::new(hot_tub_x, hot_tub_y, 0.0),
        radius: HOT_TUB_RADIUS,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(hot_tub_outer));
    entity.common.layer = LAYER_HOT_TUB.to_string();
    drawing.add_entity(entity);

    // Hot tub inner circle (water line)
    let hot_tub_inner = Circle {
        center: Point::new(hot_tub_x, hot_tub_y, 0.0),
        radius: HOT_TUB_RADIUS - 0.15,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(hot_tub_inner));
    entity.common.layer = LAYER_HOT_TUB.to_string();
    drawing.add_entity(entity);
}

fn draw_fixtures(drawing: &mut Drawing) {
    let deck_y = DECK_DEPTH;

    // =========================================================================
    // SAUNA FIXTURES
    // =========================================================================

    // Sauna heater (circle)
    let heater_x = WALL_THICKNESS + HEATER_DIAMETER / 2.0 + 0.20;
    let heater_y = deck_y + WALL_THICKNESS + HEATER_DIAMETER / 2.0 + 0.20;
    let heater = Circle {
        center: Point::new(heater_x, heater_y, 0.0),
        radius: HEATER_DIAMETER / 2.0,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(heater));
    entity.common.layer = LAYER_FIXTURES.to_string();
    drawing.add_entity(entity);

    // Lower bench (L-shaped along walls)
    let bench_lower = polyline_from_points(vec![
        Point::new(WALL_THICKNESS + 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS - 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS - 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - 0.10, 0.0),
        Point::new(WALL_THICKNESS + 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - 0.10, 0.0),
        Point::new(WALL_THICKNESS + 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(bench_lower));
    entity.common.layer = LAYER_FIXTURES.to_string();
    drawing.add_entity(entity);

    // Upper bench
    let bench_upper = polyline_from_points(vec![
        Point::new(WALL_THICKNESS + 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH * 2.0 - 0.10, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS - 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH * 2.0 - 0.10, 0.0),
        Point::new(SAUNA_WIDTH - WALL_THICKNESS - 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH - 0.20, 0.0),
        Point::new(WALL_THICKNESS + 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH - 0.20, 0.0),
        Point::new(WALL_THICKNESS + 0.10, deck_y + BUILDING_DEPTH - WALL_THICKNESS - BENCH_DEPTH * 2.0 - 0.10, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(bench_upper));
    entity.common.layer = LAYER_FIXTURES.to_string();
    drawing.add_entity(entity);
}

fn draw_room_labels(drawing: &mut Drawing) {
    let deck_y = DECK_DEPTH;
    let building_top = deck_y + BUILDING_DEPTH;

    // Sauna label
    draw_room_label(
        drawing,
        SAUNA_WIDTH / 2.0,
        (deck_y + building_top) / 2.0,
        "SAUNA",
        sauna_area(),
    );

    // Wash label
    draw_room_label(
        drawing,
        SAUNA_WIDTH + WASH_WIDTH / 2.0,
        (deck_y + building_top) / 2.0,
        "WASH",
        wash_area(),
    );

    // Seating label
    draw_room_label(
        drawing,
        SAUNA_WIDTH + WASH_WIDTH + SEATING_WIDTH / 2.0,
        (deck_y + building_top) / 2.0,
        "CHANGING",
        seating_area(),
    );

    // Deck label (covered portion)
    draw_room_label(
        drawing,
        ENCLOSED_WIDTH + OUTDOOR_SEATING_WIDTH / 2.0,
        (deck_y + building_top) / 2.0,
        "DECK",
        outdoor_area(),
    );

    // Deck label (open terrace strip)
    draw_room_label(
        drawing,
        TOTAL_WIDTH / 3.0,
        DECK_DEPTH / 2.0,
        "TERRACE",
        deck_area(),
    );

    // Hot tub label
    let hot_tub_x = TOTAL_WIDTH - HOT_TUB_OFFSET_X;
    let hot_tub_y = -HOT_TUB_RADIUS + HOT_TUB_OFFSET_Y;
    let ht_label = Text {
        location: Point::new(hot_tub_x - 0.30, hot_tub_y, 0.0),
        text_height: DIM_TEXT_HEIGHT,
        value: format!("HOT TUB\n{}m dia.", HOT_TUB_DIAMETER),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(ht_label));
    entity.common.layer = LAYER_TEXT.to_string();
    drawing.add_entity(entity);
}

fn draw_dimensions(drawing: &mut Drawing) {
    let deck_y = DECK_DEPTH;
    let building_top = deck_y + BUILDING_DEPTH;

    // =========================================================================
    // HORIZONTAL DIMENSIONS (along bottom)
    // =========================================================================

    // Overall width
    draw_dim_horizontal(drawing, 0.0, TOTAL_WIDTH, -HOT_TUB_RADIUS - 0.50, -DIM_OFFSET * 2.0, "");

    // Room widths (chain dimension)
    // Sauna width
    draw_dim_horizontal(drawing, 0.0, SAUNA_WIDTH, deck_y, -DIM_OFFSET, "");

    // Wash width
    draw_dim_horizontal(drawing, SAUNA_WIDTH, SAUNA_WIDTH + WASH_WIDTH, deck_y, -DIM_OFFSET, "");

    // Seating width
    draw_dim_horizontal(drawing, SAUNA_WIDTH + WASH_WIDTH, SAUNA_WIDTH + WASH_WIDTH + SEATING_WIDTH, deck_y, -DIM_OFFSET, "");

    // Covered deck width
    draw_dim_horizontal(drawing, ENCLOSED_WIDTH, TOTAL_WIDTH, deck_y, -DIM_OFFSET, "");

    // =========================================================================
    // VERTICAL DIMENSIONS (along left side)
    // =========================================================================

    // Overall depth
    draw_dim_vertical(drawing, 0.0, 0.0, building_top, -DIM_OFFSET * 2.0, "");

    // Deck depth
    draw_dim_vertical(drawing, 0.0, 0.0, DECK_DEPTH, -DIM_OFFSET, "");

    // Building depth
    draw_dim_vertical(drawing, 0.0, DECK_DEPTH, building_top, -DIM_OFFSET, "");

    // =========================================================================
    // WALL THICKNESS CALLOUT
    // =========================================================================

    let wall_note = Text {
        location: Point::new(-DIM_OFFSET * 3.0, building_top / 2.0, 0.0),
        text_height: DIM_TEXT_HEIGHT * 0.8,
        value: format!("WALLS: {}mm", (WALL_THICKNESS * 1000.0) as i32),
        rotation: 90.0,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(wall_note));
    entity.common.layer = LAYER_ANNO.to_string();
    drawing.add_entity(entity);
}

fn draw_annotations(drawing: &mut Drawing) {
    // North arrow
    draw_north_arrow(drawing, -1.0, DECK_DEPTH + BUILDING_DEPTH + 0.50, 0.50);

    // Title block
    draw_title_block(drawing, TOTAL_WIDTH - 4.0, -HOT_TUB_RADIUS - 2.0, 4.0, 1.5);

    // Drawing border
    let border = polyline_from_points(vec![
        Point::new(-2.0, -HOT_TUB_RADIUS - 2.5, 0.0),
        Point::new(TOTAL_WIDTH + 1.0, -HOT_TUB_RADIUS - 2.5, 0.0),
        Point::new(TOTAL_WIDTH + 1.0, DECK_DEPTH + BUILDING_DEPTH + 1.5, 0.0),
        Point::new(-2.0, DECK_DEPTH + BUILDING_DEPTH + 1.5, 0.0),
        Point::new(-2.0, -HOT_TUB_RADIUS - 2.5, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(border));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);
}
