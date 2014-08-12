fn main() {
    // Целочисленное сложение
    println!("1 + 2 = {}", 1u + 2);

    // Вычитание
    println!("1 - 2 = {}", 1i - 2);
    // Попробуйте изменить `1i` на `1u` и понять, почему тип важен

    // Булева логика
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Битовые операции
    println!("0011 AND 0101 is {:04t}", 0b0011u & 0b0101);
    println!("0011 OR 0101 is {:04t}", 0b0011u | 0b0101);
    println!("0011 XOR 0101 is {:04t}", 0b0011u ^ 0b0101);
    println!("1 << 5 is {}", 1u << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u >> 2);

    // Используйте подчеркивания, чтобы улучшить читаемость
    println!("One million is written as {}", 1_000_000u);
}
