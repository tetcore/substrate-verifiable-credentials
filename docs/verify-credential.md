## Verify Credentials

Now that we can securely issue credentials, others can easily go to third parties and _claim that their account holds a specific subject credential_ by providing `AccountId` and `Subject`. However, for the verifyer that isn't actually easy to confirm at the moment: they would have to understand our internal structures, where and how they are stored.

Let's make it a bit more convenient to them and provide a helper method `verify_credential` that either succeeds or fails to execute depending whether the given account has the given credentials. We can use `ensure!` to do that.

## Revokation

And while we are add it, let's also add a helper function that allows the issuer to revoke an issued credential. Just think of someone loosing their drivers license because of driving while intoxicated - in practice being able to revoke a credential is existential. And don't forget that the verification must not check out after the credentials have been revoked, and it would be useful to have a new Event alert us when this happens.

## Your Turn

Now you can show what you've learnt and put all that into practice. We've added the functions in the template, but they are still missing their parameters and all the code. Go ahead and fill that in.


<!-- tabs:start -->

#### ** Template **

[embedded-code-template](../assets/2.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../assets/2.3-solution-a.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code-previous](../assets/2.2-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->