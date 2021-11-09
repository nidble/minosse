use std::error::Error;

use fs_walker;
use json::PackageJson;

use std::path::PathBuf;
use structopt::StructOpt;

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s.find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;

    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Debug, StructOpt)]
#[structopt(name = "minosse", about = "Minosse usage...")]
struct Opt {
    /// Input dir
    #[structopt(parse(from_os_str))]
    input_dir: PathBuf,

    /// Output file: `file` or `file<suffix>`
    #[structopt(default_value = "", long)]
    suffix: String,

    /// New value for package.json field `license`
    #[structopt(long)]
    field_license: Option<String>,

    /// New value for package.json field `private`
    #[structopt(long)]
    field_private: Option<bool>,

    /// New value for package.json field `peerDependencies`
    #[structopt(long, parse(try_from_str = parse_key_val))]
    field_peer_dependencies: Option<Vec<(String, String)>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();
    let license = opt.field_license.as_deref();
    let new_peer_dependencies = opt.field_peer_dependencies.as_deref();
    println!("Arguments: {:?}", opt);

    for entry in fs_walker::walk(opt.input_dir) {
        let dir_entry = entry?;
        let path =  dir_entry.path();
        let file_name = dir_entry.file_name;
        if let Some(file_name) = file_name.to_str() {
            if file_name.contains("package.json") {
                let fath = path.as_path();
                let data = std::fs::read_to_string(fath).expect("Unable to read file");
                let mut package_json: PackageJson = PackageJson::load(&data).expect(&format!("Unable to parse {}", fath.to_str().unwrap()));

                if license.is_some() {
                    package_json.license = license;
                }

                if opt.field_private.is_some() {
                    package_json.private = opt.field_private;
                }

                if let Some(new_deps) = new_peer_dependencies {
                    package_json.peer_dependencies.as_mut().and_then(|old_deps| {
                        for (k, v) in new_deps {
                            let key: &str = k.as_ref();
                            if old_deps.contains_key(key) {
                                old_deps.insert(key, v);
                            }
                        }
                        Some(old_deps)
                    });
                }
                
                let destination = format!("{}{}", fath.to_str().unwrap(), opt.suffix);
                std::fs::write(destination, PackageJson::save(&package_json).expect("Error during writing file to disk"))?;

                println!("{:?}", "DONE\n");
            }
        }
    }

    Ok(())
}
