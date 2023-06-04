fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    let x = sum(5, 2);

    println!("Sum of 5 and 2 is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
