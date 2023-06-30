use simple_logger::SimpleLogger;
use std::{env, path, fs};
use clap::{App, Arg};

mod python;
use python::python::create_python_project;

fn create_directory(project_path: &path::PathBuf) {
    if project_path.exists() {
        log::error!("Project already exists, exiting...");
        return;
    }

    log::info!("Creating directory...");    
    fs::create_dir(&project_path).unwrap();
}

fn create_project(language: &str, project_type: Option<&str>, project_name: &str, path: Option<&str>) {
    let current_dir = env::current_dir().unwrap();
    let mut project_path = path::Path::new(&current_dir).join(project_name);

    if let Some(path) = path {
        let path = path::Path::new(path);
        if path.exists() {
            project_path = path.join(project_name);
        } else {
            log::error!("Path does not exist, exiting...");
            return;
        }
    }

    create_directory(&project_path);

    match language {
        "py" => create_python_project(&project_path, &project_type),
        _ => log::warn!("Invalid language, please use 'py'"),
    }
}
fn main() {
    SimpleLogger::new().init().unwrap();
    
    let project_types = ["default", "data"];
    let languages = ["py"];

    let matches = App::new("Latest")
        .arg(Arg::with_name("language")
            .help("The programming language")
            .possible_values(&languages)
            .required(true)
            .index(1))
        .arg(Arg::with_name("project_name")
            .help("The name of the project")
            .required(true)
            .index(2))
        .arg(Arg::with_name("project_type")
            .help("The type of project")
            .possible_values(&project_types)
            .required(false)
            .index(3))
        .arg(Arg::with_name("path")
            .help("The path to use (optional)")
            .index(4))
        .get_matches();

    let language = matches.value_of("language").unwrap();
    let project_type = matches.value_of("project_type");
    let project_name = matches.value_of("project_name").unwrap();
    let path = matches.value_of("path");

    create_project(language, project_type, project_name, path);
    
}