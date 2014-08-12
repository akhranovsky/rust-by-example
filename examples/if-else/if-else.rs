fn main() {
    let n = 5i;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // Это выражение возвращает `int`
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // Это выражение должно возвращать `int`
            n / 2
            // ^ Попробуйте поставить точку с запятой
        };

    println!("{} -> {}", n, big_n);
}
