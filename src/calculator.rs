pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        panic!("Division by zero is not allowed!");
    }
}

pub fn exponent(a: f64, b: f64) -> f64{
    if b == 0.0{
        1.0
    } else{
        return a.powf(b)
    }
}
