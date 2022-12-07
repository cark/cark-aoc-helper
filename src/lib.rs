fn run_with_timing<Solution>(f: impl Fn() -> Solution) -> (Solution, u128) {
    let start_time = std::time::Instant::now();
    let result = f();
    let duration = start_time.elapsed().as_micros();
    (result, duration)
}

pub fn exec_printing_duration<Value>(topic: &str, f: impl Fn() -> Value) -> Value {
    let (result, duration) = run_with_timing(f);
    println!("{topic}: {duration} µs.");
    result
}

pub fn exec_and_print<Value>(topic: &str, f: impl Fn() -> Value)
where
    Value: std::fmt::Display,
{
    let (result, duration) = run_with_timing(f);
    println!("{topic}: {result} in {duration} µs.");
}

#[cfg(test)]
mod tests {
    // No tests, we never make any mistake ... right ?
}
