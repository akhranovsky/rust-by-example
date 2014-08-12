// `NanoSecond` это новое имя для `u64`
type NanoSecond = u64;
type Inch = u64;

// Используйте этот атрибут, чтобы не выводить предупреждение
#[allow(non_camel_case_types)]
type uint64_t = u64;
// Попробуйте удалить атрибут

fn main() {
    // `NanoSecond` = `Inch` = `uint64_t` = `u64`
    let nanoseconds: NanoSecond = 5 as uint64_t;
    let inches: Inch = 2 as uint64_t;

    // Обратите внимание, что псевдонимы новых типов не предоставляют
    // дополнительную безопасность, из-за того, что они не нового типа
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
