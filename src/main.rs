use proj::run;
use std::{env, io};

fn main() -> io::Result<()> {
    run(env::args().collect());
    Ok(())
}
