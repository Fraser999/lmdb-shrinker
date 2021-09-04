use std::{fs, path::Path};

use clap::{crate_description, crate_version, App, Arg};
use lmdb::{Environment, EnvironmentFlags};

const ARG_NAME: &str = "file-path";

fn main() {
    let app = App::new("LMDB Shrinker")
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new(ARG_NAME)
                .required(true)
                .value_name("PATH")
                .about("Path to DB file"),
        );
    let matches = app.get_matches();
    let path = Path::new(
        matches
            .value_of(ARG_NAME)
            .expect("should have file-path arg"),
    );

    let size_before = fs::metadata(path)
        .unwrap_or_else(|error| panic!("failed to get metadata for {}: {}", path.display(), error))
        .len();

    let _env = Environment::new()
        .set_flags(EnvironmentFlags::WRITE_MAP | EnvironmentFlags::NO_SUB_DIR)
        .set_max_dbs(100)
        .set_map_size(1)
        .open(path)
        .unwrap_or_else(|error| panic!("failed to open {}: {}", path.display(), error));

    let size_after = fs::metadata(path)
        .unwrap_or_else(|error| panic!("failed to get metadata for {}: {}", path.display(), error))
        .len();

    if size_before > size_after {
        println!(
            "Reduced size of {} from {} to {} bytes.",
            path.display(),
            size_before,
            size_after
        );
    } else {
        println!(
            "Failed to reduce size of {} from {} bytes.",
            path.display(),
            size_before
        );
    }
}
