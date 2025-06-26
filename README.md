# Node Template REST API


# NFT Marketplace server

Backend API for NFT Marketplace project

## Screencast demo
https://youtu.be/pM4YEwt0nHc [unlisted and availible only by derect link]

## Deployment
API available by root URL: http://3.86.155.38:8080/api/v1

## Endpoints
- `/listings` - returns listings from NFT owners
- `/bids` - returns bids from auction participants
- `/settlements` - returns settled deals ready to be executed on chain
## Postman collection
You can use this collection to run requests to server:
https://github.com/alexsanya/nft-market-server/blob/main/nft-marketplace-requests.json
## Features
- Data format validation
- Signatures validation
- On-chain data validation
- Support of pagination
- Ability to specify NFT collection in `/listings` request by providing `collection` parameter
- Ability to specify NFT owner in `/bids` request by providing `owner` parameter
- Fault-tolerance - each entity is independent from it's parents - you can create bid if listing data is no longer stored in redis, you can also create settlement if bid and listing data are removed from redis 

## TODO
- Implement micro-service that will perform regular clean-ups by removing expired entities from redis

## Installation

Clone this repository

```bash
git clone https://github.com/alexsanya/nft-market-server
```

Install dependencies

```bash
yarn
```

Clone `.env.template` file and rename to `.env`.

Replace your environment variables in `.env` file

## Scripts
Since there is no UI you can use script `scripts/settleOnChain.ts` in order to run prepare the wallets of owner and buyer and run settlement transaction

## Running the app

Run `yarn dev`

## Running tests
Run `yarn test`
