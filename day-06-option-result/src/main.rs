fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = safe_divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result: {value}"),
        Err(e) => println!("Error: {e}"),
    }

    let bad = safe_divide(10.0, 0.0);
    match bad {
        Ok(value) => println!("Result: {value}"),
        Err(e) => println!("Error: {e}"),
    }
}
