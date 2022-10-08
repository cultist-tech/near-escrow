# Escrow

Возможность обменять NFT на FT, NFT на NFT, FT на FT вне специализированных площадок с определенным пользователем. Это даст возможность производить действия с NFT/FT, даже если их нет на площадка (не залистили), либо если ты не хочешь чтобы NFT/FT попали в другие руки.


```rust
pub trait EscrowCore {
  // вывести ft токен
  fn escrow_withdraw_ft(&mut self, ft_token_id: AccountId, amount: U128);
  // вывести nft токен
  fn escrow_withdraw_nft(&mut self, nft_token_id: AccountId, token_id: TokenId);

  // предложить обмен (ft -> ft, nft -> ft, ft -> nft, nft -> nft)
  fn escrow_make_offer(&mut self, offer: EscrowEnum, receiver_id: AccountId) -> JsonEscrow;
  // принять обмен
  fn escrow_accept_offer(&mut self, offer_id: OfferId);

  // просмотр информации об сделке
  fn escrow_offer(&self, offer_id: OfferId) -> Option<JsonEscrow>;
  // просмотр списока сделок созданных пользователем
  fn escrow_offers_by_owner(&self, account_id: AccountId, limit: Option<u64>, offset: Option<U128>) -> Vec<JsonEscrow>;
  // просмотр списка сделак направленных пользователю
  fn escrow_offers_for_owner(&self, account_id: AccountId, limit: Option<u64>, offset: Option<U128>) -> Vec<JsonEscrow>;
  // кол-во актуальных сделок от пользователя
  fn escrow_offers_total_by_owner(&self, account_id: AccountId) -> u64;
  // кол-во актуальных сделок направленных пользователю
  fn escrow_offers_total_for_owner(&self, account_id: AccountId) -> u64;
}

pub trait NonFungibleTokenApprovalsReceiver {
  // передача nft токена в контракт обмена
  fn nft_on_transfer(
    &mut self,
    sender_id: AccountId,
    previous_owner_id: AccountId,
    token_id: TokenId,
    msg: String,
  ) -> PromiseOrValue<bool>;
}

trait FungibleTokenReceiver {
  // передача ft токена в контракт обмена
  fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}
```
