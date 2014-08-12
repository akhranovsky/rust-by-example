// Функция, которая возвращает булево значение
fn is_divisible_by(lhs: uint, rhs: uint) -> bool {
    // Некорректный входной параметр, сразу выходим из функции
    if rhs == 0 {
        return false;
    }

    // Это выражение, так что ключевое слово `return` тут не нужно
    lhs % rhs == 0
}

// Функции, которые не возвращают значение, на самом деле возвращают тип `()`
fn fizzbuzz(n: uint) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// Если функция возвращает `()`, возвращаемый тип можно опустить
fn fizzbuzz_to(n: uint) {
    for n in range(1, n + 1) {
        fizzbuzz(n);
    }
}

fn main() {
    fizzbuzz_to(100);
}
