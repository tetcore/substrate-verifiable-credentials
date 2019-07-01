Introduction
===

[Verifiable Credentials](https://w3c.github.io/vc-data-model/) are an arising specification to model cryptographically proofable digital claims of credentials. To quote the abstract of the specification:
> Credentials are a part of our daily lives; driver's licenses are used to assert that we are capable of operating a motor vehicle, university degrees can be used to assert our level of education, and government-issued passports enable us to travel between countries. This specification provides a mechanism to express these sorts of credentials on the Web in a way that is cryptographically secure, privacy respecting, and machine-verifiable. 

In this tutorial we attempt to build a runtime that allows for a simplistic implementation of verifiable credentials. Though inspiried by the specification and using [its terminilogy](https://w3c.github.io/vc-data-model/#terminology), this is no attempt to create a feature complete or even spec compliant implementation. For our tutorial we will focus on the most simple case, where we know of two main objects: `identities` and `credential`.

In this chain, any `identity` can create a `credential` - making them the `issuer` - to any other `Identity` - making them the holder - by  assiging them a  `subject` through a signed extrinsics call. Once issued, the `holder` can claim to hold the `subject` and anyone can verify that claim by calling a function on the runtime with the `holder`s `Identity` and the given `subject`. Of course not everyone is allowed to create a `credential` for every `subject` but needs the appropriate `credentials` themselves - it is turtles all the way down ;D .

A simple example of that would be when a local government (`issuer`) would allow someone (`holder`)  to drive (`subject`) by issuing a drivers license (`credential`). In the physical world the drivers license is the de-facto proof of that, but if a police officer believes it to be faked, they might call in the issuing authority and ask them directly whether their records match up. In our chain, one would just `claim` to hold the `credentials` to drive and anyone can verify that claim against with the runtime. 

In this section, we will show you how to create a runtime which allows users create and own credentials.
