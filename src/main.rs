use std::io::Write;

use anyhow::Result;
use stdinix::stdinix;

fn main() -> Result<()> {
    let mut num = 0;

    stdinix(|buf| {
        num += 1;
        println!("{}\t{}", num, buf.trim());
        std::io::stdout().flush()
    })?;

    Ok(())
}
