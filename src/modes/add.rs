use {
  colorful::Colorful,
  crate::{CONFIG, run_rebuild},
  std::{env::var, fs::write, process::exit},
};

pub fn add_package(mut file: Vec<String>, package: String, rebuild: bool) {
  if let Some(beginning) = file
    .iter()
    .position(|x| x.trim().contains("# SNOW BEGIN"))
    && let Some(end) = file
    .iter()
    .position(|x| x.trim().contains("# SNOW END"))
  {
    let whitespace = file[beginning]
      .chars()
      .take_while(|x| x.is_whitespace())
      .collect::<String>();

    if file[beginning..end].iter().any(|x| x.trim() == package.trim()) {
      eprintln!("{} Package already installed, not adding.", "✗".red());
      exit(1);
    }

    file.insert(beginning + 1, whitespace + &package);
    file[beginning..end].sort();
  }

  write(
    {
      CONFIG.get("path").map_or_else(
        || format!("{}/nix-config/home/default.nix", var("HOME").unwrap()),
        |path| path.replace('~', &var("HOME").unwrap()),
      )
    },
    file.join("\n"),
  ).unwrap();

  println!("{} Added {package} to your Nix packages.", "✓".green());

  if rebuild {
    run_rebuild();
  }
}