use simple_logger::SimpleLogger;
use clap::{App, Arg};


use std::{env, path, fs, process::Command};

fn create_project(project_path: &path::PathBuf) {
    log::info!("Creating directory...");    
    fs::create_dir(&project_path).unwrap();
}

fn initialize_venv(project_path: &path::PathBuf) {
    log::info!("Creating virtual environment in {}", project_path.to_string_lossy());    

    let output = Command::new("python")
        .args(&["-m", "venv", "venv"])
        .current_dir(project_path)
        .output()
        .expect("Failed to create virtual environment");

    if output.status.success() {
        log::info!("Virtual env was created!");    
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("Error occurred while creating venv: {}", stderr);    
    }
}

fn create_main_file(project_path: &path::PathBuf) {
    log::info!("Creating main.py...");    
    let main_file_path = path::Path::new(&project_path).join("main.py");
    fs::write(&main_file_path, "print('Hello World')").unwrap();
}

fn pip_freeze(project_path: &path::PathBuf) {
    let path_string = project_path.to_string_lossy().into_owned();

    log::info!("Creating requirements.txt in {}", path_string);    

    let output = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C")
            .arg("pip freeze")
            .stdout(std::fs::File::create(project_path.join("requirements.txt")).unwrap())
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("pip freeze > {}/requirements.txt", path_string))
            .output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                log::info!("requirements.txt was created!");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::warn!("Error occurred while creating requirements.txt: {}", stderr);
            }
        }
        Err(err) => {
            log::error!("Failed to execute command: {}", err);
        }
    }
}

fn create_python_project(project_name: &str, path: Option<&str>) {
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

    create_project(&project_path);
    initialize_venv(&project_path);
    create_main_file(&project_path);
    pip_freeze(&project_path);

}
fn main() {
    SimpleLogger::new().init().unwrap();
    
    let matches = App::new("My CLI App")
        .arg(Arg::with_name("command")
            .help("The command to execute")
            .required(true)
            .index(1))
        .arg(Arg::with_name("project_name")
            .help("The name of the project")
            .required(true)
            .index(2))
        .arg(Arg::with_name("path")
            .help("The path to the project")
            .required(false)
            .index(3))
        .get_matches();

    let command = matches.value_of("command").unwrap();
    let project_name = matches.value_of("project_name").unwrap();
    let path = matches.value_of("path");

    if command == "create" {
        create_python_project(project_name, path);
    } else {
        log::warn!("Invalid command, please use 'create'");
        println!("Command not found");
    }
}