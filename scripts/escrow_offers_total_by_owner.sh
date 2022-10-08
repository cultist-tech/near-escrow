#!/bin/bash
source neardev/dev-account.env

ACCOUNT_ID="nalogovik.testnet"

near view $CONTRACT_NAME escrow_offers_total_by_owner --accountId $ACCOUNT_ID "{ \"account_id\": \"$ACCOUNT_ID\" }"

