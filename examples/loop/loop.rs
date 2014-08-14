fn main() {
    let mut count = 0u;

    println!("Let's count until infinity!");

    // Бесконечная петля
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Пропустим оставшуюся часть этой итерации
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Выход из этой петли
            break;
        }
    }
}
