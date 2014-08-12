fn main() {
    // Эта переменная живет в области функции main
    let long_lived_variable = 1i;

    // Это блок, он имеет меньший объем нежели основная функция
    {
        // Эта переменная существует только в этом блоке
        let short_lived_variable = 2i;

        println!("inner short: {}", short_lived_variable);

        // Эта переменная не видна внешней функции
        let long_lived_variable = 5_f32;

        println!("inner long: {}", long_lived_variable);
    }
    // Конец блока

    // Ошибка! `short_lived_variable` не существует в этой области
    println!("outer short: {}", short_lived_variable);
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    println!("outer long: {}", long_lived_variable);
}
