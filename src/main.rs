mod constants;
mod helpers;
mod layers;
mod plan;
mod views;

use dxf::enums::AcadVersion;
use dxf::Drawing;

use constants::*;
use layers::setup_layers;
use plan::draw_floor_plan;
use views::*;

fn main() {
    let mut drawing = Drawing::new();
    drawing.header.version = AcadVersion::R2013;

    // Set up drawing units (meters)
    drawing.header.default_drawing_units = dxf::enums::Units::Meters;

    // Set up all layers
    setup_layers(&mut drawing);

    // Draw floor plan with all elements
    draw_floor_plan(&mut drawing);

    // Create elevation views offset from plan view
    let offset_x = TOTAL_WIDTH + 2.0;
    let offset_y = DECK_DEPTH;

    create_front_elevation(&mut drawing, offset_x, offset_y);
    create_top_view(&mut drawing, offset_x, offset_y - 5.0);
    create_side_elevation(&mut drawing, offset_x + 5.0, offset_y);
    create_section_cut(&mut drawing, offset_x + 8.0, offset_y);

    // Save the drawing
    match drawing.save_file("sauna_design.dxf") {
        Ok(_) => {
            println!("========================================");
            println!("  SAUNA BUILDING - CONSTRUCTION DRAWING");
            println!("========================================");
            println!();
            println!("Drawing saved: sauna_design.dxf");
            println!("Scale: {}", DRAWING_SCALE);
            println!();
            println!("DIMENSIONS:");
            println!("  Total width:     {:.2} m", TOTAL_WIDTH);
            println!("  Building depth:  {:.2} m", BUILDING_DEPTH);
            println!("  Deck depth:      {:.2} m", DECK_DEPTH);
            println!("  Wall thickness:  {:.0} mm", WALL_THICKNESS * 1000.0);
            println!("  Ceiling height:  {:.2} m", BUILDING_HEIGHT);
            println!();
            println!("ROOM AREAS:");
            println!("  Sauna:           {:.1} m²", sauna_area());
            println!("  Wash:            {:.1} m²", wash_area());
            println!("  Changing:        {:.1} m²", seating_area());
            println!("  Deck:            {:.1} m²", outdoor_area() + deck_area());
            println!("  ─────────────────────────");
            println!("  TOTAL:           {:.1} m²", total_floor_area());
            println!();
            println!("LAYERS:");
            println!("  A-WALL       Exterior walls");
            println!("  A-WALL-INTR  Interior partitions");
            println!("  A-DOOR       Doors");
            println!("  A-GLAZ       Windows");
            println!("  A-DIMS       Dimensions");
            println!("  A-TEXT       Labels");
            println!("  A-DECK       Deck structure");
            println!("  A-DECK-PATT  Deck planking");
            println!("  A-PLMB-FIXT  Hot tub");
            println!("  A-FIXT       Fixtures");
            println!("  A-ROOF       Roof");
            println!();
        }
        Err(e) => eprintln!("Error saving drawing: {}", e),
    }
}
