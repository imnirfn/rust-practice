// use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: tricoder <target.com")]
    CliUsage,
}

fn main() {
    println!("Hello, world!");

}

