use anyhow::Result;
use ember_plus_rs::connect;

fn main() -> Result<()> {
    env_logger::init();
    let addr = "127.0.0.1:9002".parse().unwrap();
    connect(addr);

    Ok(())
}
