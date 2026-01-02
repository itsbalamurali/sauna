use dxf::entities::*;
use dxf::{Drawing, LwPolylineVertex, Point};

use crate::constants::*;
use crate::layers::*;

pub fn polyline_from_points(points: Vec<Point>) -> LwPolyline {
    let mut polyline = LwPolyline::default();
    for point in points {
        polyline.vertices.push(LwPolylineVertex {
            x: point.x,
            y: point.y,
            starting_width: 0.0,
            ending_width: 0.0,
            bulge: 0.0,
            id: 0,
        });
    }
    polyline
}

/// Draw a horizontal dimension line
pub fn draw_dim_horizontal(
    drawing: &mut Drawing,
    x1: f64,
    x2: f64,
    y: f64,
    offset: f64,
    prefix: &str,
) {
    let dim_y = y + offset;
    let length = (x2 - x1).abs();

    // Extension lines
    let ext1 = Line {
        p1: Point::new(x1, y, 0.0),
        p2: Point::new(x1, dim_y + DIM_TICK_SIZE, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ext1));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    let ext2 = Line {
        p1: Point::new(x2, y, 0.0),
        p2: Point::new(x2, dim_y + DIM_TICK_SIZE, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ext2));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    // Dimension line
    let dim_line = Line {
        p1: Point::new(x1, dim_y, 0.0),
        p2: Point::new(x2, dim_y, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(dim_line));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    // Tick marks (architectural style)
    let tick1 = Line {
        p1: Point::new(x1 - DIM_TICK_SIZE, dim_y - DIM_TICK_SIZE, 0.0),
        p2: Point::new(x1 + DIM_TICK_SIZE, dim_y + DIM_TICK_SIZE, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(tick1));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    let tick2 = Line {
        p1: Point::new(x2 - DIM_TICK_SIZE, dim_y - DIM_TICK_SIZE, 0.0),
        p2: Point::new(x2 + DIM_TICK_SIZE, dim_y + DIM_TICK_SIZE, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(tick2));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    // Dimension text (positioned above dimension line)
    let text_value = if prefix.is_empty() {
        format!("{:.2}m", length)
    } else {
        format!("{}{:.2}m", prefix, length)
    };
    let dim_text = Text {
        location: Point::new((x1 + x2) / 2.0, dim_y + DIM_TEXT_HEIGHT * 0.5, 0.0),
        text_height: DIM_TEXT_HEIGHT,
        value: text_value,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(dim_text));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);
}

/// Draw a vertical dimension line
pub fn draw_dim_vertical(
    drawing: &mut Drawing,
    x: f64,
    y1: f64,
    y2: f64,
    offset: f64,
    prefix: &str,
) {
    let dim_x = x + offset;
    let length = (y2 - y1).abs();

    // Extension lines
    let ext1 = Line {
        p1: Point::new(x, y1, 0.0),
        p2: Point::new(dim_x + DIM_TICK_SIZE, y1, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ext1));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    let ext2 = Line {
        p1: Point::new(x, y2, 0.0),
        p2: Point::new(dim_x + DIM_TICK_SIZE, y2, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(ext2));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    // Dimension line
    let dim_line = Line {
        p1: Point::new(dim_x, y1, 0.0),
        p2: Point::new(dim_x, y2, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(dim_line));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    // Tick marks
    let tick1 = Line {
        p1: Point::new(dim_x - DIM_TICK_SIZE, y1 - DIM_TICK_SIZE, 0.0),
        p2: Point::new(dim_x + DIM_TICK_SIZE, y1 + DIM_TICK_SIZE, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(tick1));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    let tick2 = Line {
        p1: Point::new(dim_x - DIM_TICK_SIZE, y2 - DIM_TICK_SIZE, 0.0),
        p2: Point::new(dim_x + DIM_TICK_SIZE, y2 + DIM_TICK_SIZE, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(tick2));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);

    // Dimension text (rotated 90 degrees for vertical, positioned beside line)
    let text_value = if prefix.is_empty() {
        format!("{:.2}m", length)
    } else {
        format!("{}{:.2}m", prefix, length)
    };
    let dim_text = Text {
        location: Point::new(dim_x + DIM_TEXT_HEIGHT * 0.5, (y1 + y2) / 2.0, 0.0),
        text_height: DIM_TEXT_HEIGHT,
        value: text_value,
        rotation: 90.0,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(dim_text));
    entity.common.layer = LAYER_DIMENSIONS.to_string();
    drawing.add_entity(entity);
}

/// Draw a door symbol (plan view) with swing arc
/// hinge_x, hinge_y: position of door hinge
/// direction: 0=right, 90=up, 180=left, 270=down (swing direction)
/// swing: true=clockwise, false=counter-clockwise
pub fn draw_door(
    drawing: &mut Drawing,
    hinge_x: f64,
    hinge_y: f64,
    width: f64,
    direction: f64,
    swing_cw: bool,
) {
    let rad = direction.to_radians();

    // Door leaf (closed position line)
    let door_end_x = hinge_x + width * rad.cos();
    let door_end_y = hinge_y + width * rad.sin();

    let door_line = Line {
        p1: Point::new(hinge_x, hinge_y, 0.0),
        p2: Point::new(door_end_x, door_end_y, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(door_line));
    entity.common.layer = LAYER_DOORS.to_string();
    drawing.add_entity(entity);

    // Door swing arc (90 degree arc)
    let (start_angle, end_angle) = if swing_cw {
        (direction, direction - 90.0)
    } else {
        (direction, direction + 90.0)
    };

    let arc = Arc {
        center: Point::new(hinge_x, hinge_y, 0.0),
        radius: width,
        start_angle: start_angle.min(end_angle),
        end_angle: start_angle.max(end_angle),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Arc(arc));
    entity.common.layer = LAYER_DOORS.to_string();
    drawing.add_entity(entity);

    // Wall opening (break in wall shown as gap) - draw threshold line
    let threshold = Line {
        p1: Point::new(hinge_x, hinge_y, 0.0),
        p2: Point::new(
            hinge_x + width * (direction + if swing_cw { -90.0 } else { 90.0 }).to_radians().cos(),
            hinge_y + width * (direction + if swing_cw { -90.0 } else { 90.0 }).to_radians().sin(),
            0.0,
        ),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(threshold));
    entity.common.layer = LAYER_DOORS.to_string();
    drawing.add_entity(entity);
}

/// Draw a window symbol (plan view) - double line with glass indication
pub fn draw_window(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    width: f64,
    wall_thickness: f64,
    horizontal: bool,
) {
    if horizontal {
        // Window in horizontal wall
        let frame = polyline_from_points(vec![
            Point::new(x, y, 0.0),
            Point::new(x + width, y, 0.0),
            Point::new(x + width, y + wall_thickness, 0.0),
            Point::new(x, y + wall_thickness, 0.0),
            Point::new(x, y, 0.0),
        ]);
        let mut entity = Entity::new(EntityType::LwPolyline(frame));
        entity.common.layer = LAYER_WINDOWS.to_string();
        drawing.add_entity(entity);

        // Glass line (center)
        let glass = Line {
            p1: Point::new(x, y + wall_thickness / 2.0, 0.0),
            p2: Point::new(x + width, y + wall_thickness / 2.0, 0.0),
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Line(glass));
        entity.common.layer = LAYER_WINDOWS.to_string();
        drawing.add_entity(entity);
    } else {
        // Window in vertical wall
        let frame = polyline_from_points(vec![
            Point::new(x, y, 0.0),
            Point::new(x + wall_thickness, y, 0.0),
            Point::new(x + wall_thickness, y + width, 0.0),
            Point::new(x, y + width, 0.0),
            Point::new(x, y, 0.0),
        ]);
        let mut entity = Entity::new(EntityType::LwPolyline(frame));
        entity.common.layer = LAYER_WINDOWS.to_string();
        drawing.add_entity(entity);

        // Glass line (center)
        let glass = Line {
            p1: Point::new(x + wall_thickness / 2.0, y, 0.0),
            p2: Point::new(x + wall_thickness / 2.0, y + width, 0.0),
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Line(glass));
        entity.common.layer = LAYER_WINDOWS.to_string();
        drawing.add_entity(entity);
    }
}

/// Draw deck planking pattern
pub fn draw_deck_pattern(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) {
    let board_spacing = DECK_BOARD_WIDTH + DECK_BOARD_GAP;
    let mut y = y1 + board_spacing;

    while y < y2 {
        let board_line = Line {
            p1: Point::new(x1, y, 0.0),
            p2: Point::new(x2, y, 0.0),
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Line(board_line));
        entity.common.layer = LAYER_DECK_PATTERN.to_string();
        drawing.add_entity(entity);
        y += board_spacing;
    }
}

/// Draw a room label with area
pub fn draw_room_label(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    name: &str,
    area: f64,
) {
    // Room name
    let name_text = Text {
        location: Point::new(x, y + LABEL_TEXT_HEIGHT * 0.6, 0.0),
        text_height: LABEL_TEXT_HEIGHT,
        value: name.to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(name_text));
    entity.common.layer = LAYER_TEXT.to_string();
    drawing.add_entity(entity);

    // Area
    let area_text = Text {
        location: Point::new(x, y - LABEL_TEXT_HEIGHT * 0.6, 0.0),
        text_height: DIM_TEXT_HEIGHT,
        value: format!("{:.1} m²", area),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(area_text));
    entity.common.layer = LAYER_TEXT.to_string();
    drawing.add_entity(entity);
}

/// Draw north arrow
pub fn draw_north_arrow(drawing: &mut Drawing, x: f64, y: f64, size: f64) {
    // Arrow shaft
    let shaft = Line {
        p1: Point::new(x, y, 0.0),
        p2: Point::new(x, y + size, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(shaft));
    entity.common.layer = LAYER_ANNO.to_string();
    drawing.add_entity(entity);

    // Arrow head (triangle)
    let head = polyline_from_points(vec![
        Point::new(x, y + size, 0.0),
        Point::new(x - size * 0.2, y + size * 0.7, 0.0),
        Point::new(x, y + size * 0.8, 0.0),
        Point::new(x + size * 0.2, y + size * 0.7, 0.0),
        Point::new(x, y + size, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(head));
    entity.common.layer = LAYER_ANNO.to_string();
    drawing.add_entity(entity);

    // N label
    let n_text = Text {
        location: Point::new(x - size * 0.1, y + size + size * 0.2, 0.0),
        text_height: size * 0.3,
        value: "N".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(n_text));
    entity.common.layer = LAYER_ANNO.to_string();
    drawing.add_entity(entity);
}

/// Draw title block
pub fn draw_title_block(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    // Outer border
    let border = polyline_from_points(vec![
        Point::new(x, y, 0.0),
        Point::new(x + width, y, 0.0),
        Point::new(x + width, y + height, 0.0),
        Point::new(x, y + height, 0.0),
        Point::new(x, y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(border));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);

    // Horizontal dividers
    let div1 = Line {
        p1: Point::new(x, y + height * 0.5, 0.0),
        p2: Point::new(x + width, y + height * 0.5, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(div1));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);

    let div2 = Line {
        p1: Point::new(x, y + height * 0.25, 0.0),
        p2: Point::new(x + width, y + height * 0.25, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(div2));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);

    // Project title
    let title = Text {
        location: Point::new(x + width * 0.05, y + height * 0.7, 0.0),
        text_height: TITLE_TEXT_HEIGHT,
        value: "SAUNA BUILDING".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(title));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);

    // Drawing title
    let drawing_title = Text {
        location: Point::new(x + width * 0.05, y + height * 0.35, 0.0),
        text_height: LABEL_TEXT_HEIGHT,
        value: "FLOOR PLAN & ELEVATIONS".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(drawing_title));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);

    // Scale
    let scale_text = Text {
        location: Point::new(x + width * 0.05, y + height * 0.1, 0.0),
        text_height: DIM_TEXT_HEIGHT,
        value: format!("SCALE: {}", DRAWING_SCALE),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(scale_text));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);

    // Date
    let date_text = Text {
        location: Point::new(x + width * 0.5, y + height * 0.1, 0.0),
        text_height: DIM_TEXT_HEIGHT,
        value: "DATE: 2026-01-03".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(date_text));
    entity.common.layer = LAYER_TITLE.to_string();
    drawing.add_entity(entity);
}
