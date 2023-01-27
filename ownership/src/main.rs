fn main() {
    let x = give_ownership(); // Ownership taken from function
    let x2 = take_and_give(x); // Must assign to a new variable, x is given away and no longer valid
    take_ownership(x2); // x no longer valid

    let x3: String = String::from("hello 2");
    borrow(&x3);
    println!("x3 is {x3}"); // Still valid since we still have ownership

    let mut x3: String = x3;
    borrow_mutable(&mut x3);
    println!("x3 is {x3}"); // x3 reference is mutated in borrow_mutable() so shows new value
}

fn give_ownership() -> String {
    String::from("hello") // Ownership created and given away
}

fn take_and_give(x: String) -> String {
    println!("Ownership temporarily taken for {x}.");
    x
}

fn take_ownership(x: String) {
    println!("Ownership for {x} taken.") // Ownership taken and discarded after fn scope
}

fn borrow(x: &String) -> usize {
    x.len()
}

fn borrow_mutable(x: &mut String) {
    x.push_str(" world");
}
