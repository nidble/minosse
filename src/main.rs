use std::error::Error;

use fs_walker;
use json::PackageJson;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "minosse", about = "Minosse usage...")]
struct Opt {
    /// Input dir
    #[structopt(parse(from_os_str))]
    input_dir: PathBuf,

    /// Output file: `file` or `file<suffix>` 
    #[structopt(default_value = "", long)]
    suffix: String,

    /// New value for Package.json field `license`
    #[structopt(long)]
    field_license: Option<String>,

    /// New value for Package.json field `private`
    #[structopt(long)]
    field_private: Option<bool>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();
    let license = opt.field_license.as_deref();
    println!("{:?}", opt);

    for entry in fs_walker::walk(opt.input_dir) {
        let dir_entry = entry?;
        let path =  dir_entry.path();
        let file_name = dir_entry.file_name;
        if let Some(file_name) = file_name.to_str() {
            if file_name.contains("package.json") {
                let fath = path.as_path();
                let data = std::fs::read_to_string(fath).expect("Unable to read file");
                let mut package_json: PackageJson = PackageJson::load(&data).expect(&format!("Unable to parse {}", fath.to_str().unwrap()));

                package_json.license = license;   
                package_json.private = opt.field_private;

                let destination = format!("{}{}", fath.to_str().unwrap(), opt.suffix);
                std::fs::write(destination, PackageJson::save(&package_json).expect("Error during writing file to disk"))?;

                println!("{:?}", "DONE\n");
            }
        }
    }

    Ok(())
}