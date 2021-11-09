use std::{collections::HashMap, error::Error};

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

    /// Update value for package.json's field `license`
    #[structopt(long)]
    update_license: Option<String>,

    /// Update value for package.json's field `private`
    #[structopt(long)]
    update_private: Option<bool>,

    /// Update value for package.json's field `peerDependencies`
    #[structopt(long, parse(try_from_str = parse_key_val))]
    update_peer_dependencies: Option<Vec<(String, String)>>,

    /// Update value for package.json's field `devDependencies`
    #[structopt(long, parse(try_from_str = parse_key_val))]
    update_dev_dependencies: Option<Vec<(String, String)>>,

    /// Update value for package.json's field `dependencies`
    #[structopt(long, parse(try_from_str = parse_key_val))]
    update_dependencies: Option<Vec<(String, String)>>,

    /// Remove value from package.json's field `peerDependencies`
    #[structopt(long)]
    remove_peer_dependencies: Option<Vec<String>>,

    /// Remove value from package.json's field `dependencies`
    #[structopt(long)]
    remove_dev_dependencies: Option<Vec<String>>,

    /// Remove value from package.json's field `dependencies`
    #[structopt(long)]
    remove_dependencies: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();
    let new_license = opt.update_license.as_deref();
    let new_peer_dependencies = opt.update_peer_dependencies.as_deref();
    let new_dev_dependencies = opt.update_dev_dependencies.as_deref();
    let new_dependencies = opt.update_dependencies.as_deref();

    let remove_peer_dependencies = opt.remove_peer_dependencies.as_deref();
    let remove_dev_dependencies = opt.remove_dev_dependencies.as_deref();
    let remove_dependencies = opt.remove_dependencies.as_deref();

    println!("Arguments: {:?}", opt);

    for entry in fs_walker::walk(opt.input_dir) {
        let dir_entry = entry?;
        let path_buf =  dir_entry.path();
        let file_name = dir_entry.file_name;
        if let Some(file_name) = file_name.to_str() {
            if file_name.contains("package.json") {
                let path = path_buf.as_path();
                let data = std::fs::read_to_string(path).expect("Unable to read file");
                let mut package_json: PackageJson = PackageJson::load(&data).expect(&format!("Unable to parse {}", path.to_str().unwrap()));

                update_single_field(&mut package_json.license, new_license);
                update_single_field(&mut package_json.private, opt.update_private);

                update_map_field(&mut package_json.peer_dependencies, new_peer_dependencies);
                update_map_field(&mut package_json.dev_dependencies, new_dev_dependencies);
                update_map_field(&mut package_json.dependencies, new_dependencies);

                remove_map_field(&mut package_json.peer_dependencies, remove_peer_dependencies);
                remove_map_field(&mut package_json.dev_dependencies, remove_dev_dependencies);
                remove_map_field(&mut package_json.dependencies, remove_dependencies);
                
                let destination = format!("{}{}", path.to_str().unwrap(), opt.suffix);
                std::fs::write(destination, PackageJson::save(&package_json).expect("Error during writing file to disk"))?;

                println!("{:?}", "DONE");
            }
        }
    }

    Ok(())
}

fn update_single_field<T>(maybe_field: &mut Option<T>, maybe_new_value: Option<T>) {
    if maybe_new_value.is_some() {
        *maybe_field = maybe_new_value;
    }
}

fn update_map_field<'a>(maybe_field: &mut Option<HashMap<&'a str, &'a str>>, maybe_new_value: Option<&'a [(String, String)]>) {
    if let Some(new_value) = maybe_new_value {
        maybe_field.as_mut().and_then(|field| {
            for (k, v) in new_value {
                let key: &str = k.as_ref();
                if field.contains_key(key) {
                    field.insert(key, v);
                }
            }
            Some(field)
        });
    }
}

fn remove_map_field<'a>(maybe_field: &mut Option<HashMap<&'a str, &'a str>>, maybe_new_value: Option<&'a [String]>) {
    if let Some(new_value) = maybe_new_value {
        maybe_field.as_mut().and_then(|field| {
            for k in new_value {
                let key: &str = k.as_ref();
                if field.contains_key(key) {
                    field.remove(key);
                }
            }
            Some(field)
        });
    }
}

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
