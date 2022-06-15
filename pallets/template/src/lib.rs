#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/* Placeholder for defining custom types. */

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a Club has been created. [club_index, owner]
		ClubCreated(u64, T::AccountId),

		/// Event emitted when a member added by owner. [club_index, member]
		MemberAdded(u64, T::AccountId),

		/// Event emitted when a member removed by owner. [club_index, member]
		MemberRemoved(u64, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		ClubAlreadyCreated,

		MemberAlreadyAdded,

		NoSuchMember,

		NotClubOwner,

		NoSuchClub,
	}

	#[pallet::storage]
	/// Maps each club to its owner
	pub(super) type Clubs<T: Config> =
		StorageMap<_, Blake2_128Concat, u64, T::AccountId, OptionQuery>;

	#[pallet::storage]

	/// Maps each (member, club) to its owner
	pub(super) type Members<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u64, T::AccountId>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_club(origin: OriginFor<T>, club: u64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer
			let sender = ensure_signed(origin)?;

			// Verify that the specified club has not been created
			ensure!(!Clubs::<T>::contains_key(&club), Error::<T>::ClubAlreadyCreated);

			// Store the club with the owner
			Clubs::<T>::insert(&club, &sender);

			// Emit an vent that the club was created.
			Self::deposit_event(Event::ClubCreated(club, sender));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn add_member(origin: OriginFor<T>, member: T::AccountId, club: u64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer
			let sender = ensure_signed(origin)?;

			// Verify that the specified club has been created
			ensure!(Clubs::<T>::contains_key(&club), Error::<T>::NoSuchClub);

			// Verify that the specified member has not been added
			ensure!(!Members::<T>::contains_key(&member, &club), Error::<T>::MemberAlreadyAdded);

			// Get owner of the club.
			// Panic condition: there is no way to set a `None` owner, so this must always unwrap.
			let owner = Clubs::<T>::get(club).expect("All clubs must have an owner");

			// Verify that sender of the current call is the club owner.
			ensure!(sender == owner, Error::<T>::NotClubOwner);

			Members::<T>::insert(&member, &club, sender);

			// Emit an vent that the member was added.
			Self::deposit_event(Event::MemberAdded(club, member));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn remove_member(
			origin: OriginFor<T>,
			member: T::AccountId,
			club: u64,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer
			let sender = ensure_signed(origin)?;

			// Verify that the specified club has been created
			ensure!(Clubs::<T>::contains_key(&club), Error::<T>::NoSuchClub);

			// Verify that the specified member has been added
			ensure!(Members::<T>::contains_key(&member, &club), Error::<T>::NoSuchMember);

			// Get owner of the club.
			// Panic condition: there is no way to set a `None` owner, so this must always unwrap.
			let owner = Clubs::<T>::get(club).expect("All clubs must have an owner");

			// Verify that sender of the current call is the club owner.
			ensure!(sender == owner, Error::<T>::NotClubOwner);

			Members::<T>::remove(&member, &club);

			// Emit an vent that the member is removed
			Self::deposit_event(Event::MemberRemoved(club, member));

			Ok(())
		}
	}
}
