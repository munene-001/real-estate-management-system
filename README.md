# Real Estate Management System Documentation

## Overview
The Real Estate Management System is a Rust-based application designed to manage properties, lease agreements, and tenants. It provides CRUD (Create, Read, Update, Delete) operations for properties, lease agreements, and tenants, allowing users to add, retrieve, update, and delete data related to real estate management.

The system utilizes thread-local storage for memory management, allowing efficient storage and retrieval of data. It provides error handling for cases such as not found errors and invalid inputs to ensure robustness.

## Table of Contents
1. [Dependencies](#dependencies)
2. [Data Structures](#data-structures)
3. [Functions](#functions)
4. [Usage](#usage)

## Dependencies <a name="dependencies"></a>
- `serde`: Serialization and deserialization library for Rust.
- `candid`: Library for Candid serialization and deserialization.
- `ic_stable_structures`: Library providing stable data structures for memory management.
- `std`: Standard library for Rust.

## Data Structures <a name="data-structures"></a>
### Structs
- `Property`: Represents a real estate property with fields such as ID, address, property type, and description.
- `LeaseAgreement`: Represents a lease agreement with fields including ID, property ID, tenant ID, start date, and end date.
- `Tenant`: Represents a tenant with fields such as ID and name.

### Enums
- `PropertyType`: Represents the possible types of properties including House, Apartment, and Commercial.

## Functions <a name="functions"></a>
The Real Estate Management System provides various functions for managing properties, lease agreements, and tenants. Some key functions include:
- `add_property`: Add a new property to the system.
- `delete_property`: Delete a property from the system.
- `create_lease_agreement`: Create a new lease agreement.
- `cancel_lease_agreement`: Cancel an existing lease agreement.
- `add_tenant`: Add a new tenant to the system.
- `delete_tenant`: Delete a tenant from the system.
- `get_property`: Retrieve a property by its ID.
- `get_lease_agreement`: Retrieve a lease agreement by its ID.
- `get_tenant`: Retrieve a tenant by its ID.
- `update_property`: Update an existing property.
- `update_lease_agreement`: Update an existing lease agreement.
- `update_tenant`: Update an existing tenant.

## Usage <a name="usage"></a>
The Real Estate Management System offers a user-friendly interface for adding, retrieving, updating, and deleting properties, lease agreements, and tenants. Users can interact with the system through function calls to perform desired operations.

Proper error handling is implemented to handle cases such as not found errors and invalid inputs, ensuring the reliability and stability of the system.

1. **Install Rust and Dependencies**
   - Ensure you have Rust installed, version 1.64 or higher. You can install it using the following commands:
     ```bash
     $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     $ source "$HOME/.cargo/env"
     ```
   - Install the `wasm32-unknown-unknown` target:
     ```bash
     $ rustup target add wasm32-unknown-unknown
     ```
   - Install `candid-extractor`:
     ```bash
     $ cargo install candid-extractor
     ```

2. **Install DFINITY SDK (`dfx`)**
   - Install `dfx` using the following commands:
     ```bash
     $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
     $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
     $ source ~/.bashrc
     $ dfx start --background
     ```

3. **Update Dependencies**
   - Update the `dependencies` block in `/src/{canister_name}/Cargo.toml` with the required dependencies.

4. **Autogenerate DID**
   - Add the provided script to the root directory of the project.
   - Update line 16 with the name of your canister.
   - Run the script each time you modify/add/remove exported functions of the canister.

5. **Running the Project Locally**
   - Start the replica, running in the background:
     ```bash
     $ dfx start --background
     ```
   - Deploy your canisters to the replica and generate your Candid interface:
     ```bash
     $ npm run gen-deploy
     ```