Viewing a Structure
===

Now that we have set up our runtime to make kitties, we should check our work!

We have introduced a custom structure to our chain, and while the Polkadot-JS Apps UI is very good at adapting to our changes, in this situation, we need to give it a hint on how to deserialize our structured data.

> REMINDER: Remember to reset your chain so that you start of fresh when interacting with the UI:
>
> ```
> ./scripts/build.sh
> cargo build --release
> ./target/release/substrate-verifiable-credentials purge-chain --dev
> ./target/release/substrate-verifiable-credentials --dev
> ```

## Registering a Custom Struct

Fortunately, the Polkadot-JS Apps UI provides us with a very simple way to import custom structures so that the page will be able to decode the information correctly.

Go back to the **Settings** app. Under the **Developer** section, you can either submit a JSON file with your custom structure or add it manually through a code editor. Copy & paste this JSON object into the code editor and press `Save`.

```
{
    "Credential": {
        "subject": "u32",
        "when": "Moment",
        "by": "AccountId"
    }
}
```

## Issuing a Credential

After we have created a subject previously, we can now issue credentials for them. In the **Extrinsics** app, go to:

```
VerifiableCredits > issueCredentials(AccountId, u32)
```
You will be asked, which AccounId you want to issue the subject to and the subject id we previously generated.

## Viewing a Credential

Finally, we can go into the **Chain State** app and view our stored credential object. Select:

```
VerifiableCreds > credentials((AccountId, u32)): Credential
```

Then select the account you issued the credentials for. You should then be able to see the individual properties of the `Credential` object.
