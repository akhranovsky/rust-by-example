В числовых литералах тип может быть аннотирован, добавив тип в качестве суффикса, за исключением `uint`, использующей суффикс `u` и `int`, который использует суффикс `i`.

Тип литералов без суффикса будет зависеть от того, как они используются. Если никаких ограничений не существует, то компилятор выдаст сообщение об ошибке.


{literals.play}

Есть некоторые понятия, используемые в предыдущем коде, которые не были объяснены раньше, вот краткое объяснение для нетерпеливых читателей:
* `fun(&foo)` используется, чтобы передать аргумент в функцию по ссылке, а не по значению `fun(foo)`.
* `std::mem::size_of_val` является функцией, но вызывается с указанием полного пути. Код можно разделить на логические единицы, называемые модулями. Здесь функция `size_of_val` определена в модуле `mem`, а модуль `mem` определен в крэйте `std`.