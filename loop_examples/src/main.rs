fn main() {
    loop_return_value_from_loop();

    iterate_through_array();
}

fn loop_return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Final value is {}", result);
}


fn iterate_through_array() {
    let a = [90, 30, 908, 45, 89, 09, 878, 65, 78];
    for element in a.iter() {
        println!("Value is {}", element);
    }
}