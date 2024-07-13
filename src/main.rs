#![allow(non_snake_case)]

mod cli;

use std::fs;

use clap::Parser;
use cli::Cli;
use schema::{
    PalworldSave, PalworldSavePropertiesWorldSaveDataValueItemContainerSaveDataValue,
    PalworldSavePropertiesWorldSaveDataValueItemContainerSaveDataValueValueSlotsValueValue,
};
mod schema;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let f = fs::File::open(cli.json_path)?;

    let palworld_save: PalworldSave = serde_json::from_reader(f)?;

    println!("Deserialization succeeded");

    let itemContainers = palworld_save
        .properties
        .worldSaveData
        .value
        .itemContainerSaveData
        .value;
    println!("{}", itemContainers.len());

    Ok(())
}
