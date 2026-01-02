use dxf::entities::*;
use dxf::{Drawing, Point};

use crate::constants::*;
use crate::helpers::*;
use crate::layers::*;

pub fn create_top_view(drawing: &mut Drawing, offset_x: f64, offset_y: f64) {
    // Flat roof outline (with overhang)
    let roof_outline = polyline_from_points(vec![
        Point::new(offset_x - ROOF_OVERHANG, offset_y - ROOF_OVERHANG, 0.0),
        Point::new(offset_x + TOTAL_WIDTH + ROOF_OVERHANG, offset_y - ROOF_OVERHANG, 0.0),
        Point::new(offset_x + TOTAL_WIDTH + ROOF_OVERHANG, offset_y + BUILDING_DEPTH + ROOF_OVERHANG, 0.0),
        Point::new(offset_x - ROOF_OVERHANG, offset_y + BUILDING_DEPTH + ROOF_OVERHANG, 0.0),
        Point::new(offset_x - ROOF_OVERHANG, offset_y - ROOF_OVERHANG, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(roof_outline));
    entity.common.layer = LAYER_ROOF.to_string();
    drawing.add_entity(entity);

    // Building outline (dashed - walls below roof)
    let building_outline = polyline_from_points(vec![
        Point::new(offset_x, offset_y, 0.0),
        Point::new(offset_x + TOTAL_WIDTH, offset_y, 0.0),
        Point::new(offset_x + TOTAL_WIDTH, offset_y + BUILDING_DEPTH, 0.0),
        Point::new(offset_x, offset_y + BUILDING_DEPTH, 0.0),
        Point::new(offset_x, offset_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(building_outline));
    entity.common.layer = LAYER_WALLS.to_string();
    drawing.add_entity(entity);

    // Enclosed section boundary
    let enclosed = polyline_from_points(vec![
        Point::new(offset_x, offset_y, 0.0),
        Point::new(offset_x + ENCLOSED_WIDTH, offset_y, 0.0),
        Point::new(offset_x + ENCLOSED_WIDTH, offset_y + BUILDING_DEPTH, 0.0),
        Point::new(offset_x, offset_y + BUILDING_DEPTH, 0.0),
        Point::new(offset_x, offset_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(enclosed));
    entity.common.layer = LAYER_WALLS_INNER.to_string();
    drawing.add_entity(entity);

    // Vent/chimney for sauna heater
    let vent = Circle {
        center: Point::new(offset_x + HEATER_DIAMETER, offset_y + BUILDING_DEPTH / 2.0, 0.0),
        radius: 0.15,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(vent));
    entity.common.layer = LAYER_FIXTURES.to_string();
    drawing.add_entity(entity);

    // Dimensions
    draw_dim_horizontal(drawing, offset_x - ROOF_OVERHANG, offset_x + TOTAL_WIDTH + ROOF_OVERHANG, offset_y + BUILDING_DEPTH + ROOF_OVERHANG, 0.30, "");
    draw_dim_vertical(drawing, offset_x + TOTAL_WIDTH + ROOF_OVERHANG, offset_y - ROOF_OVERHANG, offset_y + BUILDING_DEPTH + ROOF_OVERHANG, 0.30, "");

    // Label
    let label = Text {
        location: Point::new(offset_x + 1.00, offset_y + BUILDING_DEPTH + ROOF_OVERHANG + 0.50, 0.0),
        text_height: LABEL_TEXT_HEIGHT,
        value: "ROOF PLAN".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(label));
    entity.common.layer = LAYER_TEXT.to_string();
    drawing.add_entity(entity);
}
