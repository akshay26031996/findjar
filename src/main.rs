use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::SearcherBuilder;

use jwalk::WalkDir;
use std::ffi::OsStr;

use clap::{App, Arg};

fn main() {
    // for cli
    let matches = App::new("FindJar")
        .version("0.1.0")
        .author("Akshay Ghiya <akshayghiya123@gmail.com>")
        .about("Find jars that have the give class in a directory")
        .arg(
            Arg::with_name("path")
                .required(true)
                .takes_value(true)
                .help("Path in which to look for the JARs"),
        )
        .arg(
            Arg::with_name("classname")
                .required(true)
                .takes_value(true)
                .help("Fully qualified classname"),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let classname = matches.value_of("classname").unwrap();

    let jar_extension = OsStr::new("jar");
    // let pattern = String::from("org.apache.commons.lang3.ArrayUtils");
    let pattern = String::from(classname);
    let matcher = match RegexMatcher::new_line_matcher(&pattern) {
        Err(_) => {
            panic!("Cannot create RegexMatcher for \"{}\" pattern", pattern);
        }
        Ok(matcher) => matcher,
    };
    let mut searcher = SearcherBuilder::new().line_number(false).build();
    for entry in WalkDir::new(path).sort(true) {
        match entry {
            Ok(a) => match a.path().extension() {
                None => {}
                Some(ext) => {
                    if ext == jar_extension {
                        // println!("{}", a.path().display());
                        let result =
                            searcher.search_path(&matcher, a.path(), UTF8(|_lnum, _line| Ok(true)));
                        match result {
                            Err(_) => {
                                eprintln!("{}", a.path().display());
                            }
                            Ok(_) => {}
                        }
                    }
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
