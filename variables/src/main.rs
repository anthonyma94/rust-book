fn main() {
    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("Implicit return: {}", implicit_return());
    println!("Counter value: {}", loop_return());

    loop_labels();
}

fn implicit_return() -> i32 {
    45
}

fn another_function(x: i32) {
    println!("THe value of x is: {x}");
}

fn loop_return() -> i32 {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}")
}
