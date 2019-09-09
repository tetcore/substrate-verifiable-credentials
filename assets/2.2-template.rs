// ACTION: Add `support::decl_event` to use the `decl_event!` macro
use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};
use core::u32::MAX as MAX_SUBJECT;

// NOTE: we have moved the code to have a cleaner
//       structure
pub type Subject = u32;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Credential<Timestamp, AccountId> {
    subject: Subject,
    when: Timestamp,
    by: AccountId
}

pub trait Trait: system::Trait + timestamp::Trait  {
    // ACTION: Define your `Event` type here
    //   HINT: It needs these traits: `From<Event<Self>> + Into<<Self as system::Trait>::Event>`
}
// NOTE: We have added this `decl_event!` template for you
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // ACTION: Add a `SubjectCreated` event which includes an `AccountId` the `u32`
        // ACTION: Add a `CredentialIssued` event which includes an `AccountId` the `u32` and the issuers `AccountId`
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as VerifiableCreds {
        SubjectCount get(subject_count) config(): Subject;
        Subjects get(subjects): map Subject => T::AccountId;
        Credentials get(credentials): map (T::AccountId, Subject) => Credential<T::Moment, T::AccountId>;
    }
}


decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // ACTION: Define your generic `deposit_event<T>()` function
        //      REMINDER: You can use the default implementation provided by the `decl_module!` macro with `default`

        fn create_subject(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let subject = Self::subject_count();

            ensure!(subject <= MAX_SUBJECT, "Exhausted all Subjects");

            <SubjectCount<T>>::put(subject + 1);
            <Subjects<T>>::insert(subject, sender);
            // ACTION: Deposit your event here

            Ok(())
        }

        fn issue_credential(origin, to: T::AccountId, subject: Subject) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(Self::subjects(subject) == sender, "Unauthorized.");

            let new_cred = Credential {
                subject: subject,
                when: <timestamp::Module<T>>::get(),
                by: sender,
            };

            <Credentials<T>>::insert((to, subject), new_cred);
            // ACTION: Deposit your event here

            Ok(())
        }
    }
}
// ACTION: Don't forget to update `lib.rs` with the `Event` type
