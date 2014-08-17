fn main() {
    // Переменная для счетчика
    let mut n = 1u;

    // Итерируем пока `n` меньше 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Увеличиваем счетчик
        n += 1;
    }
}
