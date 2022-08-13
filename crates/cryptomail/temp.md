# DB Design

## Column Families





## Infura API usage syntax

curl \
https://mainnet.infura.io/v3/[project_id] \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_blockNumber","params": [],"id":1}'


curl https://mainnet.infura.io/v3/f35e500f9a9949539ba4b3cf375bb1d8 \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_getTransactionByHash","params": ["0xbb3a336e3f823ec18197f1e13ee875700f08f03e2cab75f0d0b118dabb44cba0"],"id":1}'

## Verifying Deposit
- When new msg with tx is added - add it to collection of messages to check deposits for. e.g ("pending_verification", Array(msg_ids)). Peridoically (every 10 minutes try to verify deposit for all pendingin messages )