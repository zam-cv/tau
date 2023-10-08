# Tau

Tau is a powerful project construction tool made in Rust. It enables users to build projects based on predefined templates and commands.

## Features
- **Template Management:** Easily manage templates by adding new ones, editing existing ones, or removing them.
- **Command Management:** Add or edit predefined commands that work seamlessly with your templates.
- **Directory Intelligence:** Tau recognizes the template in use within a directory and adjusts its available commands accordingly.
- **Root Directory Scaling:** Tau automatically scales across directories to find the root of your project and execute commands from there.

### Main Commands:
- `new`: Create a new project from an available template.
- `path`: Display the resource paths used by Tau.
- `list`: List all available templates.
- `help`: Print this help message or the help of the given subcommand(s).

## Getting Started

### 1. Installation:

To get started with Tau, you first need to install it. Follow these steps:

```bash
git clone https://github.com/zam-cv/tau
cd tau
cargo install --path .
```

### 2. Navigate to your project directory:

Move to the directory where you want to create or manage your project.

### 3. Create a new project:

Use the command `tau new <project_name>` to initiate a new project based on an available template.

### 4. Directory Intelligence:

Once you're within a project directory, Tau can detect the template in use and provides corresponding commands for that template.

### 5. Work seamlessly across directories:

Tau can traverse directories to find the root of your project, ensuring commands are executed correctly, even if you're in a subdirectory of the project.

### 6. Editing Commands and Managing Templates:

To edit commands or add more templates, navigate to the `config.json` file by using the command `tau path`. You will see:

```bash
Config: "PROJECT_DIR/config.json"
Templates: "PROJECT_DIR/templates/"
```

The `config.json` file is where you can add or modify commands. The `templates/` directory is where you can place new project templates or edit existing ones.

## Example

Example of configuring the default C++ template found in the `config.json` file.

```json
{
  "c++": {
    "optional_files": [
      ".gitignore",
      "README.md",
      "tests/test.cpp"
    ],
    "commands": {
      "run": {
        "tasks": [
          {
            "name": "Build",
            "command": "make"
          },
          {
            "name": "Run",
            "command": "./build/main"
          }
        ],
        "args": null,
        "description": "Build and run the project"
      },
      "bin": {
        "tasks": [
          {
            "name": "Bin",
            "command": "./build/main"
          }
        ],
        "args": null,
        "description": "Run the project"
      },
      "test": {
        "tasks": [
          {
            "name": "Test",
            "command": "make tests"
          },
          {
            "name": "Run",
            "command": "./build/tests"
          }
        ],
        "args": null,
        "description": "Build and run the tests"
      }
    },
    "routes": []
  }
}
```