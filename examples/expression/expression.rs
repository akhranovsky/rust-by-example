fn main() {
    let x = 5u;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // `y` будет этим выражением
        x_cube + x_squared + x
    };

    let z = {
        // Точка с запятой "подавляет" последнее выражение, вместо него
        // будет записано `()` в переменную `z`
        2 * x;
    };

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
