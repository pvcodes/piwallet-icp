#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "Starting Token Wallet Tests..."

# Get current identity principal
IDENTITY_PRINCIPAL=$(dfx identity get-principal)
echo "Current identity principal: $IDENTITY_PRINCIPAL"

# Create a test identity
dfx identity new --disable-encryption test_user
dfx identity use test_user
TEST_PRINCIPAL=$(dfx identity get-principal)
echo "Test user principal: $TEST_PRINCIPAL"

# Switch back to default identity
dfx identity use default

# Test 1: Check initial balance
echo -e "\n${GREEN}Test 1: Checking initial balance${NC}"
dfx canister call rust_hello_backend get_balance
echo "Expected: Initial balance should be 1000000 for deployer"

# Test 2: Test minting
echo -e "\n${GREEN}Test 2: Testing mint function${NC}"
dfx canister call rust_hello_backend mint "(principal \"$TEST_PRINCIPAL\", 500)"
echo "Expected: Should mint 500 tokens to test user"

# Test 3: Check test user balance
echo -e "\n${GREEN}Test 3: Checking test user balance${NC}"
dfx identity use test_user
dfx canister call rust_hello_backend get_balance
echo "Expected: Balance should be 500"

# Test 4: Send tokens
echo -e "\n${GREEN}Test 4: Testing send_tokens${NC}"
dfx canister call rust_hello_backend send_tokens "(principal \"$IDENTITY_PRINCIPAL\", 200)"
echo "Expected: Should send 200 tokens to default identity"

# Test 5: Check balances after transfer
echo -e "\n${GREEN}Test 5: Checking balances after transfer${NC}"
echo "Test user balance:"
dfx canister call rust_hello_backend get_balance
dfx identity use default
echo "Default user balance:"
dfx canister call rust_hello_backend get_balance

# Test 6: Test insufficient balance
echo -e "\n${GREEN}Test 6: Testing insufficient balance transfer${NC}"
dfx identity use test_user
dfx canister call rust_hello_backend send_tokens "(principal \"$IDENTITY_PRINCIPAL\", 1000000)"
echo "Expected: Should return insufficient balance error"

# Test 7: Test receive tokens
echo -e "\n${GREEN}Test 7: Testing receive_tokens${NC}"
dfx canister call rust_hello_backend receive_tokens "(principal \"$IDENTITY_PRINCIPAL\", 100)"
echo "Expected: Should receive 100 tokens"

# Test 8: Check if address exists
echo -e "\n${GREEN}Test 8: Testing address_exists${NC}"
dfx canister call rust_hello_backend address_exists "(principal \"$TEST_PRINCIPAL\")"
echo "Expected: Should return true"

# Test 9: Get total supply
echo -e "\n${GREEN}Test 9: Testing get_total_supply${NC}"
dfx canister call rust_hello_backend get_total_supply
echo "Expected: Should return total supply of tokens"

# Clean up
echo -e "\n${GREEN}Cleaning up...${NC}"
dfx identity use default
dfx identity remove test_user

echo -e "\n${GREEN}Testing complete!${NC}"