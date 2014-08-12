fn main() {
    // Литералы с суффиксами, их вид известен при инициализации
    let x = 1u8;
    let y = 2u;
    let z = 3f32;

    // Литералы без суффикса, их вид зависит от того, как они используются
    let i = 1;
    let f = 1.0;

    // `size_of_val` возвращает размер переменной в байтах
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // Ограничения (слагаемые должны иметь тот же тип) для `i` и `f`
    let _constraint_i = x + i;
    let _constraint_f = z + f;
    // Закомментируйте эти две строки
}
