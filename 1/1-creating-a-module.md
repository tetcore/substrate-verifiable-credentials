Creating a Module
===

[Cryptokitties](https://www.cryptokitties.co/) is a popular Ethereum dApp that at one point accounted for more than 20% of all incoming transactions on the blockchain. It only makes sense to follow in their footsteps and create the next viral application on Substrate.

To start, we need to enable users to both create and own kitties. For that we will work with an empty module template which we will place in a new `cryptokitties.rs` file:

```
substrate-collectables
|
+-- runtime
    |  
    +-- src
        |
        +-- lib.rs
        |
        +-- * cryptokitties.rs
```

**cryptokitties<span>.</span>rs**

```
pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
          // Declare storage and getter functions here
  }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
    }
}
```

You can see that this template allows us to start writing the most basic parts of our runtime, the public functions and the storage.

But before we even start doing that, we should include this file into our overall runtime which is defined in the `lib.rs` file located in the same directory.

If you take a closer look at the `lib.rs` file, you will notice it contains details about all the modules that make up your runtime. For each module, we:

 - Import the rust file containing the module
 - Implement it's `Trait`
 - Include the module into the `construct_runtime!` macro

 So we will need to do the same here.

 To include the new module file we created, we simply add this line near the top of our file [TODO: Better way to describe the location?]:

 ```
 mod cryptokitties;
 ```

Since we have not defined anything in our module, our `Trait` implementation is also very simple. We can include this line after the other trait implementations:

```
impl cryptokitties::Trait for Runtime {}
```

Finally, we can add this line at the end of our `construct_runtime!` definition:

```
Cryptokitties: cryptokitties::{Module, Call, Storage},
```

Note that we have added three `types` to this definition (`Module`, `Call`, `Storage`), all of which are produced by the macros defined in our template.

As is, this code is valid and should compile. Give it a shot with:

```
./build.sh
cargo build --release
```

Now it's time to start adding some of our own logic!

---
**Learn More**

Link to wiki page on Constructing a runtime!

[TODO: make this a page]

---