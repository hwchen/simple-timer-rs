fn timed_fn() {
    println!("hello world");
}

#[test]
fn test_it_works() {
    simple_timer::timeit!("one", timed_fn());
    simple_timer::timeit!("two" ,timed_fn());

    simple_timer::timeit!("two",
        {
            println!("great weather");
            println!("i agree");
        }
    );

    let forty_two = simple_timer::timeit!("three", 42);
    assert_eq!(forty_two, 42);
}
