//! `errlog` is a simple API to create, or append an error log
//! at a specified file path. 
//! 
//! ## Objective
//! This crate is to only use the standard library and
//! not rely on 3rd-party dependencies while still offering
//! the high-level convience needed from this tool.

use std::path::{PathBuf, Path};
use std::fs;
use std::thread::Thread;
use std::time::SystemTime;

mod error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// You must specify the file name within the path. In it's current state,
/// only a single new directory can be created. If you are placing error logs
/// outside of the root of the project, it's recommended to use an absolute
/// file path. Timestamps on error log are in relation to `UNIX_EPOCH`. This is
/// a change for the future but will take *time* to implement. Get it?
pub fn errlog(path: &str, error: String) -> Result<()> {

    let path = create_path_from_str(path)?;

    check_or_make_directory(&path)?;

    check_or_make_log(&path)?;

    append_log(&path, error.as_str())?;
    Ok(())
}

fn create_path_from_str(text: &str) -> Result<PathBuf> {
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

fn check_or_make_directory(path: &Path) -> Result<()> {
    let mut dir_path = path.to_path_buf();
    dir_path.pop();

    if dir_path.is_dir() {
        return Ok(());
    }

    fs::create_dir(dir_path)?;
    Ok(())    
}

fn check_or_make_log(path: &Path) -> Result<()> {
    let path = path.to_path_buf();
    if path.try_exists()? {
        Ok(())
    } else {
        fs::File::create(path)?;
        Ok(())
    }
}

fn append_log(file_path: &PathBuf, error: &str) -> Result<()> {

    // TODO: parse into MM/DD/YYYY HH:MM:SS
    // let date_in_sec = SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)?
    //         .as_secs();

    let date_time = get_date()?;

    let current_log = fs::read_to_string(file_path)?;

    let updated_log = format!("{}\n{} - {}\n", current_log, date_time, error);

    fs::write(file_path, updated_log)?;
    Ok(())
}


// TODO: This probably needs to be it's own library - or just use chrono?
// This will output time as UTC, should be marked as such in above log
fn get_date() -> Result<String> {
    let date_in_sec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    const DAY_MONTH: [u64; 11] = [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const SECONDS_IN_YEAR: u64 = 31_536_000;
    const SECONDS_IN_DAY: u64 = 86_400;
    const THREE_YEARS_IN_DAYS: u64 = 365 * 3;

    let num_days = date_in_sec / SECONDS_IN_DAY;

    let cycles = (num_days - (365+366)) / (THREE_YEARS_IN_DAYS + 366);
    let remainder_years = ((num_days - (365+366)) % (THREE_YEARS_IN_DAYS + 366)) / 365;
    let year = (cycles * 4) + remainder_years + 1972;
    let remainder_days =  (num_days % (cycles * (THREE_YEARS_IN_DAYS + 366))) % 365;
    let mut month: u64 = 12;
    let mut day: u64 = 31;
    for (i, val) in DAY_MONTH.iter().enumerate() {
        if remainder_days <= *val {
            month = i as u64;   // TODO!
            day = remainder_days % val;
            break;
        }
    }

    let hours = ( date_in_sec % (60 * 60 * 24) ) / ( 60 * 60 );
    let minutes = ( date_in_sec % (60 * 60) ) / 60;
    let seconds = date_in_sec % 60;

    // TODO: Leap years????
    // let years: u64 = (date_in_sec / SECONDS_IN_YEAR) + 1970;
    // let months: u64 = (date_in_sec % SECONDS_IN_YEAR) / (SECONDS_IN_DAY * 30);
    
    // let date_format = format!("Years: {}\nMonths: {}", years, months);
    let date_format = format!("{}/{}/{} - {}:{}:{}", month, day, year, hours, minutes, seconds);
    // let date_format = format!("{}:{}:{}", hours, minutes, seconds);

    Ok(date_format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_date_test() {
        let date = get_date().unwrap();
        println!("{date}");
        assert!(false);
    }

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
