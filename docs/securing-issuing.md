## Securing the issuing of credentials

You might have noticed, that at this point anyone can issue credentials to anyone, regardless of their connection to the subject - nor do we check whether the referenced subject even exists in the first place. That is obviously less than ideal.

So, let's add some code to secure the issuing of credentials.

## "Verify First, Write Last"

> IMPORTANT: This section is important

As a developer building on Substrate, it is critical that you make a distinction about how you should design your runtime logic versus developing a smart contract on a platform like Ethereum.

On Ethereum, if at any point your transaction fails (error, out of gas, etc...), the state of your smart contract will be unaffected. However, on Substrate this is not the case. As soon as a transaction starts to modify the storage of the blockchain, those changes are permanent, even if the transaction would fail at a later time during runtime execution.

This is necessary for blockchain systems since you may want to track things like the nonce of a user or subtract gas fees for any computation that occurred. Both of these things actually happen in the Ethereum state transition function for failed transactions, but you have never had to worry about managing those things as a contract developer.

Now that you are a Substrate runtime developer, you will have to be conscious of any changes you make to the state of your blockchain, and ensure that it follows the **"verify first, write last"** pattern. We will be helping you do this throughout the tutorial.


## Rest `ensure`'d

Similarly to `ensure_signed!` the regular `ensure!`-macro allows us to assert a regular boolean. As with `ensure_signed`, execution will stop if the `ensure!`-macro asserts to false. We can use this, together with the `subjects` map to ensure the `sender` is allowed to issue the given `credential`. 

There is a second potential vulnerbility in our current code though, did you spot it yet? As a hint, what happens if `SubjectCount` reached `MAX_U32`? While this is very unlikely to happen soon, we are working with a lower level code base in runtimes and need to take care of these border- and edge cases from early on. For now, let's just use the same `ensure!` to check that the value will not overflow on `createSubject`.

## Initial values and defaults

One other issue, you might have noticed by now, is that our SubjectCount starts from `0` - which also equals the non-existing item as we've bound the count to a `u32` and `0` is its default value. That could cause issues down the line as mixing business-logic with special/edge-case values can result in unexpected behaviours - like checks passing that shouldn't or the other way around.

In order to prevent that problem, we should set the initial value in the genesis config. To allow this, we need to extend the definition of our module to contain the `Config` trait:

```rust
		VerifiableCreds: verifiablecreds::{Module, Call, Storage, Event<T>, Config<T>},
```

Now you can mark the state variables you want to allow to be configured by marking them with `config(name_of_variable)` - you can omit the name if there is already a getter configured:

```rust 
    trait Store for Module<T: Trait> as MyStore {
        MyNone get(my_nonce) config(): u32;
        NoGetter config(no_getter): u8;
    }
```

## Your turn

Now it is up to you to make our code more secure and robust: add gensis configurability to the `SubjectCount`, ensure it doesn't overflow when adding new subjects and verify that the origin is actually allowed to do so when issuing credentials.


<!-- tabs:start -->

#### ** Template **

[embedded-code](../assets/1.7-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../assets/1.7-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code](../assets/1.6-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
