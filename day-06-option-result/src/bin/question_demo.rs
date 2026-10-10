fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn calculate() -> Result<f64, String> {
    let x = safe_divide(10.0, 2.0)?;
    let y = safe_divide(x, 5.0)?;
    Ok(y * 100.0)
}

fn bad_calculate() -> Result<f64, String> {
    let x = safe_divide(10.0, 0.0)?;
    Ok(x * 2.0)
}

fn main() {
    match calculate() {
        Ok(v) => println!("calculate: {v}"),
        Err(e) => println!("calculate error: {e}"),
    }
    match bad_calculate() {
        Ok(v) => println!("bad_calculate: {v}"),
        Err(e) => println!("bad_calculate error: {e}"),
    }
}
