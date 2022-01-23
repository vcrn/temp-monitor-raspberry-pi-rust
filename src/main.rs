/// CLI app which monitors temperature of GPU and CPU of Raspberry Pi running RaspberryOS 10.
/// Author: github.com/vicnil

use std::io::Write;
use std::process::Command;
use std::{io, str, thread, time};

fn main() {
    println!("========================================================================");
    println!("This CLI app monitors the GPU and CPU temperatures of a Raspberry Pi.");

    let delay = take_input();
    rerun_print_temp(delay);
}

/// Takes the input 'delay' from the user, returning '2' if none valid number is entered.
fn take_input() -> u64 {
    print!("Enter the temperature update interval in whole seconds (2 by default): ");
    io::stdout().flush().expect("Could not flush stdout."); // Used with above line so input is entered at same line as output in terminal.

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
        // Check if it's an integer and positive. Else return 2.
        Ok(i) if i > 0 => i, // If positive integer.
        _ => 2,
    }
}

/// Perpetually runs print_temp() with an interval of parameter 'delay'.
fn rerun_print_temp(delay: u64) {
    print!("{esc}c", esc = 27 as char); // Clears the terminal.
    println!("============================");
    print_temp();
    println!("| Press Ctrl + C to abort  |");
    println!("============================");
    thread::sleep(time::Duration::from_secs(delay));
    rerun_print_temp(delay);
}

/// Prints the GPU and CPU temperatures
fn print_temp() {
    // Get GPU temperature.
    // Returns stdout (standard output stream) and stderr (standard error stream).
    let gpu_temp_output = Command::new("vcgencmd")
        .arg("measure_temp")
        .output()
        .expect("Failed to execute command");
    // Takes stdout (bytestring), turns to utf8, splits at '=' and ''' and saves as vector.
    let gpu_temp_str_splitted: Vec<&str> = str::from_utf8(&gpu_temp_output.stdout)
        .ok()
        .expect("Failed to convert from byte string")
        .split(['=', '\''].as_ref())
        .collect();
    //  Takes second element of vector, i.e. the temperature.
    let gpu_temp = gpu_temp_str_splitted[1];

    // Get CPU temperature.
    let cpu_temp_output = Command::new("cat")
        .arg("/sys/class/thermal/thermal_zone0/temp")
        .output()
        .expect("Failed to execute command");
    // Returns the CPU temperature multiplied by 1000.
    let cpu_temp_str_1000 = str::from_utf8(&cpu_temp_output.stdout)
        .ok()
        .expect("Failed to convert from byte string");
    // Takes first to third element, i.e. the CPU temperature multiplied by 10.
    let cpu_temp_str_10 = &cpu_temp_str_1000[0..3];
    let cpu_temp = cpu_temp_str_10.parse::<f32>().unwrap() / 10.0;

    println!("| GPU temperature: {}\u{00B0} C |", gpu_temp);
    println!("| CPU temperature: {}\u{00B0} C |", cpu_temp);
}
