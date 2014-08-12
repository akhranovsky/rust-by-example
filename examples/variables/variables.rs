fn main() {
    let an_integer = 1u;
    let a_boolean = true;
    let unit = ();

    // скопировать значение `an_integer` в `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {}", unit);

    // Компилятор предупреждает о неиспользуемых переменных; эти предупреждения можно
    // отключить используя подчёркивание перед именем переменной
    let _unused_variable = 3u;
    let noisy_unused_variable = 2u;
    // ИСПРАВЬТЕ ^ Добавьте подчёркивание
}
