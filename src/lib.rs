use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault,
    Promise, PromiseOrValue, BorshStorageKey,
};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedSet};

//

use mfight_sdk::pause::PauseFeature;
use mfight_sdk::owner::OwnerFeature;
use mfight_sdk::blacklist::BlacklistFeature;
use mfight_sdk::escrow::{EscrowFeature, EscrowOfferId, EscrowEnum};

mod ft_callbacks;
mod nft_callbacks;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pause: PauseFeature,
  owner: OwnerFeature,
  blacklist: BlacklistFeature,
  escrow: EscrowFeature,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
  BlacklistAccounts,

  EscrowOffer,
  EscrowOffersToAccount,
  EscrowOffersFromAccount,
  EscrowOwnerByOffer,
  EscrowReceiverByOffer,
  EscrowOfferAccepted,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id,
    )
  }

  #[init]
  pub fn new(owner_id: AccountId) -> Self {
    let this = Self {
      pause: PauseFeature::new(),
      owner: OwnerFeature::new(owner_id.clone()),
      blacklist: BlacklistFeature::new(StorageKey::BlacklistAccounts),

      escrow: EscrowFeature::new(
        StorageKey::EscrowOffer,
        StorageKey::EscrowOffersToAccount,
        StorageKey::EscrowOffersFromAccount,
        StorageKey::EscrowOwnerByOffer,
        StorageKey::EscrowReceiverByOffer,
        StorageKey::EscrowOfferAccepted,
      ),
    };

    this
  }

  fn assert_owner(&self) {
    self.owner.assert_owner();
  }

  fn assert_use(&self) {
    self.blacklist.assert_not_blocked(&env::predecessor_account_id());
  }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldEscrow {
      pub offer_by_id: TreeMap<EscrowOfferId, EscrowEnum>,

      pub offers_by_account: TreeMap<AccountId, UnorderedSet<EscrowOfferId>>,
      pub offers_for_account: TreeMap<AccountId, UnorderedSet<EscrowOfferId>>,

      pub offer_owner_by_account: LookupMap<EscrowOfferId, AccountId>,
      pub offer_receiver_by_account: LookupMap<EscrowOfferId, AccountId>,

      pub offer_accepted_by_id: LookupMap<EscrowOfferId, bool>,
    }

    #[derive(BorshDeserialize)]
    struct Old {
      pause: PauseFeature,
      owner: OwnerFeature,
      blacklist: BlacklistFeature,
      escrow: OldEscrow,
    }

    let old: Old = env::state_read().expect("Error");
    let escrow = EscrowFeature {
      offer_by_id: old.escrow.offer_by_id,
      offers_by_account: old.escrow.offers_by_account,
      offers_for_account: old.escrow.offers_for_account,
      offer_owner_by_account: old.escrow.offer_owner_by_account,
      offer_receiver_by_account: old.escrow.offer_receiver_by_account,
      offer_accepted_by_id: old.escrow.offer_accepted_by_id,
    };

    Self {
      owner: old.owner,
      pause: old.pause,
      blacklist: old.blacklist,
      escrow,
    }
  }
}

mfight_sdk::impl_escrow_core!(Contract, escrow, assert_use);
mfight_sdk::impl_escrow_enumeration!(Contract, escrow);

mfight_sdk::impl_pause_feature!(Contract, pause, assert_owner);
mfight_sdk::impl_owner_feature!(Contract, owner);
mfight_sdk::impl_blacklist_feature!(Contract, blacklist, assert_owner);
