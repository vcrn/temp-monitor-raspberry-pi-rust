/// CLI app which monitors temperature of GPU and CPU of Raspberry Pi running RaspberryOS 10.
/// Author: github.com/vicnil

use std::io::Write;
use std::process::Command;
use std::{io, str, thread, time};
use rounded_div::RoundedDiv;

fn main() {
    println!("========================================================================");
    println!("This CLI app monitors the GPU and CPU temperatures of a Raspberry Pi.");
    print_cpu_load(2, 0);

    let delay = take_input();
    rerun_print_info(delay, 0);
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
    // Check if it's an integer and positive. Else return 2.
    match trimmed.parse::<u64>() {
        Ok(i) if i > 0 => i, // If positive integer.
        _ => 2,
    }
}

/// Perpetually runs print_temp() with an interval of parameter 'delay'.
fn rerun_print_info(delay: u64, cpu_tot_load_prev: usize) {
    print!("{esc}c", esc = 27 as char); // Clears the terminal.
    let cpu_tot_load_prev_updated = print_info(delay, cpu_tot_load_prev);
    //println!("cpu_tot_load_prev_updated is {}", cpu_tot_load_prev_updated);
    thread::sleep(time::Duration::from_secs(delay));
    rerun_print_info(delay, cpu_tot_load_prev_updated);
}

fn print_info(delay: u64, cpu_tot_load_prev: usize) -> usize{
    println!("============================");
    print_temp();
    print_ram_use();
    let cpu_tot_load = print_cpu_load(delay, cpu_tot_load_prev);
    println!("CPU saved is {}", cpu_tot_load);
    println!("| Press Ctrl + C to abort  |");
    println!("============================");
    cpu_tot_load
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


/// Prints available and total RAM
fn print_ram_use() {
    let mem_info = Command::new("cat")
        .arg("/proc/meminfo")
        .output()
        .expect("Failed to execute command");

    let mem_info_utf8 = str::from_utf8(&mem_info.stdout)
        .ok()
        .expect("Failed to convert mem info from byte string");
    
    let mut mem_info_utf8_split: Vec<&str> = mem_info_utf8.split(&[' ', '\n'][..]).collect();

    mem_info_utf8_split.retain(|&x| x != "");  // Removes empty elements.

    let mem_total = mem_info_utf8_split[1];
    let mem_available = mem_info_utf8_split[7];

    let mem_total_mb = mem_total.parse::<usize>().unwrap().rounded_div(1000);
    let mem_available_mb = mem_available.parse::<usize>().unwrap().rounded_div(1000);

    let formatted_ram = format!("{}/{} MB", mem_total_mb-mem_available_mb, mem_total_mb);  // TODO: Change to isize in case diff is slightly < 0?

    println!("| RAM used:{:>15} |", formatted_ram);  // Sets correct amount of leading whitespace.
}


fn print_cpu_load(delay: u64, cpu_tot_load_prev: usize) -> usize{
    let cpu_info = Command::new("cat")
        .arg("/proc/stat")
        .output()
        .expect("Failed to execute command");

    let cpu_info_utf8 = str::from_utf8(&cpu_info.stdout)
        .ok()
        .expect("Failed to convert cpu info from byte string");

    let cpu_info_utf8_split: Vec<&str> = cpu_info_utf8.split(&[' ', '\n'][..]).collect();
    
    // TODO: Make it more universal, and not just for Raspberry 4 (probably different amount of CPUs. Lazy way: only check total CPU). Search for "cpu", save the string and number after (need to remove empty elements).
    let cpu_tot_load = cpu_info_utf8_split[2].parse::<usize>().unwrap();
    let cpu0_load = cpu_info_utf8_split[13].parse::<usize>().unwrap();
    let index_cpu1 = cpu_info_utf8_split.iter().position(|&r| r == "cpu1").unwrap();
    let cpu1_load = cpu_info_utf8_split[index_cpu1+1];
    let index_cpu2 = cpu_info_utf8_split.iter().position(|&r| r == "cpu2").unwrap();
    let cpu2_load = cpu_info_utf8_split[index_cpu2+1];
    let index_cpu3 = cpu_info_utf8_split.iter().position(|&r| r == "cpu3").unwrap();
    let cpu3_load = cpu_info_utf8_split[index_cpu3+1];

    //println!("CPU INFO {:?}", cpu_info_utf8);
    //println!("CPU INFO {:?}", cpu_info_utf8_split);
    println!("CPU:  {}", cpu_tot_load);
    println!("CPU0: {}", cpu0_load);
    println!("CPU1: {}", cpu1_load);
    println!("CPU2: {}", cpu2_load);
    println!("CPU3: {}", cpu3_load);

    println!("CPU prev is {}", cpu_tot_load_prev);

    println!("CPU0 diff:  {}", cpu0_load - cpu_tot_load_prev);

    cpu0_load
}