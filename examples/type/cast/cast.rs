fn main() {
    let decimal = 65.4321_f32;

    // Ошибка! Нет неявного преобразования
    let integer: u8 = decimal;
    // ИСПРАВЬТЕ ^ Закомментируйте строку

    // Явное преобразование
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}
