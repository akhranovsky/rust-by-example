fn main() {
    let _immutable_variable = 1i;
    let mut mutable_variable = 1i;

    println!("Before mutation: {}", mutable_variable);

    // Ок
    mutable_variable += 1;

    println!("After mutation: {}", mutable_variable);

    // Ошибка!
    _immutable_variable += 1;
}
