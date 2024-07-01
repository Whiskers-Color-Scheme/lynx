use colored::Colorize;
use tabled::{
    settings::Style,
    Table, Tabled,
};
use whiskers_palette_rs::{
    color::Color,
    palette::{get_panther_palette, get_tiger_palette},
};

#[derive(Tabled)]
struct TableCell {
    name: String,
    hex: String,
    rgb: String,
    hsl: String,
}

pub fn show_palette() {
    let mut panther_cells = Vec::<TableCell>::new();

    for color in get_panther_palette().colors() {
        panther_cells.push(get_color_cell(color));
    }

    let mut panther_table = Table::new(panther_cells);
    panther_table.with(Style::modern());

    println!("🖤 Panther\n{}", panther_table);

    let mut tiger_cells = Vec::<TableCell>::new();

    for color in get_tiger_palette().colors() {
        tiger_cells.push(get_color_cell(color));
    }

    let mut tiger_table = Table::new(tiger_cells);
    tiger_table.with(Style::modern());

    println!("\n🐯 Tiger\n{}", tiger_table);
}

fn get_color_cell(color: Color) -> TableCell {
    let r: u8 = color.rgb.r.parse().unwrap();
    let g: u8 = color.rgb.g.parse().unwrap();
    let b: u8 = color.rgb.b.parse().unwrap();

    TableCell {
        name: color.name,
        hex: color.hex.truecolor(r, g, b).to_string(),
        rgb: color.rgb.rgb.truecolor(r, g, b).to_string(),
        hsl: color.hsl.hsl.truecolor(r, g, b).to_string(),
    }
}
