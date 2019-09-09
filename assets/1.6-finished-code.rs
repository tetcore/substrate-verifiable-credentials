use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};

pub trait Trait: system::Trait + timestamp::Trait {}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Credential<Timestamp, AccountId> {
    subject: u32,
    when: Timestamp,
    by: AccountId
}

decl_storage! {
    trait Store for Module<T: Trait> as VerifiableCreds {
        SubjectCount: u32;
        Subjects: map u32 => T::AccountId;
        Credentials get(credentials): map (T::AccountId, u32) => Credential<T::Moment, T::AccountId>;
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

            let new_cred = Credential {
                subject: subject,
                when: <timestamp::Module<T>>::get(),
                by: sender,
            };

            <Credentials<T>>::insert((to, subject), new_cred);

            Ok(())
        }
    }
}
