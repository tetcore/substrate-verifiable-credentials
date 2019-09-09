use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
// ACTION: Import `runtime_primitives::traits::{As, Hash}`
// ACTION: Import `parity_codec::{Encode, Decode}`

pub trait Trait: system::Trait + timestamp::Trait {}

// NOTE: We have added this struct template for you
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Credential<Timestamp, AccountId> {
    // ACTION: Define the properties of your credential struct here
    //         - `subject` as a `u32` 
    //         - `when` as a `Timestamp`
    //         - `by` as a `AccountId`
}

decl_storage! {
    trait Store for Module<T: Trait> as VerifiableCreds {
        SubjectCount: u32;
        Subjects: map u32 => T::AccountId;
        // ACTION: Add a map variable to be named `Credentials` to store a `Credential<T::Moment, T::AccountId>`
        //         for every `(T::AccountId, u32)` (per subject).
        // ACTION: Add a getter function named `credentials`
    }
}
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_subject(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let subject = <SubjectCount<T>>::get();

            <SubjectCount<T>>::put(subject + 1);
            <Subjects<T>>::insert(subject, sender);

            Ok(())
        }
        // NOTE: We added a new function
        fn issue_credential(origin, to: T::AccountId, subject: u32) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Create a `Credential` object named `new_cred` here for the given subject
            // HINT: you can receive the current timestamp via ` <timestamp::Module<T>>::get()`

            // ACTION: Store your `new_cred` into the runtime storage for
            //         the given accountId

            Ok(())
        }
    }
}
