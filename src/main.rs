use colour::{green_ln_bold, red_ln_bold, yellow_ln_bold};
use std::process;
mod helpers;
mod license;

use clap::Parser;
use license::LicenseContent;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    license_name: Option<String>,

    #[arg(short, long)]
    skip_prompt: Option<bool>,
}

fn main() {
    let licenses = license::Licenses::fetch_licenses();

    match licenses {
        Ok(licenses) => {
            let valid_license_names = &licenses.get_license_keys();

            let args: Args = Args::parse();
            let license_name = &args.license_name.unwrap_or(String::from("")).to_lowercase();
            let license_content: LicenseContent;

            if valid_license_names.contains(&license_name) {
                license_content = licenses.get_license_from_key(&license_name);
            } else {
                let license = helpers::select(&licenses.get_license_names());
                license_content = licenses.get_license_from_name(&license);
            }

            helpers::fill_content(&license_content, args.skip_prompt.unwrap_or(false));
        }
        Err(err) => {
            yellow_ln_bold!("\nErro Occured while getting Licenses.\n");
            red_ln_bold!("{}\n", err);
            green_ln_bold!("Tip: Check Your Internet Connection And try again\n");
            process::exit(1);
        }
    }
}
