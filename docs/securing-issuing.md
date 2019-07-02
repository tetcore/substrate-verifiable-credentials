## Securing the issuing credentials

You might have noticed, that at this point anyone can issue credentials to anyone, regardless of their connection to the subject - nor do we check whether the referenced subject even exists in the first place. That is obviously less than ideal.

So, let's add some code to secure the issuing of credentials.

## "Verify First, Write Last"

> IMPORTANT: This section is important

As a developer building on Substrate, it is critical that you make a distinction about how you should design your runtime logic versus developing a smart contract on a platform like Ethereum.

On Ethereum, if at any point your transaction fails (error, out of gas, etc...), the state of your smart contract will be unaffected. However, on Substrate this is not the case. As soon as a transaction starts to modify the storage of the blockchain, those changes are permanent, even if the transaction would fail at a later time during runtime execution.

This is necessary for blockchain systems since you may want to track things like the nonce of a user or subtract gas fees for any computation that occurred. Both of these things actually happen in the Ethereum state transition function for failed transactions, but you have never had to worry about managing those things as a contract developer.

Now that you are a Substrate runtime developer, you will have to be conscious of any changes you make to the state of your blockchain, and ensure that it follows the **"verify first, write last"** pattern. We will be helping you do this throughout the tutorial.


## Your turn

Similarly to `ensure_signed!` the regular `ensure!`-macro allows us to assert a regular boolean. As with `ensure_signed`, execution will stop if the `ensure!`-macro asserts to false. We can use this, together with the `subjects` map to ensure the `sender` is allowed to issue the given `credential`. 

There is a second potential vulnerbility in our current code though, did you spot it yet? As a hint, what happens if `SubjectCount` reached `MAX_U32`? While this is very unlikely to happen soon, we are working with a lower level code base in runtimes and need to take care of these border- and edge cases from early on. For now, let's just use the same `ensure!` to check that the value will not overflow on `createSubject`.


<!-- tabs:start -->

#### ** Template **

[embedded-code](../assets/1.7-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../assets/1.7-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code](../assets/1.6-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
