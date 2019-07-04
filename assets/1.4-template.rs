// ACTION: Add StorageMap to support mappings
use support::{decl_storage, decl_module, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: balances::Trait + timestamp::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as VerifiableCreds {
        SubjectCount: u32;
        // ACTION: Add a `map` item name `Subjects` from `u32` to `T::AccountId`
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // ACTION: rename this function to `create_subject`, remove it's input value
        fn set_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: read the current `SubjectCount`, save it in the variable `subject`

            // ACTION: update the counter by `subject + 1` rather than `value`
            <SubjectCount<T>>::put(value);
            // ACTION: add an `insert` to the new map `Subjects` for `subject` to `sender`
            Ok(())
        }
    }
}