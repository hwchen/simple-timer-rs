#[macro_export]
macro_rules! timeit {
    ($t: literal, $x:expr) => {
        {
            use std::time::Instant;
            let start = Instant::now();
            $x;
            let end = start.elapsed();
            println!("time({}) : {}.{:03}", $t, end.as_secs(), end.subsec_millis());
        }
    };
}
