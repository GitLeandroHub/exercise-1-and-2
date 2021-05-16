
pub struct Kitty { //d. Users can create kitties
    id: u64, //Kittyindex //Kitty_id //d. Users can create kitties
	dna: u128, [u8; 16] //a. Kitty must have a 128 bit DNA, which is randomly generated
	owner: AccountId; //c. A user can have zero or more kitties
}

a. Kitty must have a 128 bit DNA, which is randomly generated
b. Kitty must have one owner

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
	}

	fn insert_owned_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex) {
		<OwnedKittiesList<T>>::append(owner, kitty_id);
	}

	fn mint_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
		// Create and store kitty
		<Kitties<T>>::insert(kitty_id, kitty);
		<KittiesCount<T>>::put(kitty_id + One::one());
		<KittyOwners<T>>::insert(kitty_id, owner.clone());

		Self::insert_owned_kitty(owner, kitty_id);
	}

	
	let owned_kitty_count = Self::owned_kitty_count(&to);

	let new_owned_kitty_count = owned_kitty_count.checked_add(1)
		.ok_or("Overflow adding a new kitty to account balance")?;
}

	
c. A user can have zero or more kitties

	decl_storage! {
		trait Store for Module<T: Trait> as KittyStorage {
			Kitties get(kitty): map T::KittyIndex => Kitty<T::KittyIndex, T::Balance>;
			KittyOwner get(owner_of): map T::KittyIndex => Option<T::AccountId>;
	
			// list of kitty in our runtime represented by T::Hash
			AllKittiesArray get(kitty_by_index): map u64 => T::KittyIndex;
			AllKittiesCount get(all_kitties_count): u64;
			//little extra storage to keep track of each item and its position in our list
			AllKittiesIndex: map T::KittyIndex => u64;
	
			//kitty list unique to each person
			OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::KittyIndex;
			OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
			
			//kitties all have unique identifiers
			//OwnedKittiesIndex: map (T::AccountId, T::Hash) => u64;
			OwnedKittiesIndex: map T::KittyIndex => u64;
			
			Nonce: u64;
		}
	}


	d. Users can create kitties

	decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {
			fn deposit_event() = default;
	
			fn create_kitty(origin) -> Result {
				let sender = ensure_signed(origin)?;
				let kitty_id = Self::next_kitty_id()?;
	
				// Generate a random 128bit value
				let dna = Self::random_value(&sender);
				ensure!(!<Kitties<T>>::exists(Kitty_id), "This id is already exists");
	
				// Create and store kitty
				let new_kitty = Kitty(dna);
				Self::mint_kitty(&sender, kitty_id, new_kitty); //&sender and sender ";?"
	
				Self::deposit_event(RawEvent::Created(sender, kitty_id))
			}
		}
	}

	decl_event! {
		pub enum Event<T>
		where
			<T as system::Trait>::AccountId,
			<T as system::Trait>::Kittyindex,
		{
			//KittyCreated (owner, kitty_id)
			Created(AccountId, KittyIndex),
		}
	}