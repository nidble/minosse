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

    /// New value for package.json's field `license`
    #[structopt(long)]
    field_license: Option<String>,

    /// New value for package.json's field `private`
    #[structopt(long)]
    field_private: Option<bool>,

    /// New value for package.json's field `peerDependencies`
    #[structopt(long, parse(try_from_str = parse_key_val))]
    field_peer_dependencies: Option<Vec<(String, String)>>,

    /// New value for package.json's field `devDependencies`
    #[structopt(long, parse(try_from_str = parse_key_val))]
    field_dev_dependencies: Option<Vec<(String, String)>>,

    /// New value for package.json's field `dependencies`
    #[structopt(long, parse(try_from_str = parse_key_val))]
    field_dependencies: Option<Vec<(String, String)>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();
    let license = opt.field_license.as_deref();
    let new_peer_dependencies = opt.field_peer_dependencies.as_deref();
    let new_dev_dependencies = opt.field_dev_dependencies.as_deref();
    let new_dependencies = opt.field_dependencies.as_deref();
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

                update_single_field(&mut package_json.license, license);
                update_single_field(&mut package_json.private, opt.field_private);

                update_map_field(&mut package_json.peer_dependencies, new_peer_dependencies);
                update_map_field(&mut package_json.dev_dependencies, new_dev_dependencies);
                update_map_field(&mut package_json.dependencies, new_dependencies);
                
                let destination = format!("{}{}", path.to_str().unwrap(), opt.suffix);
                std::fs::write(destination, PackageJson::save(&package_json).expect("Error during writing file to disk"))?;

                println!("{:?}", "DONE\n");
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
