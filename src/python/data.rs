use std::{path, fs};

pub fn create_data_template(project_path: &path::PathBuf) {
    log::info!("Creating jupyter notebook...");    

    let main_file_path = &project_path.join("main.py");
    fs::write(&main_file_path, "import pandas as pd\n\nprint(pd.__version__)").unwrap();

    let ipynb_file_path_ = &project_path.join("main.ipynb");
    fs::write(&ipynb_file_path_, "").unwrap();

    let directory = &project_path.join("data");
    fs::create_dir(&directory).unwrap();

    let csv = directory.join("data.csv");
    fs::File::create(&csv).unwrap();
}