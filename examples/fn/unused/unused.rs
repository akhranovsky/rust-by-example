fn used_function() {}

// `#[allow(dead_code)]` это атрибут, который отключает предупреждение `dead_code`
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// ИСПРАВЬТЕ ^ Добавьте атрибут, чтобы отключить предупреждение

fn main() {
    used_function();
}
