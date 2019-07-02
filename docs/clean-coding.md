## Cleaning up

While we are already add it, let's clean up the existing code base a bit to make it more legible and easier to enhance later.

## Getters

We've already spoken about getters previously. By adding a `get(name_of_accessor)` to the storage definition, we can use `Self::name_of_accessor()` instead of the less legible `<Whatever<T>>::get()` in the code base. This works for single values as well as for maps.

## Type aliases

Secondly, we are using `u32` to refer to our subject, but that is not very legible - especially in type definitions. Luckily, rust knows of something called "type alias", where you can use any existing type and define a local alias for it. You can then use that alias instead of the previous type, making the code more expressive.

``` rust
pub type MyType = u32;
```

## Time to clean up

Okay, let's go in there and clean up our code a bit. Ad get accessors to the existing storage items and add a type alias we can use throughout the file instead of `u32`.

<!-- tabs:start -->

#### ** Template **

[embedded-code](../assets/1.8-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../assets/1.8-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code](../assets/1.7-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
