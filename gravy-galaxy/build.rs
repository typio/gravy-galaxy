use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::{create_all, remove, CopyOptions};
use std::env;

fn main() -> Result<()> {
    // println!("cargo:rerun-if-changed=res/*");

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["res/"];

    let out_dir = std::path::Path::new(&env::var("CARGO_MANIFEST_DIR")?)
        .parent()
        .unwrap()
        .join("www/public/");
    remove(&out_dir)?;
    create_all(&out_dir, false)?;
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    let out_dir = std::path::Path::new(&env::var("CARGO_MANIFEST_DIR")?)
        .parent()
        .unwrap()
        .join("www_production/public/");
    remove(&out_dir)?;
    create_all(&out_dir, false)?;
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    // move root README to crate
    copy_items(&vec!["../README.md"], "./", &copy_options)?;

    Ok(())
}
