
use chrono::prelude::*;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use env_logger;
use human_panic::setup_panic;
use serde_json::to_writer_pretty;

use reading_plan::{error::Result, read_pages};

fn main() -> Result<()> {
    setup_panic!();
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let start_date = args
        .start_date
        .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
        .transpose()?
        .unwrap_or_else(|| Local::now().naive_local().date());
    let end_date = NaiveDate::parse_from_str(&args.end_date, "%Y-%m-%d")?;

    let pages = read_pages(
        start_date,
        args.start_page,
        end_date,
        args.end_page
    ).collect::<Vec<_>>();

    to_writer_pretty(std::io::stdout(), &pages)?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,

    /// The start date of the reading plan. Defaults to today.
    #[arg(short = 'd', long)]
    start_date: Option<String>,

    /// The end date of the reading plan.
    #[arg(short = 'D', long)]
    end_date: String,

    /// The start page of the reading plan. Defaults to 0.
    #[arg(short = 'p', long, default_value = "0")]
    start_page: u32,

    /// The end page of the reading plan.
    #[arg(short = 'P', long)]
    end_page: u32,
}
