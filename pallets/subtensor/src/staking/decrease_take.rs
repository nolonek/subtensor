use super::*;
use frame_support::{
    storage::IterableStorageDoubleMap,
    traits::{
        tokens::{
            fungible::{Balanced as _, Inspect as _, Mutate as _},
            Fortitude, Precision, Preservation,
        },
        Imbalance,
    },
};

impl<T: Config> Pallet<T> {
    /// ---- The implementation for the extrinsic decrease_take
    ///
    /// # Args:
    /// * 'origin': (<T as frame_system::Config>::RuntimeOrigin):
    ///     - The signature of the caller's coldkey.
    ///
    /// * 'hotkey' (T::AccountId):
    ///     - The hotkey we are delegating (must be owned by the coldkey.)
    ///
    /// * 'take' (u16):
    ///     - The stake proportion that this hotkey takes from delegations for subnet ID.
    ///
    /// # Event:
    /// * TakeDecreased;
    ///     - On successfully setting a decreased take for this hotkey.
    ///
    /// # Raises:
    /// * 'NotRegistered':
    ///     - The hotkey we are delegating is not registered on the network.
    ///
    /// * 'NonAssociatedColdKey':
    ///     - The hotkey we are delegating is not owned by the calling coldket.
    ///
    /// * 'DelegateTakeTooLow':
    ///     - The delegate is setting a take which is not lower than the previous.
    ///
    pub fn do_decrease_take(
        origin: T::RuntimeOrigin,
        hotkey: T::AccountId,
        take: u16,
    ) -> dispatch::DispatchResult {
        // --- 1. We check the coldkey signature.
        let coldkey = ensure_signed(origin)?;
        log::info!(
            "do_decrease_take( origin:{:?} hotkey:{:?}, take:{:?} )",
            coldkey,
            hotkey,
            take
        );

        // --- 2. Ensure we are delegating a known key.
        //        Ensure that the coldkey is the owner.
        Self::do_take_checks(&coldkey, &hotkey)?;

        // --- 3. Ensure we are always strictly decreasing, never increasing take
        if let Ok(current_take) = Delegates::<T>::try_get(&hotkey) {
            ensure!(take < current_take, Error::<T>::DelegateTakeTooLow);
        }

        // --- 3.1 Ensure take is within the min ..= InitialDefaultTake (18%) range
        let min_take = MinTake::<T>::get();
        ensure!(take >= min_take, Error::<T>::DelegateTakeTooLow);

        // --- 4. Set the new take value.
        Delegates::<T>::insert(hotkey.clone(), take);

        // --- 5. Emit the take value.
        log::info!(
            "TakeDecreased( coldkey:{:?}, hotkey:{:?}, take:{:?} )",
            coldkey,
            hotkey,
            take
        );
        Self::deposit_event(Event::TakeDecreased(coldkey, hotkey, take));

        // --- 6. Ok and return.
        Ok(())
    }
}
