#![warn(clippy::pedantic)]
use std::{env::args_os, error::Error, process::Command};

use crate::primitives::Package;

type Res<T> = Result<T, Box<dyn Error>>;

mod colors;
mod primitives;

const VALUE_REQUIRED: &str = "\
Please specify the name of a package to search.
Usage: nil-query <package_name>";

fn main() -> Res<()> {
    let package = args_os().nth(1).ok_or(VALUE_REQUIRED)?;
    let package = package.to_string_lossy();
    let search = search_pkg(&package)?;
    let mut search = search.iter().take(100).collect::<Vec<_>>();

    search.sort_by_key(|p| {
        (p.name != package, !p.name.contains(&*package), p.name.len())
    });

    for package in search {
        println!("{package}");
    }
    Ok(())
}

fn search_pkg(package: &str) -> Res<Vec<Package>> {
    let query = cmd_output("xbps-query", &["-Rs", package])?;
    query.trim().lines().map(Package::new).collect()
}

#[inline]
fn cmd_output(cmd: &str, args: &[&str]) -> Res<String> {
    Ok(String::from_utf8(Command::new(cmd).args(args).output()?.stdout)?)
}
