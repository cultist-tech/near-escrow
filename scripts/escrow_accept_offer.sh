#!/bin/bash
source neardev/dev-account.env

ACCOUNT_ID="muzikant.testnet"
OFFER_ID="muzikanto.testnet-1"

near call $CONTRACT_NAME escrow_accept_offer --accountId $ACCOUNT_ID "{ \"offer_id\": \"$OFFER_ID\" }"

