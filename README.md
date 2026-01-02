# Sauna Design Generator

A Rust application that generates professional DXF (Drawing Exchange Format) CAD drawings for a complete sauna facility with outdoor amenities.

## Quick Start

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and run
cd sauna
cargo build --release
cargo run --release
```

Output: `sauna_design.dxf` (36KB) - Open with LibreCAD, AutoCAD, or any DXF viewer.

---

## Features

### 🏠 Main Sauna Room (12' × 10')
- Double-wall construction (15cm thick)
- Tiered bench seating (upper & lower)
- Central heater/stove (50×50cm)
- Outward-opening door with swing arc

### 🚿 Shower Area (3' × 3')
- Integrated within sauna
- Wall-mounted shower head
- Center floor drain
- Waterproof enclosure

### 🏞️ Deck/Terrace (24' × 22')
- 6' extension on all sides
- Horizontal deck board pattern
- Railing posts every 120cm
- 4-step access stairs (150cm wide)

### 🛁 Circular Hot Tub (7' diameter)
- 4 designated seating positions
- 8 hydrotherapy jets
- 3-step entry staircase
- Connected to deck

---

## Technical Specifications

### Dimensions (in centimeters)

| Component | Size | Details |
|-----------|------|---------|
| Sauna Exterior | 366 × 305 | 12' × 10' |
| Sauna Interior | 336 × 275 | After 15cm walls |
| Shower | 91 × 91 | 3' × 3' corner unit |
| Hot Tub | Ø 213 | 7' diameter, 90cm deep |
| Deck Total | 732 × 671 | 24' × 22' |
| Wall Thickness | 15 | Double-wall insulated |

### DXF Output Details

- **Format**: AutoCAD R2013
- **Units**: Centimeters
- **Entities**: 150+ elements (polylines, circles, arcs, lines, text)

### Color Coding

| Color | Component | Index |
|-------|-----------|-------|
| Red | Sauna walls | 1 |
| Yellow | Railing posts | 2 |
| Green | Heater, jets | 3 |
| Cyan | Water features | 4 |
| Blue | Deck structure | 5 |
| Magenta | Seating | 6 |
| White | Labels | 7 |
| Gray | Deck boards | 8 |

---

## Customization

Edit `src/main.rs` to modify dimensions:

```rust
// Sauna dimensions
let sauna_width = 366.0;   // cm
let sauna_depth = 305.0;   // cm
let wall_thickness = 15.0;

// Deck size
let deck_extension = 183.0; // cm (6 feet)

// Hot tub
let hot_tub_radius = 106.5; // cm (7 feet diameter)

// Railing
let post_spacing = 120.0;  // cm between posts
```

After changes: `cargo run --release`

---

## Material Specifications

### Sauna Interior
- **Wood**: Western Red Cedar, Hemlock, or Aspen
- **Grade**: Clear, knot-free
- **Finish**: Natural (no stain/varnish for safety)
- **Benches**: Lower 45cm high, Upper 90cm high

### Sauna Heater
- **Power**: 6-8 kW (electric or wood-burning)
- **Clearance**: 75cm from benches
- **Location**: Center, against back wall

### Deck/Terrace
- **Decking**: Pressure-treated lumber or composite
- **Board Width**: 15cm (6") with 5mm gaps
- **Posts**: 4×4 on 120cm centers
- **Height**: 30-45cm above ground

### Hot Tub
- **Capacity**: ~1,500 liters (400 gallons)
- **Pump**: 1-2 HP circulation
- **Heater**: 4-6 kW
- **Shell**: Acrylic or polyethylene

---

## Building Code Compliance

### Safety Requirements
- **Sauna Door**: Must open outward
- **Deck Railings**: 106cm (42") height, 10cm max baluster spacing
- **Stairs**: Handrails both sides, 18cm risers, 30cm treads
- **Hot Tub**: GFCI protection required
- **Ventilation**: Fresh air intake and exhaust for sauna

### Electrical
- Sauna heater: 240V, 30-40 amp circuit
- Hot tub: 240V, 50-60 amp GFCI circuit
- Lighting: 12V LED (moisture-rated)

### Plumbing
- Shower: ½" supply lines, anti-scald valve, 2" drain
- Hot tub: ¾" fill line, 1.5" drain valve

---

## Installation Guidelines

### Foundation
- Reinforced concrete pad (10cm thick minimum)
- Level surface (±1cm tolerance)
- Footings below frost line

### Construction Timeline
1. Site prep: 1 week
2. Foundation/pad: 1-2 weeks
3. Deck framing: 1-2 weeks
4. Sauna construction: 2-3 weeks
5. Hot tub install: 1 week
6. Electrical/plumbing: 1 week
7. Finishing: 1-2 weeks
8. Inspections: 1 week

**Total: 6-10 weeks**

### Cost Estimate (USD)
- Sauna kit: $3,000 - $6,000
- Heater: $500 - $2,000
- Hot tub: $3,000 - $10,000
- Deck materials: $2,000 - $5,000
- Electrical: $1,500 - $3,000
- Plumbing: $800 - $2,000
- Labor: $5,000 - $15,000

**Total: $15,800 - $43,000**

---

## Maintenance

### Sauna
- Clean benches weekly
- Check heater rocks quarterly
- Inspect door seal annually
- Refinish wood every 3-5 years

### Hot Tub
- Test water chemistry 2-3x weekly
- Clean filter monthly
- Drain/refill quarterly
- Professional service annually

### Deck
- Clean annually
- Inspect seasonally
- Reseal/restain every 2-3 years
- Check structural integrity annually

---

## Safety Guidelines

### Sauna
- Max temp: 90°C (194°F)
- Session: 15-20 minutes max
- Stay hydrated
- No alcohol use
- Consult doctor if health concerns

### Hot Tub
- Max temp: 40°C (104°F)
- Session: 15-30 minutes
- Supervise children
- No glass containers
- Proper chemical balance

---

## Opening the DXF File

### Free Software
- **LibreCAD** (recommended): https://librecad.org/
- **FreeCAD**: https://www.freecad.org/
- **QCAD**: https://qcad.org/

### Professional
- AutoCAD
- BricsCAD
- DraftSight

### Online Viewers
- https://sharecad.org/
- https://www.autodesk.com/viewers

---

## Troubleshooting

**"cargo: command not found"**
- Install Rust: https://rustup.rs/

**"failed to compile"**
- Update Rust: `rustup update`

**"Cannot open DXF file"**
- Verify file exists: `ls -la sauna_design.dxf`
- Check file size: ~36KB
- Try different viewer

**"Colors not showing"**
- Enable color display in CAD software
- Check layer settings
- Try LibreCAD

---

## Project Structure

```
sauna/
├── Cargo.toml          # Dependencies (dxf = "0.6")
├── src/
│   └── main.rs         # Main application (430 lines)
├── AGENTS.md           # AI agent instructions
├── README.md           # This file
└── sauna_design.dxf    # Generated output
```

---

## Dependencies

```toml
[dependencies]
dxf = "0.6"
```

---

## License

Provided as-is for educational and design purposes.

## Contributing

Fork and customize for your own designs!

---

**Build your dream sauna! 🔥💦🧖**