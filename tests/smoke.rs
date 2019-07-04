fn timed_fn() {
    println!("hello world");
}

#[test]
fn test_it_works() {
    simple_timer::timeit!("one", timed_fn());
    simple_timer::timeit!("two" ,timed_fn());
}
