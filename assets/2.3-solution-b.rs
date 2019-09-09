use support::{decl_storage, decl_event, decl_module, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};
use core::u32::MAX as MAX_SUBJECT;

pub type Subject = u32;

// SOLUTION B: we add an new field on the credential allowing us to track whether it has been 
//             revoked. This makes it possible to keep exact timestamp of revokation accessible easily
//             and thus distinguish between revoked and not-yet-issued. It also always later to add
//             further options to for e.g. give reasons for revokation.


#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Credential<Timestamp, AccountId> {
    subject: Subject,
    when: Timestamp,
    by: AccountId,
    // We added another optional field
    revoked: Option<Timestamp>,
}

pub trait Trait: system::Trait + timestamp::Trait  {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        SubjectCreated(AccountId, Subject),
        CredentialIssued(AccountId, Subject, AccountId),
        CredentialRevoked(AccountId, Subject, AccountId),
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
        fn deposit_event<T>() = default;

        /// Verify a credential.
        pub fn verify_credential(origin, holder: T::AccountId, subject: Subject) {
            let _sender = ensure_signed(origin)?;

            // Ensure credential is issued and allowed to be verified.
            ensure!(<Credentials<T>>::exists((holder.clone(), subject)), "Credential not issued yet.");
            ensure!(Self::credentials((holder.clone(), subject)).revoked.is_some(), "Credential has been revoked.");
        }

        /// Revoke a credential.
        /// Only an issuer can call this function. 
        pub fn revoke_credential(origin, to: T::AccountId, subject: Subject) {
            // Check if origin is an issuer.
            // Check if credential is issued.
            // Change the internal flag to the current timestamp

            let sender = ensure_signed(origin)?;
            let subject_issuer = Self::subjects(subject);
            ensure!(subject_issuer == sender, "Unauthorized.");
            ensure!(<Credentials<T>>::exists((to.clone(), subject)), "Credential not issued yet.");

            <Credentials<T>>::mutate(|&mut cred| {
                cred.revoked = Some(<timestamp::Module<T>>::get())
            });
            Self::deposit_event(RawEvent::CredentialRevoked(to, subject, sender));
        }

        fn create_subject(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let subject = Self::subject_count();

            ensure!(subject <= MAX_SUBJECT, "Exhausted all Subjects");

            <SubjectCount<T>>::put(subject + 1);
            <Subjects<T>>::insert(subject, sender);

            Self::deposit_event(
              RawEvent::SubjectCreated(sender, subject)
            );

            Ok(())
        }

        fn issue_credential(origin, to: T::AccountId, subject: Subject) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(Self::subjects(subject) == sender, "Unauthorized.");

            let new_cred = Credential {
                subject: subject,
                when: <timestamp::Module<T>>::get(),
                by: sender,
                revoked: None, // we need to add the empty field here
            };

            <Credentials<T>>::insert((&sender, subject), new_cred);
            Self::deposit_event(
              RawEvent::CredentialIssued(to, subject, sender)
            );
            
            Ok(())
        }
    }
}