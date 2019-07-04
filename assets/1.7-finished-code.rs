use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};
use core::u32::MAX as MAX_SUBJECT;

pub trait Trait: balances::Trait + timestamp::Trait {}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Credential<Timestamp, AccountId> {
    subject: u32,
    when: Timestamp,
    by: AccountId
}

decl_storage! {
    trait Store for Module<T: Trait> as VerifiableCreds {
        SubjectCount config(subject_count): u32;
        Subjects: map u32 => T::AccountId;
        Credentials get(credentials): map (T::AccountId, u32) => Credential<T::Moment, T::AccountId>;
    }
}


decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_subject(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let subject = <SubjectCount<T>>::get();

            // An alternative would be to use Rusts internal checking system:
            // let new_subject = subject.checked_add(1).ok_or( "Exhausted all Subjects");
            ensure!(subject <= MAX_SUBJECT, "Exhausted all Subjects");

            <SubjectCount<T>>::put(subject + 1);
            <Subjects<T>>::insert(subject, sender);

            Ok(())
        }

        fn issue_credential(origin, to: T:AccountId, subject: u32) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(<Subjects<T>>::get(subject) == sender, "Unauthorized.");

            let new_cred = Credential {
                subject: subject,
                when: <timestamp::Module<T>>::get(),
                by: sender,
            };

            <Credentials<T>>::insert((&sender, subject), new_cred);

            Ok(())
        }
    }
}
