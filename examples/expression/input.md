В Rust почти каждые фигурные скобки являются выражением, это означает, что они могут возвратить результат. Это не всегда нужно, поэтому можно не возвращать его добавив в конец `;`.

Выражения в блоке могут использоваться в качестве [r-values](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue) значений. Последнее выражение в блоке будет назначено как [l-value](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue).
> Что такое Rvalue и Lvalue читайте [здесь](http://msdn.microsoft.com/ru-ru/library/f90831hc.aspx).

Но, если последнее выражение в блоке будет точкой с запятой, результат будет `()`.

{expression.play}
