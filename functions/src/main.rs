fn main() {
    print_labeled_measurement(5, 'h');
    test_section();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn test_section() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
