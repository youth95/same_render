use colored::Colorize;
use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::hinting::HintingOptions;
use font_kit::source::SystemSource;
use pathfinder_geometry::rect::RectI;
use pathfinder_geometry::transform2d::Transform2F;
use pbr::ProgressBar;
use prettytable::{Attr, Cell, Row, Table};
use std::fmt::Write;

struct App {
    chart_slice: Vec<(RectI, Canvas)>,
}

impl App {
    pub fn new() -> Self {
        return App {
            chart_slice: vec![],
        };
    }

    pub fn render_str(&mut self, text: String) -> &mut Self {
        for ch in text.chars() {
            self.chart_slice.push(App::render_char(ch));
        }
        return self;
    }

    pub fn render_char(ch: char) -> (RectI, Canvas) {
        let font = SystemSource::new()
            .select_by_postscript_name("仿宋")
            .unwrap()
            .load()
            .unwrap();
        let point_size = 64.0;
        let transform = Transform2F::default().scale(1.0);
        let glyph_id = font.glyph_for_char(ch).unwrap();
        // let hinting_options = HintingOptions::Full(1.0);
        let hinting_options = HintingOptions::None;
        let rasterization_options = RasterizationOptions::Bilevel;
        // let rasterization_options = RasterizationOptions::GrayscaleAa;
        // let rasterization_options = RasterizationOptions::SubpixelAa;
        let raster_rect = font
            .raster_bounds(
                glyph_id,
                point_size,
                transform,
                hinting_options,
                rasterization_options,
            )
            .unwrap();
        let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            point_size,
            Transform2F::from_translation(-raster_rect.origin().to_f32()) * transform,
            hinting_options,
            rasterization_options,
        )
        .unwrap();

        return (raster_rect, canvas);
    }

    pub fn show(&self) {
        for (raster_rect, canvas) in &self.chart_slice {
            println!("{:?}", raster_rect.size())
        }
        // let height = self.chart_slice[0].0.height();
        // for y in 0..height {
        //     let mut line = String::new();
        //     for (raster_rect, canvas) in &self.chart_slice {
        //         let (row_start, row_end) =
        //             (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
        //         let row = &canvas.pixels[row_start..row_end]; // 取当前行的像素值
        //         for x in 0..raster_rect.width() {
        //             match canvas.format {
        //                 Format::Rgba32 => unimplemented!(),
        //                 Format::Rgb24 => {
        //                     write!(
        //                         &mut line,
        //                         "{}{}{}",
        //                         shade(row[x as usize * 3]).to_string().red(),
        //                         shade(row[x as usize * 3 + 1]).to_string().green(),
        //                         shade(row[x as usize * 3 + 2]).to_string().blue()
        //                     )
        //                     .unwrap();
        //                 }
        //                 Format::A8 => {
        //                     let shade = shade(row[x as usize]);
        //                     line.push(shade);
        //                     line.push(shade);
        //                 }
        //             }
        //         }
        //     }
        //     println!("{}", line)
        // }
    }
}

fn main() {
    // list();
    // for ch in "爱萍萍，爱生活".to_string().chars() {
    //     render(ch);
    // }

    App::new().render_str("爱萍萍，爱生活".to_string()).show();
}

fn shade(value: u8) -> char {
    match value {
        0 => ' ',
        1..=84 => '░',
        85..=169 => '▒',
        170..=254 => '▓',
        _ => '█',
    }
}

fn render(ch: char) {
    let font = SystemSource::new()
        .select_by_postscript_name("仿宋")
        .unwrap()
        .load()
        .unwrap();
    let point_size = 64.0;
    let transform = Transform2F::default().scale(1.0);
    let glyph_id = font.glyph_for_char(ch).unwrap();
    // let hinting_options = HintingOptions::Full(1.0);
    let hinting_options = HintingOptions::None;
    let rasterization_options = RasterizationOptions::Bilevel;
    // let rasterization_options = RasterizationOptions::GrayscaleAa;
    // let rasterization_options = RasterizationOptions::SubpixelAa;
    let raster_rect = font
        .raster_bounds(
            glyph_id,
            point_size,
            transform,
            hinting_options,
            rasterization_options,
        )
        .unwrap();
    let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        point_size,
        Transform2F::from_translation(-raster_rect.origin().to_f32()) * transform,
        hinting_options,
        rasterization_options,
    )
    .unwrap();

    // println!("font metrics: {:?}", font.metrics());

    // println!(
    //     "glyph:{} size:{:?}",
    //     glyph_id,
    //     (raster_rect.width(), raster_rect.height())
    // );
    for y in 0..raster_rect.height() {
        let mut line = String::new();
        let (row_start, row_end) = (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
        let row = &canvas.pixels[row_start..row_end]; // 取当前行的像素值
        for x in 0..raster_rect.width() {
            match canvas.format {
                Format::Rgba32 => unimplemented!(),
                Format::Rgb24 => {
                    write!(
                        &mut line,
                        "{}{}{}",
                        shade(row[x as usize * 3]).to_string().red(),
                        shade(row[x as usize * 3 + 1]).to_string().green(),
                        shade(row[x as usize * 3 + 2]).to_string().blue()
                    )
                    .unwrap();
                }
                Format::A8 => {
                    let shade = shade(row[x as usize]);
                    line.push(shade);
                    line.push(shade);
                }
            }
        }
        println!("{}", line);
    }
}
fn list() {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(vec![
        Cell::new("PostScript Name").with_style(Attr::Bold),
        Cell::new("Name").with_style(Attr::Bold),
        Cell::new("Family").with_style(Attr::Bold),
        Cell::new("Style").with_style(Attr::Bold),
        Cell::new("Weight").with_style(Attr::Bold),
        Cell::new("Stretch").with_style(Attr::Bold),
    ]));

    let source = SystemSource::new();
    let fonts = source.all_fonts().unwrap();
    let mut progress_bar = ProgressBar::new(fonts.len() as u64);
    progress_bar.message("Loading fonts… ");

    for font in fonts {
        if let Ok(font) = font.load() {
            let properties = font.properties();
            table.add_row(Row::new(vec![
                Cell::new(&font.postscript_name().unwrap_or_else(|| "".to_owned())),
                Cell::new(&font.full_name()),
                Cell::new(&font.family_name()),
                Cell::new(&properties.style.to_string()),
                Cell::new(&properties.weight.0.to_string()),
                Cell::new(&properties.stretch.0.to_string()),
            ]));
        }

        progress_bar.inc();
    }

    progress_bar.finish_print("");
    table.printstd();
}
