use std::io::Read;

use miette::IntoDiagnostic;

#[derive(kfl::Decode, Debug)]
#[allow(dead_code)]
struct Plugin {
    #[kfl(argument)]
    name: String,
    #[kfl(property)]
    url: String,
    #[kfl(child, unwrap(argument))]
    version: String,
}

#[derive(kfl::Decode, Debug)]
#[allow(dead_code)]
struct Config {
    #[kfl(child, unwrap(argument))]
    version: String,
    #[kfl(children)]
    plugins: Vec<Plugin>,
}

fn main() -> miette::Result<()> {
    let mut buf = String::new();
    println!("Please type KDL document, press Return, Ctrl+D to finish");
    std::io::stdin().read_to_string(&mut buf).into_diagnostic()?;
    let cfg: Config = kfl::parse("<stdin>", &buf)?;
    println!("{:#?}", cfg);
    Ok(())
}