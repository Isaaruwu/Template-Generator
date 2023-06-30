use std::process::Command;
use std::{path, fs};

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
            log::info!("requirements.txt was created! {}", output.status);
        }
        Err(err) => {
            log::error!("Failed to execute command: {}", err);
        }
    }
}

pub fn create_python_project(project_path: &path::PathBuf) {
    initialize_venv(&project_path);
    create_main_file(&project_path);
    pip_freeze(&project_path);
}
