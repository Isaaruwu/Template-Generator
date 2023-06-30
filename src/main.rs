use simple_logger::SimpleLogger;
use std::{env, path, fs};
use clap::{App, Arg};
use std::io::{Write};
use git2::Repository;


mod python;
use python::python::create_python_project;

fn create_directory(project_path: &path::PathBuf) {
    log::info!("Creating directory...");    
    fs::create_dir(&project_path).unwrap();
}

fn init_gitrepo(project_path: &path::PathBuf, language: &str) {
    log::info!("Initializing Git repository...");
    Repository::init(&project_path).unwrap();

    let temp = env::current_exe().unwrap();
    let current_dir = temp.ancestors().nth(3).unwrap();
    let template_name = &format!("{}.gitignore", language);
    let src_gitignore_path = current_dir
        .join("src")
        .join("ignore_templates")
        .join(template_name);

    let contents = fs::read_to_string(&src_gitignore_path).unwrap();

    let gitignore = project_path.join(".gitignore");
    let mut dest = fs::File::create(&gitignore).unwrap();

    dest.write_all(contents.as_bytes()).unwrap();

}

fn create_project(language: &str, project_type: Option<&str>, project_name: &str, path: Option<&str>, init_git: bool) {
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

    if project_path.exists() {
        log::error!("Project already exists, exiting...");
        return;
    }

    create_directory(&project_path);

    match language {
        "py" => create_python_project(&project_path, &project_type),
        _ => log::warn!("Invalid language, please use 'py'"),
    }

    if init_git {init_gitrepo(&project_path, language);}
    
    log::info!("Done!");
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
        .arg(Arg::with_name("git")
            .short("g")
            .long("git")
            .takes_value(false)
            .help("To initialize a Git repository"))
        .get_matches();
        

    let language = matches.value_of("language").unwrap();
    let project_type = matches.value_of("project_type");
    let project_name = matches.value_of("project_name").unwrap();
    let path: Option<&str> = matches.value_of("path");
    let init_git: bool = matches.is_present("git");

    create_project(language, project_type, project_name, path, init_git);
}