//! `errlog` is a simple API to create, or append an error log
//! at a specified file path. 
//! 
//! ## Objective
//! This crate is to only use the standard library and
//! not rely on 3rd-party dependencies while still offering
//! the high-level convience needed from this tool.

use std::error::Error;
use std::path::PathBuf;
use std::fs;

mod error;

/// You must specify the file name within the path. In it's current state,
/// only a single new directory can be created. If you are placing error logs
/// outside of the root of the project, it's recommended to use an absolute
/// file path.
pub fn errlog(path: &str, error: String) -> Result<(), Box<dyn Error>> {

    let path = create_path_from_str(path)?;

    check_or_make_directory(&path)?;

    check_or_make_log(&path)?;

    append_log(&path, error.as_str())?;
    Ok(())
}

fn create_path_from_str(text: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = PathBuf::from(text);
    if let Some(ext) = path.extension() {
        if ext != "log" {
            return Err(Box::new(error::BadExtensionError {message: String::from("must use \".log\" extension in file name")}));
        }
    } else {
        return Err(Box::new(error::BadExtensionError {message: String::from("must use \".log\" extension in file name")}));
    }

    Ok(path)
}

fn check_or_make_directory(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut dir_path = path.clone();
    dir_path.pop();

    if dir_path.is_dir() {
        return Ok(());
    }

    fs::create_dir(dir_path)?;
    Ok(())    
}

fn check_or_make_log(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let path = path.clone();
    if path.try_exists()? {
        Ok(())
    } else {
        fs::File::create(path)?;
        Ok(())
    }
}

fn append_log(file_path: &PathBuf, error: &str) -> Result<(), Box<dyn Error>> {

    // create timestamp
    // open file
    // append timestamp & error
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_path() {
        let path = create_path_from_str("./output/test.log").unwrap();
        assert_eq!("test", path.file_stem().unwrap());
        assert_eq!("log", path.extension().unwrap());
        assert_eq!("./output/test.log", path.to_str().unwrap().to_owned());
    }

    #[test]
    fn test_directory_checks() {
        let path = PathBuf::from("./test-data/test.log");
        let result = check_or_make_directory(&path);
        match result {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "Could not test for `./test-data` directory. Error: {e}"),
        }

        let mut path = PathBuf::from("./new-dir/test.log");
        let result = check_or_make_directory(&path);
        match result {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "Could not create `./new-dir/` directory. Error: {e}"),
        }

        path.pop();
        fs::remove_dir(&path).unwrap();

        let path = PathBuf::from("./test.log");
        let result = check_or_make_directory(&path);
        match result {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "Failed to check current directory. Error: {e}"),
        }
    }

    #[test]
    fn test_create_log() {
        let path = PathBuf::from("./test-data/new-file.txt");
        let result = check_or_make_log(&path);
        match result {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "Failed to create file `./test-data/new-file.txt`. Error: {e}"),
        }

        fs::remove_file(&path).unwrap();
    }
    // #[test]
    // fn errlog_test() {
    //     let result = errlog("path", String::from("error"));
    //     match result {
    //         Ok(_) => assert!(true),
    //         Err(_) => assert!(false),
    //     }
    //     assert!(false)
    // }
}
