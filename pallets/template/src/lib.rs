//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        // fn ed() -> Balance;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    pub type Balance = u128;

    #[pallet::storage]
    pub type TotalIssuance<T: Config> = StorageValue<_, Balance>;

    #[pallet::storage]
    pub type Balances<T: Config> = StorageMap<Key = T::AccountId, Value = Balance>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An unsafe mint that can be called by anyone. Not a great idea.
        pub fn mint_unsafe(
            origin: T::RuntimeOrigin,
            dest: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            // ensure that this is a signed account, but we don't really check `_anyone`.
            let _anyone = ensure_signed(origin)?;

            // Check if anyone has balance
            if Balances::<T>::contains_key(_anyone) {
                return Err("Anyone has balance".into());
            }

            // update the balances map. Notice how all `<T: Config>` remains as `<T>`.
            Balances::<T>::mutate(dest, |b| *b = Some(b.unwrap_or(0) + amount));
            // update total issuance.
            TotalIssuance::<T>::mutate(|t| *t = Some(t.unwrap_or(0) + amount));

            Ok(())
        }

        /// Transfer `amount` from `origin` to `dest`.
        pub fn transfer(
            origin: T::RuntimeOrigin,
            dest: T::AccountId,
            amount: Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // ensure sender has enough balance, and if so, calculate what is left after `amount`.
            let sender_balance = Balances::<T>::get(&sender).ok_or("NonExistentAccount")?;
            let reminder = sender_balance
                .checked_sub(amount)
                .ok_or("InsufficientBalance")?;

            // update sender and dest balances.
            Balances::<T>::mutate(dest, |b| *b = Some(b.unwrap_or(0) + amount));
            Balances::<T>::insert(&sender, reminder);

            Ok(())
        }
    }
}

// tests for pallet
#[cfg(test)]
mod tests {
    use super::pallet as currency_pallet;
    use super::pallet::*;
    use frame::testing_prelude::*;

    construct_runtime!(
        pub struct Runtime {
            System: frame_system,
            Currency: currency_pallet,
        }
    );

    #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    impl frame_system::Config for Runtime {
        type Block = MockBlock<Runtime>;
        // within pallet we just said `<T as frame_system::Config>::AccountId`, now we
        // finally specified it.
        type AccountId = u64;
    }

    impl currency_pallet::Config for Runtime {
        // fn ed() -> crate::Balance {
        //     5
        // }
    }

    #[test]
    fn mint_unsafe_works() {
        TestState::new_empty().execute_with(|| {
            // currency_pallet::Balance::<Runtime>::insert(&42u64, 100);

            let _ =
                currency_pallet::Pallet::<Runtime>::mint_unsafe(RuntimeOrigin::signed(1), 50, 100);

            assert_eq!(currency_pallet::Balances::<Runtime>::get(50), Some(100));
        });
    }
}
