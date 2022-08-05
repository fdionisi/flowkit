use std::path::PathBuf;

use libflow::{io::FlowIo, FlowError};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    author = clap::crate_authors!(),
    about = clap::crate_description!(),
    version = clap::crate_version!(),
    setting = clap::AppSettings::ColoredHelp,
    setting = clap::AppSettings::DisableHelpSubcommand,
    setting = clap::AppSettings::DeriveDisplayOrder,
)]
struct Args {
    path: PathBuf,
}

#[async_std::main]
async fn main() -> Result<(), FlowError> {
    let clp = Args::clap();
    let matches = clp.get_matches();
    let args = Args::from_clap(&matches);

    let mut flow_io = FlowIo::new(args.path).await?;
    // dbg!(&flow_io);

    // let _ = flow_io.header().await?;
    // // dbg!(&header);
    //
    // let _ = flow_io.text().await?;
    // // dbg!(&text);
    //
    // let _ = flow_io.analysis().await?;
    // dbg!(&analysis);

    let _ = flow_io.data().await?;
    // dbg!(&data.parse_float()?);

    Ok(())
}
