use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "Foodfetch")]
#[command(author = "noahcode")]
#[command(version)]
#[command(help_template = "
{name} - {about}

Author: {author}
Version: {version}

{usage-heading} {usage}
{all-args} {tab}")]
pub struct Cli {
    /// Keyword to use in the search
    #[command()]
    pub keyword: Option<String>,

    /// The infos you want to display
    #[arg(short, long, default_value = "all")]
    pub infos: Infos,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Infos {
    /// Display all available informations
    All,

    /// Ignore the links
    Reduced,
}

pub fn args() -> Result<Cli> {
    let args = Cli::parse();
    Ok(args)
}
