use std::path::Path;

use jwalk::WalkDirGeneric;

pub fn walk<P>(root: P) -> WalkDirGeneric<(usize, bool)>  where P: AsRef<Path> {
    let walk_dir = WalkDirGeneric::<(usize,bool)>::new(root)
    .process_read_dir(|_depth, _path, _read_dir_state, children| {
        children.iter_mut().for_each(|dir_entry_result| {
            if let Ok(dir_entry) = dir_entry_result {
                let file_name = dir_entry.file_name.to_str().unwrap();
                if file_name.contains("node_modules") || file_name.contains(".git") {
                    dir_entry.read_children_path = None;
                }

            }
        });
    });

    walk_dir
}

