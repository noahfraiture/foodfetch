use anyhow::Result;
use clap::{Parser, ValueEnum};

/// Neofetch-like tool to get recipes
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
    #[arg(short, long, default_value = "all", value_delimiter = ',')]
    pub infos: Vec<Info>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Info {
    /// Display all available informations
    All,

    /// Give the links
    Links,

    /// Give the instructions to cook the meal
    Instructions,
}

pub fn args() -> Result<Cli> {
    let args = Cli::parse();
    Ok(args)
}
