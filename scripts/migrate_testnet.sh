# Contract migration script
# Run it like this: `zsh ./scripts/migrate_testnet.sh`

# View your keys with `starsd keys list`

export CONTRACT_NAME=pegasus;
export CONTRACT_ADDRESS=stars16jlpdfs658klz0maee8qee3ff7fq5h6sncng39xc5m527hnc84fqxw9wg2;
export KEY_NAME=admin;

export WALLET_DATA=$(starsd keys show $KEY_NAME --output json | jq .);

export KEY_NAME=$(echo $WALLET_DATA | jq -r '.name');
export KEY_TYPE=$(echo $WALLET_DATA | jq -r '.type');
export KEY_ADDRESS=$(echo $WALLET_DATA | jq -r '.address');

echo "\nConnected to wallet '$KEY_NAME'<$KEY_TYPE> @ $KEY_ADDRESS";
echo "\n========\n";

# Instantiate message config
export MIGRATE_MSG="{ }";
echo $MIGRATE_MSG;

## INIT ##
# Get network config
echo "Sourcing network configuration...";

export CHAIN_ID="elgafar-1";
export FEE_DENOM="stars";
export STAKE_DENOM="stars";
export BECH32_HRP="stars";

export RPC="https://rpc.elgafar-1.stargaze-apis.com:443";

# Tx flag configuration
export NODE=(--node $RPC);
export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.25ustars --gas auto --gas-adjustment 1.3);

echo "Network configuration found."

## BUILD ##
# If the architecture is `arm64`, run the arm64 version of rust-optimizer
echo "\n========\n";
echo "Building contract...";

export ARCH='';
export L_ARCH='';

if [[ $(uname -m) -eq 'arm64' ]]
then
  ARCH='-arm64';
  LARCH='-aarch64';
fi

docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer$ARCH:0.12.6;

CONTRACT_NAME=$CONTRACT_NAME$LARCH;

## DEPLOY ##
# Fetch codeids
echo "\n========\n";
echo "Fetching CodeIDs...";
export RES=$(starsd tx wasm store artifacts/$CONTRACT_NAME.wasm --from $KEY_NAME $TXFLAG -y --output json -b block);
export CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value');
echo "CodeID found: $CODE_ID";

# Instantiate the contract
echo "\n========\n";
echo "Instantiating contract...";
starsd tx wasm migrate "$CONTRACT_ADDRESS" $CODE_ID "$MIGRATE_MSG" --from $KEY_NAME --label "$CONTRACT_NAME" $TXFLAG -y --no-admin;
echo "Contract migrated."

# Store contract addr in $CONTRACT
echo "\n========\n";
echo "Fetching contract address...";
sleep 6;
export CONTRACT=$(starsd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]');
echo "Contract address: $fg_bold[green]$CONTRACT";