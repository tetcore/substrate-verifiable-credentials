use support::{decl_storage, decl_module, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait + timestamp::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as VerifiableCreds {
        SubjectCount: u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn set_value(origin, value: u32) -> Result {
            let sender = ensure_signed(origin)?;

            <SubjectCount<T>>::put(value);

            Ok(())
        }
    }
}
