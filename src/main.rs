
fn main() {
    println!("Hello, world!");
    convert_fto_c(70);
    let nth = nth_fibonaci_number(9);
    println!("8th fibonaci value is {}", nth);
}


fn convert_fto_c(far: u32) -> f64 {
    println!("Your FAHRENHEIT temp is {}", far);
    let c = (far - 32) as f64 * 0.5556;
    println!("Celcius tempreture is {}", c);
    c
}

fn nth_fibonaci_number(n: u32) -> u32 {
    const PHI : f64 = 1.6180339;
    let f = [0, 1, 1, 2, 3, 5];

    if n < 6 {
        f[n as usize]
    } else {
        let mut t = 5;
        let mut fib_n = 5;    
        while t < n {
            fib_n = ((fib_n as f64 * PHI) as f64).round() as u32;
            t = t + 1;
        }
        fib_n
    }
}