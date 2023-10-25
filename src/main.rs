use rmdir::rmdir_w_keyword;

fn main() {
    rmdir_w_keyword();
}

pub mod rmdir {
    use inquire::Confirm;
    use std::collections::HashMap;
    use std::ffi::OsStr;
    use std::fs::metadata;
    use std::fs::remove_dir_all;
    use walkdir::WalkDir;

    pub struct PathList {
        pub paths: Vec<String>,
    }
    impl PathList {
        fn push_string(&mut self, path_to_remove: String) {
            self.paths.push(path_to_remove);
        }
    }

    pub fn rmdir_w_keyword() {
        println!("\nSearching File System. .\n");
        let mut path_list = PathList { paths: Vec::new() };
        for entry in WalkDir::new("../_test").into_iter().filter_map(|e| e.ok()) {
            if metadata(entry.path()).unwrap().is_dir() {
                let dir_name = entry.path().file_name();
                if dir_name == Some(OsStr::new("node_modules")) {
                    let target_dir_path = entry.path().display().to_string();
                    let keyword_freq = count_frequency(&target_dir_path, "node_modules");
                    if keyword_freq == 1 {
                        path_list.push_string(entry.path().display().to_string());
                    }
                }
            }
        }
        ask_delete(&path_list);
    }

    pub fn ask_delete(path_list: &PathList) {
        let number_of_paths: usize = path_list.paths.len();
        let delete_promt = format!(
            "found {:?} directories that contained the keyword: node_modules. Confirm deletion?",
            number_of_paths
        );
        let answer = Confirm::new(&delete_promt)
            .with_default(false)
            .with_help_message("Deletion of directories is permanent")
            .prompt();
        match answer {
            Ok(true) => println!("deleting dirs"),
            Ok(false) => println!("action aborted"),
            Err(_) => println!("There was an Error"),
        }
        //let delete_dirs = remove_dir_all(entry.path().display().to_string());
    }

    fn count_frequency(text: &str, target: &str) -> usize {
        let mut freq_map: HashMap<&str, usize> = HashMap::new();
        for word in text.split("/") {
            *freq_map.entry(word).or_insert(0) += 1;
        }
        freq_map.get(target).unwrap_or(&0).clone()
    }
}
