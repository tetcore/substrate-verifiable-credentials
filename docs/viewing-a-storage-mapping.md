## Checking Our Work in the Polkadot UI

Even though this code should compile without errors, now would be a good time to check out our work.

After running:

```bash
./scripts/build.sh
cargo build --release
./target/release/substrate-verifiable-credentials purge-chain --dev
```

We can start our node:

```bash
./target/release/substrate-verifiable-credentials --dev
```

If we go back into the [Polkadot-JS Apps UI](https://polkadot.js.org/apps), we should see evidence of our node producing blocks.

## Submit a Transaction

Go to the **Extrinsics** app, and using the "from extrinsic section" dropdown select:

```
substrate-verifiable-credentials > createSubject()
```

Type in a value and press `Submit Transaction`.

## View the Storage

Now that you have submitted a transaction to create a new subject into storage, we should take a look that the subject is properly assigned.

Go to the **Chain state** app and select:

```
VerifiableCreds > subjects(u32): AccountId
```

Just put in `0` (or any higher number, if you created multiple subjects) and query the storage by pressing the blue `[+]` button. It should show that the value is the accountId you submitted the previous transaction with.
