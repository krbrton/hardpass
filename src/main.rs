use clap::Parser;

mod manager;
mod error;

fn main() -> Result<(), error::Error> {
    let pm = manager::PasswordManager::parse();
    let result = pm.generate()?;

    println!("{}", result);

    Ok(())
}
