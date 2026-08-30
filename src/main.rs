use crate::site_parser::SiteParser;
use crate::site_renderer::SiteRenderer;
use clap::Parser;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::exit;

mod site_parser;
mod site;
mod front_matter;
mod dated_entry;
mod site_renderer;
mod fs_utils;
mod split_around;
mod highlight_code_blocks;
mod liquid_xml_escape;
mod liquid_date_to_xml_schema;
mod time_utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    root_dir: PathBuf,
    #[arg(long)]
    skip_syntax_highlighting: bool,
}

fn main() {
    let args = Args::parse();
    let root_dir = PathBuf::from(args.root_dir);
    assert!(root_dir.is_dir());
    let output_dir = root_dir.join("out");
    if !output_dir.exists() {
        match create_dir(&output_dir) {
            Ok(_) => {
                println!("Created output directory at {}", output_dir.display())
            }
            Err(e) => {
                println!("Error creating output directory at {}: {}", output_dir.display(), e);
                exit(1);
            }
        }
    }

    let site_parser = SiteParser::new(root_dir.clone(), args.skip_syntax_highlighting);
    let site = site_parser.parse();

    println!("\nValidating(TODO)");

    print!("\nRendering!\n");
    SiteRenderer::new().render(site, &root_dir, &output_dir)
}

