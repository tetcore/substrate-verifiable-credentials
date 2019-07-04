use support::{decl_storage, decl_event, decl_module, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};
use core::u32::MAX as MAX_SUBJECT;

pub type Subject = u32;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Credential<Timestamp, AccountId> {
    subject: Subject,
    when: Timestamp,
    by: AccountId
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
        }

        /// Revoke a credential.
        /// Only an issuer can call this function. 
        pub fn revoke_credential(origin, to: T::AccountId, subject: Subject) {
            // Check if origin is an issuer.
            // Check if credential is issued.
            // Change the bool flag of the stored credential tuple to false.

            let sender = ensure_signed(origin)?;
            let subject_issuer = Self::subjects(subject);
            ensure!(subject_issuer == sender, "Unauthorized.");
            ensure!(<Credentials<T>>::exists((to.clone(), subject)), "Credential not issued yet.");

            <Credentials<T>>::remove((to.clone(), subject));
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

        fn issue_credential(origin, to: T:AccountId, subject: Subject) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(Self::subjects(subject) == sender, "Unauthorized.");

            let new_cred = Credential {
                subject: subject,
                when: <timestamp::Module<T>>::get(),
                by: sender,
            };

            <Credentials<T>>::insert((&sender, subject), new_cred);
            Self::deposit_event(
              RawEvent::CredentialIssued(to, subject, sender)
            );
            
            Ok(())
        }
    }
}