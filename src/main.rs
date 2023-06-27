extern crate simple_logger;

use simple_logger::SimpleLogger;

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

    let output = Command::new("cmd")
        .arg("/C")
        .arg(format!("pip freeze > {}\\requirements.txt", path_string))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        log::info!("Requirements.txt was created!");    
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("Error occurred while creating requirements.txt: {}", stderr);    
    }
}

fn create_python_project(project_name: &str) {
    let current_dir = env::current_dir().unwrap();
    let project_path = path::Path::new(&current_dir).join(project_name);
    
    if project_path.exists() {
        log::warn!("Project already exists, exiting...");
        return;
    }

    create_project(&project_path);
    initialize_venv(&project_path);
    create_main_file(&project_path);
    pip_freeze(&project_path);

}
fn main() {
    SimpleLogger::new().init().unwrap();
    
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let project_name = &args[2];

    if command == "create" {
        create_python_project(project_name);
    }
    else {
        log::warn!("Invalid command, please use 'create'");    
        println!("Command not found");
    }
}