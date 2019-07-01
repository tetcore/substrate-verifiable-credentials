## Securing the issuing credentials

You might have noticed, that at this point anyone can issue credentials to anyone, regardless of their connection to the subject - nor do we check whether the referenced subject even exists in the first place. That is obviously less than ideal.

So, let's add some code to secure the issuing of credentials.

FIXME: RE-ADD verify-first paragraph.

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
