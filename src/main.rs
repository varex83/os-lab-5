use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;

fn main() {
    let (tx_f, rx_f) = mpsc::channel();
    let (tx_g, rx_g) = mpsc::channel();

    let mut input_string = String::new();
    println!("Enter a value for x:");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let x: i32 = input_string.trim().parse().expect("Please type a number!");

    thread::spawn(move || {
        let result = f(x);
        tx_f.send(result).unwrap();
    });

    thread::spawn(move || {
        let result = g(x);
        tx_g.send(result).unwrap();
    });

    let fx = rx_f.recv().unwrap();
    let gx = rx_g.recv().unwrap();

    match (fx, gx) {
        (Ok(f_val), Ok(g_val)) => println!("Result of f(x) * g(x) = {}", f_val * g_val),
        _ => println!("One of the functions did not return a valid result."),
    }
}

fn f(x: i32) -> Result<i32, ()> {
    thread::sleep(Duration::from_secs(2));
    Ok(x + 1)
}

fn g(x: i32) -> Result<i32, ()> {
    thread::sleep(Duration::from_secs(3));
    Ok(x * 2)
}
