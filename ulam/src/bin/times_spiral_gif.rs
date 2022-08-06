use clap::Parser;
use plotters::coord::types::RangedCoordf64;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::BitMapBackend;
use plotters::prelude::Cartesian2d;
use plotters::prelude::ChartBuilder;
use plotters::prelude::DrawingArea;
use plotters::prelude::TextStyle;
use plotters::prelude::BLACK;
use plotters::prelude::WHITE;
use plotters::style::IntoFont;
use std::fs::create_dir;
use std::fs::create_dir_all;
use std::path::Path;
use ulam::ulam::generator::generator::Generator;
use ulam::ulam::generator::times::TimesGenerator;
use ulam::ulam::tile::hexagon_spiral::HexagonSpiral;
use ulam::ulam::tile::square_spiral::SquareSpiral;
use ulam::ulam::tile::square_zigzag::SquareZigzag;
use ulam::ulam::tile::tile::Tile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg: AppArg = AppArg::parse();

    if !Path::new("output").is_dir() {
        create_dir("output")?;
    }
    let file_path = if let Some(ref file) = arg.output {
        format!("output/{}", file)
    } else {
        format!(
            "output/times-{}-{}-{}.gif",
            arg.tile, arg.image_size, arg.gp
        )
    };

    let path = Path::new(&file_path);
    if let Some(ref parent) = path.parent() {
        if !parent.is_dir() {
            println!("{:?}", parent);
            create_dir_all(parent).expect(&format!("Can't create dir({:?})", parent));
        }
    }

    let root = BitMapBackend::gif(&file_path, (arg.image_size, arg.image_size), arg.interval)?
        .into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(100);

    let size = ((arg.image_size - 100) / 2) as f64;
    let chart = ChartBuilder::on(&lower)
        .margin(20)
        .build_cartesian_2d(-size..size, -size..size)?;
    let plotting_area = chart.plotting_area();

    let style = TextStyle::from(("sans-serif", 30).into_font()).color(&BLACK);

    for i in arg.times_from..=arg.times_to {
        root.fill(&WHITE)?;

        let gp = format!("{}:{}", arg.gp, i);

        let gen = Box::new(TimesGenerator::from_gp(&gp)?);
        let generator_info = gen.generator_info();
        let mut tile = create_tile(&arg, gen, &plotting_area)?;
        let tile_info = tile.tile_info();
        while let Some(result) = tile.draw_next() {
            result?;
        }

        upper.fill(&WHITE)?;
        upper.draw_text(&generator_info, &style, (20, 10))?;
        upper.draw_text(&tile_info, &style, (20, 40))?;
        root.present()
            .expect(&format!("Failed to output file({})", file_path));

        println!("times {} done", i);
    }

    let wait_num = (arg.wait_after + arg.interval - 1) / arg.interval;

    for _ in 0..wait_num {
        root.present()
            .expect(&format!("Failed to output file({})", file_path));
    }

    Ok(())
}

fn create_tile<'a>(
    arg: &AppArg,
    gen: Box<dyn Generator>,
    plotting_area: &'a DrawingArea<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) -> Result<Box<dyn Tile + 'a>, Box<dyn std::error::Error>> {
    if arg.tile == "spiral4" {
        let gen = SquareSpiral::from_tp(&arg.tp, gen, plotting_area)?;
        return Ok(Box::new(gen));
    }

    if arg.tile == "spiral6" {
        let gen = HexagonSpiral::from_tp(&arg.tp, gen, plotting_area)?;
        return Ok(Box::new(gen));
    }

    if arg.tile == "zigzag4" {
        let gen = SquareZigzag::from_tp(&arg.tp, gen, plotting_area)?;
        return Ok(Box::new(gen));
    }

    panic!()
}

#[derive(Parser, Debug)]
#[clap(
    name = "Gen-Tile-gif",
    author = "cuboktahedron",
    version = "v0.1.0",
    about = "Draw with generator and tile."
)]

struct AppArg {
    #[clap(short, long, default_value = "spiral4")]
    tile: String,

    #[clap(long, default_value = "0")]
    gp: String,

    #[clap(long, default_value = "0")]
    tp: String,

    #[clap(long, default_value = "1")]
    times_from: u32,

    #[clap(long, default_value = "100")]
    times_to: u32,

    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long, default_value = "800")]
    image_size: u32,

    #[clap(long, default_value = "100")]
    interval: u32,

    #[clap(long, default_value = "1000")]
    wait_after: u32,
}
