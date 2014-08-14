fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // Выход из внутренней петли
            //break;

            // Выход из внешней петли
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
