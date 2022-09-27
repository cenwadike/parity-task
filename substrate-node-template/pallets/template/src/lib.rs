//! # Patient Biodata
//!
//! ## Overview
//!
//! This pallet supports creating record of patient data.  It also allow patients
//! to grant or revoke access to their biodata
//!
//! ## Interface
//!
//! ### Config
//!
//! ### Dispatchable functions
//!
//! * `create_new_record(orgin, name, age, sex)` - Create a new patient record
//! * `grant_access(orgin, new_access_id, record_id)` - Grant access to patient record to
//!   new_access_id. Only patient can grant access
//! * `grant_access(origin, access_id, record_id)` - Revoke access to patient record from access_id.
//!   Only patient can revoke access

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{inherent::Vec, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// pallet config
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// ubiquitous event trait
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[derive(Clone, Eq, PartialEq, Default, RuntimeDebug, Encode, Decode)]
	pub struct PatientBiodata<AccountId> {
		pub patient_id: AccountId,
		pub name: Vec<u8>,
		pub sex: Vec<u8>,
		pub age: u16,
		pub record_id: u64,
		pub access: Vec<AccountId>,
	}

	// type alias
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type BiodataOf<T> = PatientBiodata<AccountIdOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn recordId)]
	pub type RecordId<T> = StorageValue<_, u64>;

	/// The lookup table for patients biodata. patient_id -> PatientBiodata
	#[pallet::storage]
	#[pallet::getter(fn biodata)]
	pub(super) type Biodata<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BiodataOf<AccountIdOf<T>>, ValueQuery>;

	// pallet events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// new patient record is created [patient_id, record_id]
		NewRecordCreated(T::AccountId, u64),

		/// access granted to patient record [patient_id, access_id, record_id]
		NewAccessGranted(T::AccountId, T::AccountId, u64),

		/// access revoked from patient record [patient_id, access_id, record_id]
		AccessRevoked(T::AccountId, T::AccountId, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// only patient can grant or revoke access to their record
		PermissionDenied,
		/// record does not exist
		RecordDoesNotExist,
		/// access already exist
		AccessExist,
		/// access does not exist
		AccessDoesNotExist,
		/// storage overflow for patient record
		StorageOverflow,
	}

	// pallet dispatchables
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// create_new_record(patient_id: OriginFor<T>, name: String, sex: String, age:u16)
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_new_record(
			origin: OriginFor<T>,
			name: Vec<u8>,
			sex: Vec<u8>,
			age: u16,
		) -> DispatchResult {
			let patient_id = ensure_signed(origin)?;

			let record_id;

			//get and update records id
			match <RecordId<T>>::get() {
				None => <RecordId<T>>::put(1),
				Some(record_id) => {
					// increment record_id
					let new_record_id = record_id.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					<RecordId<T>>::put(new_record_id)
				},
			}

			// add new patient record to storage
			let pt_biodata =
				PatientBiodata { patient_id, name, sex, age, record_id, access: Vec::new() };
			<Biodata<T>>::insert(&record_id, &pt_biodata);

			Self::deposit_event(Event::NewRecordCreated(patient_id, record_id));
			Ok(())
		}

		/// grant_access(patient_id: OriginFor<T>, access_id: AccountId, record_id: u64)
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn grant_access(
			origin: OriginFor<T>,
			access_id: AccountIdOf<T>,
			record_id: u64,
		) -> DispatchResult {
			let patient_id = ensure_signed(origin)?;

			// ensure signer is patient
			let is_patient = <Biodata<T>>::get(&record_id).patient_id;
			ensure!(is_patient == patient_id, <Error<T>>::PermissionDenied);

			// check if record exist
			let record_exist = <Biodata<T>>::contains_key(&record_id);
			ensure!(record_exist, <Error<T>>::RecordDoesNotExist);

			// check if access already granted
			let access_exist = <Biodata<T>>::get(&record_id).access.contains(&record_id);
			ensure!(!access_exist, <Error<T>>::AccessExist);

			// add new access id
			<Biodata<T>>::get(&record_id).access.push(access_id);

			Self::deposit_event(Event::NewAccessGranted(patient_id, access_id, record_id));
			Ok(())
		}

		/// revoke_access(patient_idaccess_id: AccountId, record_id: u64)
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn revoke_access(
			origin: OriginFor<T>,
			access_id: AccountIdOf<T>,
			record_id: u64,
		) -> DispatchResult {
			let patient_id = ensure_signed(origin)?;

			// ensure signer is patient
			let is_patient = <Biodata<T>>::get(&record_id).patient_id;
			ensure!(is_patient == patient_id, <Error<T>>::PermissionDenied);

			// check if record exist
			let record_exist = <Biodata<T>>::contains_key(&record_id);
			ensure!(record_exist, <Error<T>>::RecordDoesNotExist);

			// check if access exist
			let access_exist = <Biodata<T>>::get(&record_id).access.contains(&record_id);
			ensure!(access_exist, <Error<T>>::AccessDoesNotExist);

			// add new access id
			<Biodata<T>>::get(&record_id).access.pop(access_id);

			Self::deposit_event(Event::AccessRevoked(patient_id, access_id, record_id));
			Ok(())
		}
	}
}
