#!/bin/bash
source neardev/dev-account.env

ACCOUNT_ID="$CONTRACT_NAME"
OFFER_ID="muzikant.testnet-1"

near view $CONTRACT_NAME escrow_offer --accountId $ACCOUNT_ID "{ \"offer_id\": \"$OFFER_ID\" }"

