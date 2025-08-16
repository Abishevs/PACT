use std::{fs::create_dir, path::Path};

use crate::{cli::NewProject, file_contents, utils::{create_license, write_to_file}};

pub fn setup_arduino_project(project_dir: &str, new_project: &NewProject) {
    let project_path = Path::new(&project_dir);
    let license_path = project_path.join("LICENSE");
    let arduino_dir = project_path.join(format!("{}", &new_project.project_name));
    create_dir(&arduino_dir).unwrap();

    let readme_path = project_path.join("README.md");
    let make_file_path = project_path.join("Makefile");
    let sketch_file_path = arduino_dir.join(format!("{}.ino", &new_project.project_name));
    let serial_connect_path = project_path.join("serial_connect.py");

    let serial_connect_content = "\
import serial
import time
# pip install pyserial

# Open serial connection
ser = serial.Serial(\'/dev/ttyUSB0\', 115200)  # Match the baud rate with the Arduino sketch

try:
    while True:
        if ser.in_waiting:
            line = ser.readline().decode('utf-8').rstrip()  # Read a line and strip trailing newline
            print(line)
        time.sleep(0.1)  # Small delay to prevent spamming
        
except KeyboardInterrupt:
    print(\"Program terminated!\")
finally:
    ser.close()  # Ensure the serial connection is closed on exit
";


    let make_file_content = format!("\
# Makefile for Arduino sketches
# Update these variables according to your project

SKETCH := {0}/{0}.ino
PORT := /dev/ttyUSB0
FQBN := esp32:esp32:esp32 # Platform

# Default target
all: upload

# Compile the sketch
build:
	arduino-cli compile --fqbn $(FQBN) $(SKETCH)

# Upload the sketch to the board
upload: build
	arduino-cli upload -p $(PORT) --fqbn $(FQBN) $(SKETCH)
", &new_project.project_name);

    let readme_content = format!("\
# {}

## Usage
```bash
    make upload
```
", &new_project.project_name);

    write_to_file(&sketch_file_path, file_contents::ARDUINO_SKETCH_CONTENT.as_bytes());
    write_to_file(&serial_connect_path, &serial_connect_content.as_bytes());
    write_to_file(&make_file_path, &make_file_content.as_bytes());
    write_to_file(&readme_path, &readme_content.as_bytes());
    create_license(&license_path);

}
