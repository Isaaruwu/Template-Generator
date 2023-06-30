use std::process::Command;
use std::{path, fs};

use super::data;

fn initialize_venv(project_path: &path::PathBuf) {
    log::info!("Creating virtual environment in {}", project_path.to_string_lossy());    

    let output = Command::new("python")
        .args(&["-m", "venv", "venv"])
        .current_dir(project_path)
        .output()
        .expect("Failed to create virtual environment");

    match output {
        Ok(_output) => {
            log::info!("Virtual environment was created!");
        }
        Err(err) => {
            log::error!("Failed to execute command: {}", err);
        }
    }
}

fn create_main_file(project_path: &path::PathBuf, project_type: &Option<&str>) {
    log::info!("Creating main.py..."); 

    let main_file_path = &project_path.join("main.py");
    fs::write(&main_file_path, "print('Hello World')").unwrap();   

    match project_type {
        Some("data") => data::create_data_template(&project_path),
        _ => {}
    }
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
            log::info!("requirements.txt was created! {}", output.status);
        }
        Err(err) => {
            log::error!("Failed to execute command: {}", err);
        }
    }
}

pub fn create_python_project(project_path: &path::PathBuf, project_type: &Option<&str>) {
    initialize_venv(&project_path);
    create_main_file(&project_path, &project_type);
    pip_freeze(&project_path);
}
