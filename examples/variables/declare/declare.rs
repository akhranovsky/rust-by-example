fn main() {
    // Объявляем переменную
    let a_variable;

    {
        let x = 2i;

        // Инициализируем переменную
        a_variable = x * x;
    }

    println!("a variable: {}", a_variable);

    let another_variable;

    // Ошибка! Использование неинициализированной переменной
    println!("another variable: {}", another_variable);
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    another_variable = 1i;

    println!("another variable: {}", another_variable);
}
