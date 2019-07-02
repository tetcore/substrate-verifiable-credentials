Writing Tests
===

Now that we have our test mock set up, we're ready to write some unit tests for our runtime.

Let's start with a simple test that passes by default. Make sure to add a `#[test]` compiler flag followed by your test set up function. Let's use an `assert!(true)` inside the test for now to make it pass by default. 

Don't forget that the test must execute within the context of `TestExternalities`!

```rust
#[test]
fn it_works() {
	with_externalities(&mut build_ext(), || {
		assert!(true);
	})
}
```

The command for executing this test is:
`cargo test -p substrate-verifiable-credentials-runtime substrate-verifiable-credentials`

You can read this line as run the `substrate-verifiable-credentials` unit test which is inside the `substrate-verifiable-credentials-runtime` sub-package. This verbose specification is necessary because your runtime is nested inside of another Substrate package. 

Conversely, if want to test your entire runtime package, you can also run `cargo test -p substrate-verifiable-credentials-runtime`.

The test should pass with the following console output: 

```zsh
running 1 tests
test template::tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Test "Add Subject" Works

We've done the heavy lifting in setting up the test mock. From this point forward, it is quite easy to write the tests to interact with your modules.

So far, your Credentials module provides critical functionalities like the ability for anyone to create a new subject.

Let's write a test to check that `should_add_subject`: 

```rust
#[test]
  fn should_add_subject() {
    with_externalities(&mut new_test_ext(), || {
        assert_ok!(
            VerifiableCreds::create_subject(Origin::signed(3)));
			// should be 3 because we configured
			// our genesis config to start at 3
        assert_eq!(
            VerifiableCreds::subjects(3), 3);
    });
  }
```

Notice how using a `u64` as `AccountId` is handy. Your account is simply referred to as `3` rather than a special type.

## Recommended Test Pattern

In Substrate, it is good practice to have comprehensive test coverage around your key state transition functions.

Recall that Substrate provides an `ensure!` macro used to check inputs and logic before updating runtime state. In particular, whenever this macro is used, you should implement thorough tests around any edge cases that may break the ensured assumptions. 

To aid you in writing comprehensive tests, the Substrate framework provides custom assert macros, in addition to the standard [assertion macros](https://doc.rust-lang.org/std/macro.assert.html) provided by Rust. 

You may want to frequently make use of: 
- [`assert_ok!()`](https://crates.parity.io/srml_support/macro.assert_ok.html): a special macro that checks a dispatch call returns an `Ok(())` Result. (Remember that dispatch calls return a special type of `Result<(), &'static str>`)
- [`assert_noop!()`](https://crates.parity.io/srml_support/macro.assert_noop.html): a special macro that checks that a call fails, whilst returning that particular error message string.

## Your Turn!

Now it is your turn. To complete this section, try writing tests for the following expectations:
  - Issuer can successfully issue a credential.
  - Issuing as a non-issuer fails. Specifically, make sure your test expectedly fails with an error message of: `"Unauthorized"`.
  - Revoking works as expected - maybe by testing verify at the same time?

You are encouraged to write as many tests as you can at this point. 

<!-- tabs:start -->

#### ** Template **

[embedded-code](../assets/5.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../assets/5.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->