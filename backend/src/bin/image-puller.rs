use log::info;
use std::{env, error::Error};
use tokio;

use playground::{Context, kubernetes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::error!("Hello!");
    Ok(())
}
