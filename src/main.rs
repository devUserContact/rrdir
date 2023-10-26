use rmdir::rmdir_w_keyword;

fn main() {
    rmdir_w_keyword();
}

pub mod rmdir {
    use clap::{Arg, ArgAction, Command};
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
        let dir_target = Command::new("rrdir")
            .author("devUserContact")
            .version("")
            .about("A rust CLI for removing directories that match a given keyword")
            .arg(
                Arg::new("Directory")
                    .short('d')
                    .long("directory")
                    .action(ArgAction::Set)
                    .required(true),
            )
            .get_matches();

        let mut path_list = PathList { paths: Vec::new() };
        if let Some(dir_tar) = dir_target.get_one::<String>("Directory") {
            println!("\nSearching File System. .\n");
            for entry in WalkDir::new("../_test").into_iter().filter_map(|e| e.ok()) {
                if metadata(entry.path()).unwrap().is_dir() {
                    let dir_name = entry.path().file_name();
                    if dir_name == Some(OsStr::new(dir_tar)) {
                        let path_target_dir = entry.path().display().to_string();
                        let keyword_freq = count_frequency(&path_target_dir, dir_tar);
                        if keyword_freq == 1 {
                            println!("{:?}", path_target_dir);
                            path_list.push_string(path_target_dir.clone());
                        }
                    }
                }
            }
            ask_delete(&path_list);
        } else {
            println!("a directory was not provided");
            return;
        }
    }

    pub fn ask_delete(path_list: &PathList) {
        let number_of_paths: usize = path_list.paths.len();
        let delete_promt = format!(
            "\nFound {:?} directories that contained the keyword: node_modules. Confirm deletion?",
            number_of_paths
        );
        let answer = Confirm::new(&delete_promt)
            .with_default(false)
            .with_help_message("Deletion of directories is permanent")
            .prompt();
        fn del_dirs(path_list: &PathList) {
            for dir in path_list.paths.clone() {
                println!("deleting directory: {:?}", dir);
                let _ = remove_dir_all(dir);
            }
            println!("\ndirectories deleted\n");
            return
        }
        match answer {
            Ok(true) => del_dirs(path_list),
            Ok(false) => println!("action aborted"),
            Err(_) => println!("There was an Error"),
        }
    }

    fn count_frequency(text: &str, target: &str) -> usize {
        let mut freq_map: HashMap<&str, usize> = HashMap::new();
        for word in text.split("/") {
            *freq_map.entry(word).or_insert(0) += 1;
        }
        freq_map.get(target).unwrap_or(&0).clone()
    }
}
