# Pharma_Chain

This project is a decentralized platform built on the Internet Computer Protocol (ICP) to track pharmaceutical products through the supply chain. It ensures transparency, accountability, and rewards participants for their involvement.

## Key Features

1. **User Management**
   - **Create User:** Allows the creation of new user profiles with validation for input fields.
   - **Get User by ID:** Retrieves the profile of a user by their unique ID.
   - **Get Users by Role:** Retrieves all users based on their role (Admin, Manufacturer, Distributor, Viewer).

2. **Pharmaceutical Management**
   - **Create Pharmaceutical:** Allows an admin to register a new pharmaceutical product.
   - **Get Pharmaceutical History:** Retrieves the event history of a specific pharmaceutical product.
   - **Get All Pharmaceuticals:** Retrieves all registered pharmaceutical products.

3. **Supply Chain Event Management**
   - **Create Supply Chain Event:** Records a new event in the supply chain process.
   - **Get All Supply Chain Events:** Retrieves all recorded supply chain events.

4. **Reward Management**
   - **Create Reward:** Assigns reward points to participants involved in supply chain events.
   - **Get All Rewards:** Retrieves all rewards assigned to participants.

5. **Error Handling**
   - **Not Found:** Returns an error if a requested resource (user, pharmaceutical, event) is not found.
   - **Unauthorized:** Handles errors related to unauthorized actions.


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```