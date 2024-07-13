use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Path to the palword save file in json format. Refer to `https://github.com/cheahjs/palworld-save-tools` for the conversion
    pub json_path: PathBuf,
    /// Nickname of the user to put the pal_sphere into. This is the name you created when you first entered the game
    #[arg(long, short)]
    pub nickname: String,
    /// Number of palspheres to put in the user
    #[arg(long, short)]
    pub count: i32,
}
