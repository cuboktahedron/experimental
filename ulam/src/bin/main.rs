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
use ulam::ulam::generator::prime1s::Prime1sGenerator;
use ulam::ulam::generator::prime3s::Prime3sGenerator;
use ulam::ulam::generator::prime7s::Prime7sGenerator;
use ulam::ulam::generator::primes::PrimesGenerator;
use ulam::ulam::generator::squares::SquareGenerator;
use ulam::ulam::generator::times::TimesGenerator;
use ulam::ulam::tile::hexagon_spiral::HexagonSpiral;
use ulam::ulam::tile::square_spiral::SquareSpiral;
use ulam::ulam::tile::square_zigzag::SquareZigzag;
use ulam::ulam::tile::tile::Tile;
use ulam::ulam::tile::tile::MARGIN;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let arg: AppArg = AppArg::parse();

  let style = TextStyle::from(("sans-serif", 30).into_font()).color(&BLACK);

  if !Path::new("output").is_dir() {
    create_dir("output")?;
  }
  let file_path = if let Some(ref file) = arg.output {
    format!("output/{}", file)
  } else {
    format!(
      "output/{}-{}-{}-{}.png",
      arg.generator,
      arg.tile,
      arg.image_size,
      arg.gp.replace(":", "_")
    )
  };

  let path = Path::new(&file_path);
  if let Some(ref parent) = path.parent() {
    if !parent.is_dir() {
      println!("{:?}", parent);
      create_dir_all(parent).expect(&format!("Can't create dir({:?})", parent));
    }
  }

  let root = BitMapBackend::new(&file_path, (arg.image_size, arg.image_size)).into_drawing_area();
  root.fill(&WHITE)?;

  let (upper, lower) = root.split_vertically(MARGIN as u32);

  let size = ((arg.image_size - MARGIN as u32) / 2) as f64;
  let chart = ChartBuilder::on(&lower)
    .margin_left(MARGIN as f64 * 0.75)
    .margin_right(MARGIN as f64 * 0.75)
    .margin_bottom(MARGIN as u32 / 2)
    .build_cartesian_2d(-size..size, -size..size)?;
  let plotting_area = chart.plotting_area();

  let gen = create_generator(&arg)?;
  upper.draw_text(&format!("{}", gen.generator_info()), &style, (20, 10))?;

  let mut tile = create_tile(&arg, gen, &plotting_area)?;
  upper.draw_text(&format!("{}", tile.tile_info()), &style, (20, 50))?;

  while let Some(result) = tile.draw_next() {
    result?;
  }

  root
    .present()
    .expect(&format!("Failed to output file({})", file_path));

  Ok(())
}

fn create_generator(arg: &AppArg) -> Result<Box<dyn Generator>, Box<dyn std::error::Error>> {
  if arg.generator == "primes" {
    let gen = PrimesGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  if arg.generator == "prime1s" {
    let gen = Prime1sGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  if arg.generator == "prime3s" {
    let gen = Prime3sGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  if arg.generator == "prime7s" {
    let gen = Prime7sGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  if arg.generator == "times" {
    let gen = TimesGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  if arg.generator == "squares" {
    let gen = SquareGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  panic!()
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
  name = "Gen-Tile",
  author = "cuboktahedron",
  version = "v0.1.0",
  about = "Draw with generator and tile."
)]

struct AppArg {
  #[clap(short, long, default_value = "primes")]
  generator: String,

  #[clap(short, long, default_value = "spiral4")]
  tile: String,

  #[clap(long, default_value = "1")]
  gp: String,

  #[clap(long, default_value = "0")]
  tp: String,

  #[clap(short, long)]
  output: Option<String>,

  #[clap(short, long, default_value = "800")]
  image_size: u32,
}
