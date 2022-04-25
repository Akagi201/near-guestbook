# near-guestbook

## Build Contract

```sh
make contract
```

## Deploy and Init Contract

```sh
near create-account nearguestbook.akagi201.testnet --masterAccount akagi201.testnet
make deploy-contract
near call nearguestbook.akagi201.testnet new_default_meta '{"owner_id": "nearguestbook.akagi201.testnet"}' --accountId nearguestbook.akagi201.testnet
```

## Build frontend

```sh
make frontend
```

## Deploy frontend

```sh
make deploy-frontend
```

## Start local frontend

```sh
make start-frontend
```
