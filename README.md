# Project Template Generator

The Project Template Generator is a command-line interface (CLI) application written in Rust. 
It allows you to quickly generate a project template with a predefined directory structure and common files for your Python projects.

## Features
- Generates a project template with a venv and requirement.txt.
- Customizable project name
- Easy setup and usage.

## Installation
1. Make sure you have Python 3 installed on your system. You can download it from the official Python website (https://www.python.org).
2. Make sure you have Rust installed on your system. You can download and install it from the official Rust website (https://www.rust-lang.org/tools/install).
3. Clone this repository or download the source code.
```shell
   git clone https://github.com/your-username/project-template-generator.git
```
3. Build the project
```shell
   cargo build --release
```
4. Add the generated target/release folder to your system's environments variables

## Usage
1. To generate a project template in your current directory run
```shell
   pyc create <project_name>
```

## Future Development 
This Project Template Generator is an ongoing project, and there are several areas that can be improved and expanded. Some ideas for future development include:

- Adding support for different programming languages and frameworks.
- Providing additional project configuration options.
- Creating a git repo. 
- Implementing templates for specific project types (e.g., web applications, data science projects).
- Enhancing the CLI interface with more interactive features and options.
