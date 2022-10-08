#!/bin/bash
source neardev/dev-account.env

ACCOUNT_ID="muzikant.testnet"
RECEIVER_ID="nalogovik.testnet"
#OFFER="{ \"FtToFt\": { \"ft_contract_id_in\": \"mfight-ft.testnet\", \"ft_contract_id_out\": \"mfight-xp.testnet\", \"amount_in\": \"1\", \"amount_out\": \"1\" } }"
OFFER="{ \"NftToFt\": { \"nft_contract_id_in\": \"mfight-nft_v2.testnet\", \"ft_contract_id_out\": \"mfight-xp.testnet\", \"nft_token_id_in\": \"1\", \"amount_out\": \"1\" } }"

near call $CONTRACT_NAME escrow_make_offer --accountId $ACCOUNT_ID "{ \"offer\": $OFFER, \"receiver_id\": \"$RECEIVER_ID\" }"

