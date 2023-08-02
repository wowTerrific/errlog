//! `errlog` is a simple API to create, or append an error log
//! at a specified file path. 
//! 
//! ## Objective
//! This crate is to only use the standard library and
//! not rely on 3rd-party dependencies while still offering
//! the high-level convience needed from this tool.

use std::error::Error;
use std::path::{PathBuf, Path};
use std::fs;
use std::time::SystemTime;

mod error;

/// You must specify the file name within the path. In it's current state,
/// only a single new directory can be created. If you are placing error logs
/// outside of the root of the project, it's recommended to use an absolute
/// file path. Timestamps on error log are in relation to `UNIX_EPOCH`. This is
/// a change for the future but will take *time* to implement. Get it?
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

fn check_or_make_directory(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut dir_path = path.to_path_buf();
    dir_path.pop();

    if dir_path.is_dir() {
        return Ok(());
    }

    fs::create_dir(dir_path)?;
    Ok(())    
}

fn check_or_make_log(path: &Path) -> Result<(), Box<dyn Error>> {
    let path = path.to_path_buf();
    if path.try_exists()? {
        Ok(())
    } else {
        fs::File::create(path)?;
        Ok(())
    }
}

fn append_log(file_path: &PathBuf, error: &str) -> Result<(), Box<dyn Error>> {

    let date_in_sec = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

    let current_log = fs::read_to_string(file_path)?;

    let updated_log = format!("{}\n{} - {}\n", current_log, date_in_sec, error);

    fs::write(file_path, updated_log)?;
    Ok(())
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
        if let Err(e) =  check_or_make_directory(&path) {
            assert!(false, "Could not test for `./test-data` directory. Error: {e}");
        }

        let mut path = PathBuf::from("./new-dir/test.log");
        if let Err(e) = check_or_make_directory(&path) {
            assert!(false, "Could not create `./new-dir/` directory. Error: {e}");
        }

        // cleanup
        path.pop();
        fs::remove_dir(&path).unwrap();

        let path = PathBuf::from("./test.log");
        if let Err(e) =  check_or_make_directory(&path) {
            assert!(false, "Failed to check current directory. Error: {e}");
        }

        assert!(true);
    }


    #[test]
    fn test_create_log() {
        let path = PathBuf::from("./test-data/new-file.txt");
        if let Err(e) = check_or_make_log(&path) {
            assert!(false, "Failed to create file `./test-data/new-file.txt`. Error: {e}");
        }

        // clean up
        fs::remove_file(&path).unwrap();

        assert!(true);
    }


    #[test]
    fn test_append() {
        let path = PathBuf::from("./test-data/test.log");
        if let Err(e) =  append_log(&path, "test error") {
            assert!(false, "Could not write contents to `./test-data/test.log`. Error: {e}");
        }

        let bad_path = PathBuf::from("./test-data/does-not-exist.log");
        if let Ok(_) =  append_log(&bad_path, "Something") {
            assert!(false, "Should not be able to write contents to `./test-data/does-not-exist.log`.");
        }

        // clean up
        fs::write("./test-data/test.log", "").unwrap();

        assert!(true);
    }


    #[test]
    fn errlog_success() {
        if let Err(e) =  errlog("./test-data/errlog-unit-test.log", String::from("error log was successful")) {
            assert!(false, "Something went horribly wrong. Error: {e}");
        }

        if let Err(e) = errlog("./new-folder/errlog-unit-test.log", String::from("error log was successful")) {
            assert!(false, "Something went horribly wrong. Error: {e}");
        }

        // clean up
        fs::remove_file("./new-folder/errlog-unit-test.log").unwrap();
        fs::remove_dir("./new-folder").unwrap();

        assert!(true);
    }


    #[test]
    fn errlog_fail_bad_path() {
        if let Ok(_) = errlog("./no-folder/abcd/errlog-unit-test.log", String::from("error log should fail")) {
            assert!(false, "`./no-folder/abcd/errlog-unit-test.log` should have failed as a bad path.");
        }

        if let Ok(_) = errlog("./test-data/errlog-unit-test", String::from("error log should fail")) {
            assert!(false, "not specifying a `.log` file should fail.");
        }

        assert!(true);
    }
}
