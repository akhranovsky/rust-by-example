fn main() {
    let x = 5u;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // `y` будет этим выражением
        x_cube + x_squared + x
    };

    let z = {
        // Если в конце стоит точка с запятой, то выражение не
        // присваивается, вместо него переменная `z` будет содержать `()`
        2 * x;
    };

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
