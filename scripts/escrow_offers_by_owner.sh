#!/bin/bash
source neardev/dev-account.env

ACCOUNT_ID="muzikant.testnet"

near view $CONTRACT_NAME escrow_offers_by_owner --accountId $ACCOUNT_ID "{ \"account_id\": \"$ACCOUNT_ID\", \"limit\": 5, \"offset\": \"0\" }"

