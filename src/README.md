# Pizza Builder Example

This project demonstrates the use of the Builder pattern in Rust to create pizza objects. The Builder pattern is a creational design pattern that allows for step-by-step creation of complex objects. It's particularly useful when an object can have numerous configurations.

## What it does

This code example showcases:

1. Definition of `Pizza`, `Size`, and `Topping` types.
2. Implementation of a `PizzaBuilder` to construct `Pizza` objects.
3. Usage of the builder pattern to create different pizza configurations.

The `PizzaBuilder` allows you to:
- Set the pizza's name
- Choose the size (Small, Medium, Large)
- Add up to three toppings

## Why use the Builder pattern?

The Builder pattern is beneficial in this scenario for several reasons:

1. **Flexible object creation**: It allows creating pizzas with different combinations of toppings and sizes without needing multiple constructors.

2. **Readable code**: The method chaining syntax (`builder.topping_1().topping_2().build()`) makes the object creation process more readable and intuitive.

3. **Default values**: The builder can set default values (e.g., medium size) which can be overridden if needed.

4. **Separation of concerns**: The construction process is separated from the pizza's representation, making it easier to add new pizza properties or change the construction process in the future.

## Limitations of this example

While this example demonstrates the basic concept of the Builder pattern, it has some limitations:

1. It doesn't prevent setting the same topping multiple times (e.g., `topping_3()` can be called twice, overriding the previous value).
2. The number of toppings is limited to three.
3. There's no validation to ensure that toppings are unique.

These limitations could be addressed with more advanced Rust features or a different design approach, depending on the specific requirements of a real-world application.

## Running the example

To run this example, ensure you have Rust installed on your system. Then, you can compile and run the code using:

```
rustc main.rs
./main
```

This will output the details of three different pizzas created using the `PizzaBuilder`.