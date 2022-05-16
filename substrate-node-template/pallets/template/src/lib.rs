#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/* Placeholder for defining custom types. */

	// TODO: Update the `config` block below
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// TODO: Update the `event` block below
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// TODO: Update the `error` block below
	#[pallet::error]
	pub enum Error<T> {}

	// TODO: add #[pallet::storage] block

	// TODO: Update the `call` block below
	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
