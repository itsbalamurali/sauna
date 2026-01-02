use dxf::entities::*;
use dxf::enums::AcadVersion;
use dxf::{Color, Drawing, LwPolylineVertex, Point};

fn main() {
    let mut drawing = Drawing::new();

    // Set up drawing units and limits
    drawing.header.version = AcadVersion::R2013;

    // Create the sauna building structure
    create_sauna_room(&mut drawing);

    // Create the shower area inside the sauna
    create_shower_area(&mut drawing);

    // Create the deck/terrace around the sauna
    create_deck(&mut drawing);

    // Create the circular hot tub connected to the deck
    create_hot_tub(&mut drawing);

    // Add labels and dimensions
    add_labels(&mut drawing);

    // Save the drawing
    match drawing.save_file("sauna_design.dxf") {
        Ok(_) => println!("✓ Sauna drawing saved successfully to 'sauna_design.dxf'"),
        Err(e) => eprintln!("Error saving drawing: {}", e),
    }
}

fn create_sauna_room(drawing: &mut Drawing) {
    // Main sauna room dimensions: 12' x 10' (3.66m x 3.05m)
    let sauna_width = 366.0;
    let sauna_depth = 305.0;
    let wall_thickness = 15.0;

    // Outer walls of sauna
    let outer_wall = polyline_from_points(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(sauna_width, 0.0, 0.0),
        Point::new(sauna_width, sauna_depth, 0.0),
        Point::new(0.0, sauna_depth, 0.0),
        Point::new(0.0, 0.0, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(outer_wall));
    entity.common.color = Color::from_index(1); // Red for outer walls
    drawing.add_entity(entity);

    // Inner walls of sauna
    let inner_wall = polyline_from_points(vec![
        Point::new(wall_thickness, wall_thickness, 0.0),
        Point::new(sauna_width - wall_thickness, wall_thickness, 0.0),
        Point::new(
            sauna_width - wall_thickness,
            sauna_depth - wall_thickness,
            0.0,
        ),
        Point::new(wall_thickness, sauna_depth - wall_thickness, 0.0),
        Point::new(wall_thickness, wall_thickness, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(inner_wall));
    entity.common.color = Color::from_index(1);
    drawing.add_entity(entity);

    // Sauna benches (tiered seating)
    // Lower bench
    let lower_bench = polyline_from_points(vec![
        Point::new(wall_thickness + 10.0, wall_thickness + 10.0, 0.0),
        Point::new(
            sauna_width - wall_thickness - 10.0,
            wall_thickness + 10.0,
            0.0,
        ),
        Point::new(
            sauna_width - wall_thickness - 10.0,
            wall_thickness + 60.0,
            0.0,
        ),
        Point::new(wall_thickness + 10.0, wall_thickness + 60.0, 0.0),
        Point::new(wall_thickness + 10.0, wall_thickness + 10.0, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(lower_bench));
    entity.common.color = Color::from_index(6); // Magenta for benches
    drawing.add_entity(entity);

    // Upper bench
    let upper_bench = polyline_from_points(vec![
        Point::new(wall_thickness + 30.0, wall_thickness + 30.0, 0.0),
        Point::new(
            sauna_width - wall_thickness - 30.0,
            wall_thickness + 30.0,
            0.0,
        ),
        Point::new(
            sauna_width - wall_thickness - 30.0,
            wall_thickness + 80.0,
            0.0,
        ),
        Point::new(wall_thickness + 30.0, wall_thickness + 80.0, 0.0),
        Point::new(wall_thickness + 30.0, wall_thickness + 30.0, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(upper_bench));
    entity.common.color = Color::from_index(6);
    drawing.add_entity(entity);

    // Sauna heater/stove
    let heater = polyline_from_points(vec![
        Point::new(sauna_width / 2.0 - 25.0, wall_thickness + 100.0, 0.0),
        Point::new(sauna_width / 2.0 + 25.0, wall_thickness + 100.0, 0.0),
        Point::new(sauna_width / 2.0 + 25.0, wall_thickness + 150.0, 0.0),
        Point::new(sauna_width / 2.0 - 25.0, wall_thickness + 150.0, 0.0),
        Point::new(sauna_width / 2.0 - 25.0, wall_thickness + 100.0, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(heater));
    entity.common.color = Color::from_index(3); // Green for heater
    drawing.add_entity(entity);

    // Door opening (on the front wall)
    let door_width = 80.0;
    let door_x = sauna_width - 100.0;
    let door = Line {
        p1: Point::new(door_x, sauna_depth, 0.0),
        p2: Point::new(door_x + door_width, sauna_depth, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(door));
    entity.common.color = Color::from_index(4); // Cyan for door
    drawing.add_entity(entity);

    // Door arc (showing swing)
    let door_arc = Arc {
        center: Point::new(door_x, sauna_depth, 0.0),
        radius: door_width,
        start_angle: 270.0,
        end_angle: 360.0,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Arc(door_arc));
    entity.common.color = Color::from_index(4);
    drawing.add_entity(entity);
}

fn create_shower_area(drawing: &mut Drawing) {
    // Shower area in corner of sauna: 3' x 3' (91cm x 91cm)
    let shower_size = 91.0;
    let shower_x = 15.0 + 10.0;
    let shower_y = 305.0 - 15.0 - 10.0 - shower_size;

    // Shower enclosure
    let shower_walls = polyline_from_points(vec![
        Point::new(shower_x, shower_y, 0.0),
        Point::new(shower_x + shower_size, shower_y, 0.0),
        Point::new(shower_x + shower_size, shower_y + shower_size, 0.0),
        Point::new(shower_x, shower_y + shower_size, 0.0),
        Point::new(shower_x, shower_y, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(shower_walls));
    entity.common.color = Color::from_index(4); // Cyan for shower
    drawing.add_entity(entity);

    // Shower drain (small circle)
    let drain = Circle {
        center: Point::new(
            shower_x + shower_size / 2.0,
            shower_y + shower_size / 2.0,
            0.0,
        ),
        radius: 5.0,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(drain));
    entity.common.color = Color::from_index(4);
    drawing.add_entity(entity);

    // Shower head (small circle on wall)
    let shower_head = Circle {
        center: Point::new(shower_x + 10.0, shower_y + shower_size - 20.0, 0.0),
        radius: 3.0,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(shower_head));
    entity.common.color = Color::from_index(4);
    drawing.add_entity(entity);
}

fn create_deck(drawing: &mut Drawing) {
    // Deck surrounds the sauna with 6' (183cm) extension on all sides
    let sauna_width = 366.0;
    let sauna_depth = 305.0;
    let deck_extension = 183.0;

    // Outer deck boundary
    let deck_outer = polyline_from_points(vec![
        Point::new(-deck_extension, -deck_extension, 0.0),
        Point::new(sauna_width + deck_extension, -deck_extension, 0.0),
        Point::new(
            sauna_width + deck_extension,
            sauna_depth + deck_extension,
            0.0,
        ),
        Point::new(-deck_extension, sauna_depth + deck_extension, 0.0),
        Point::new(-deck_extension, -deck_extension, 0.0),
    ]);
    let mut entity = Entity::new(EntityType::LwPolyline(deck_outer));
    entity.common.color = Color::from_index(5); // Blue for deck
    drawing.add_entity(entity);

    // Deck boards (horizontal lines to show planking)
    let board_spacing = 15.0;
    let mut y = -deck_extension + board_spacing;

    while y < sauna_depth + deck_extension {
        let board = Line {
            p1: Point::new(-deck_extension, y, 0.0),
            p2: Point::new(sauna_width + deck_extension, y, 0.0),
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Line(board));
        entity.common.color = Color::from_index(8); // Dark gray for deck boards
        drawing.add_entity(entity);
        y += board_spacing;
    }

    // Deck railing posts (around perimeter)
    let post_spacing = 120.0;

    // Front railing posts
    let mut x = -deck_extension;
    while x <= sauna_width + deck_extension {
        let post = Circle {
            center: Point::new(x, -deck_extension, 0.0),
            radius: 5.0,
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Circle(post));
        entity.common.color = Color::from_index(2); // Yellow for posts
        drawing.add_entity(entity);
        x += post_spacing;
    }

    // Back railing posts
    x = -deck_extension;
    while x <= sauna_width + deck_extension {
        let post = Circle {
            center: Point::new(x, sauna_depth + deck_extension, 0.0),
            radius: 5.0,
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Circle(post));
        entity.common.color = Color::from_index(2);
        drawing.add_entity(entity);
        x += post_spacing;
    }

    // Left railing posts
    let mut y = -deck_extension;
    while y <= sauna_depth + deck_extension {
        let post = Circle {
            center: Point::new(-deck_extension, y, 0.0),
            radius: 5.0,
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Circle(post));
        entity.common.color = Color::from_index(2);
        drawing.add_entity(entity);
        y += post_spacing;
    }

    // Right railing posts
    y = -deck_extension;
    while y <= sauna_depth + deck_extension {
        let post = Circle {
            center: Point::new(sauna_width + deck_extension, y, 0.0),
            radius: 5.0,
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Circle(post));
        entity.common.color = Color::from_index(2);
        drawing.add_entity(entity);
        y += post_spacing;
    }

    // Steps/stairs leading to deck
    let step_width = 150.0;
    let step_depth = 30.0;
    let step_x = sauna_width / 2.0 - step_width / 2.0;

    for i in 0..4 {
        let step_y = sauna_depth + deck_extension + i as f64 * step_depth;
        let step = polyline_from_points(vec![
            Point::new(step_x, step_y, 0.0),
            Point::new(step_x + step_width, step_y, 0.0),
            Point::new(step_x + step_width, step_y + step_depth, 0.0),
            Point::new(step_x, step_y + step_depth, 0.0),
            Point::new(step_x, step_y, 0.0),
        ]);
        let mut entity = Entity::new(EntityType::LwPolyline(step));
        entity.common.color = Color::from_index(5);
        drawing.add_entity(entity);
    }
}

fn create_hot_tub(drawing: &mut Drawing) {
    // Circular hot tub: 7' diameter (213cm) positioned to the left of the sauna
    let hot_tub_radius = 106.5;
    let hot_tub_x = -120.0;
    let hot_tub_y = 152.5; // Centered vertically with sauna

    // Hot tub outer circle
    let hot_tub_outer = Circle {
        center: Point::new(hot_tub_x, hot_tub_y, 0.0),
        radius: hot_tub_radius,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(hot_tub_outer));
    entity.common.color = Color::from_index(4); // Cyan for water feature
    drawing.add_entity(entity);

    // Hot tub inner circle (seating area)
    let hot_tub_inner = Circle {
        center: Point::new(hot_tub_x, hot_tub_y, 0.0),
        radius: hot_tub_radius - 20.0,
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Circle(hot_tub_inner));
    entity.common.color = Color::from_index(4);
    drawing.add_entity(entity);

    // Hot tub seats (4 positions around the circle)
    for i in 0..4 {
        let angle = (i as f64 * 90.0 + 45.0).to_radians();
        let seat_x = hot_tub_x + (hot_tub_radius - 40.0) * angle.cos();
        let seat_y = hot_tub_y + (hot_tub_radius - 40.0) * angle.sin();

        let seat = Circle {
            center: Point::new(seat_x, seat_y, 0.0),
            radius: 15.0,
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Circle(seat));
        entity.common.color = Color::from_index(6); // Magenta for seats
        drawing.add_entity(entity);
    }

    // Jets around the hot tub (8 positions)
    for i in 0..8 {
        let angle = (i as f64 * 45.0).to_radians();
        let jet_x = hot_tub_x + (hot_tub_radius - 10.0) * angle.cos();
        let jet_y = hot_tub_y + (hot_tub_radius - 10.0) * angle.sin();

        let jet = Circle {
            center: Point::new(jet_x, jet_y, 0.0),
            radius: 3.0,
            ..Default::default()
        };
        let mut entity = Entity::new(EntityType::Circle(jet));
        entity.common.color = Color::from_index(3); // Green for jets
        drawing.add_entity(entity);
    }

    // Hot tub steps
    let step_width = 60.0;
    let step_depth = 25.0;
    let step_x = hot_tub_x + hot_tub_radius;
    let step_y = hot_tub_y - step_width / 2.0;

    for i in 0..3 {
        let step = polyline_from_points(vec![
            Point::new(step_x + i as f64 * step_depth, step_y, 0.0),
            Point::new(step_x + (i + 1) as f64 * step_depth, step_y, 0.0),
            Point::new(
                step_x + (i + 1) as f64 * step_depth,
                step_y + step_width,
                0.0,
            ),
            Point::new(step_x + i as f64 * step_depth, step_y + step_width, 0.0),
            Point::new(step_x + i as f64 * step_depth, step_y, 0.0),
        ]);
        let mut entity = Entity::new(EntityType::LwPolyline(step));
        entity.common.color = Color::from_index(5);
        drawing.add_entity(entity);
    }
}

fn add_labels(drawing: &mut Drawing) {
    // Add text labels for different areas
    let sauna_text = Text {
        location: Point::new(183.0, 280.0, 0.0),
        text_height: 20.0,
        value: "SAUNA".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(sauna_text));
    entity.common.color = Color::from_index(7);
    drawing.add_entity(entity);

    // Label for shower
    let shower_text = Text {
        location: Point::new(30.0, 170.0, 0.0),
        text_height: 10.0,
        value: "SHOWER".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(shower_text));
    entity.common.color = Color::from_index(7);
    drawing.add_entity(entity);

    // Label for deck
    let deck_text = Text {
        location: Point::new(100.0, -100.0, 0.0),
        text_height: 15.0,
        value: "DECK/TERRACE".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(deck_text));
    entity.common.color = Color::from_index(7);
    drawing.add_entity(entity);

    // Label for hot tub
    let hottub_text = Text {
        location: Point::new(-170.0, 130.0, 0.0),
        text_height: 12.0,
        value: "HOT TUB".to_string(),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Text(hottub_text));
    entity.common.color = Color::from_index(7);
    drawing.add_entity(entity);

    // Add dimension lines
    // Top dimension line (sauna width)
    let dim_line_top = Line {
        p1: Point::new(0.0, 330.0, 0.0),
        p2: Point::new(366.0, 330.0, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(dim_line_top));
    entity.common.color = Color::from_index(2);
    drawing.add_entity(entity);

    // Side dimension line (sauna depth)
    let dim_line_side = Line {
        p1: Point::new(390.0, 0.0, 0.0),
        p2: Point::new(390.0, 305.0, 0.0),
        ..Default::default()
    };
    let mut entity = Entity::new(EntityType::Line(dim_line_side));
    entity.common.color = Color::from_index(2);
    drawing.add_entity(entity);
}

// Helper function to create a polyline from a vector of points
fn polyline_from_points(points: Vec<Point>) -> LwPolyline {
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
