#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define type aliases for memory management
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the structure for a property
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Property {
    id: u64,
    address: String,
    property_type: PropertyType,
    description: String,
    // Add any other relevant fields for the property
}

// Define the possible types of properties
#[derive(Debug, PartialEq, candid::CandidType, Deserialize, Serialize, Clone)]
enum PropertyType {
    House,
    Apartment,
    Commercial,
}

// Define the structure for a lease agreement
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct LeaseAgreement {
    id: u64,
    property_id: u64,
    tenant_id: u64,
    start_date: u64, // Assuming start_date is a Unix timestamp
    end_date: u64,   // Assuming end_date is a Unix timestamp
}

// Define the structure for a tenant
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Tenant {
    id: u64,
    name: String,
    // Add any other relevant fields for the tenant
}

// Implement serialization and deserialization for Property
impl Storable for Property {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for Property serialization
impl BoundedStorable for Property {
    const MAX_SIZE: u32 = 1024; // Maximum size in bytes
    const IS_FIXED_SIZE: bool = false; // Indicate if the size is fixed or variable
}

// Implement serialization and deserialization for LeaseAgreement
impl Storable for LeaseAgreement {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }
    
        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }
    }

// Implement bounds for LeaseAgreement serialization
impl BoundedStorable for LeaseAgreement {
    const MAX_SIZE: u32 = 1024; // Maximum size in bytes
    const IS_FIXED_SIZE: bool = false; // Indicate if the size is fixed or variable
}

// Implement serialization and deserialization for Tenant
impl Storable for Tenant {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for Tenant serialization
impl BoundedStorable for Tenant {
    const MAX_SIZE: u32 = 1024; // Maximum size in bytes
    const IS_FIXED_SIZE: bool = false; // Indicate if the size is fixed or variable
}

// Thread-local storage for memory management, ID counter, property storage, lease agreement storage, and tenant storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PROPERTY_STORAGE: RefCell<StableBTreeMap<u64, Property, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static LEASE_AGREEMENT_STORAGE: RefCell<StableBTreeMap<u64, LeaseAgreement, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static TENANT_STORAGE: RefCell<StableBTreeMap<u64, Tenant, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

// Define the possible errors
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

// Implement CRUD operations for properties
#[ic_cdk::update]
fn add_property(address: String, property_type: PropertyType, description: String) -> Result<Property, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let property = Property {
        id,
        address,
        property_type,
        description,
    };

    PROPERTY_STORAGE.with(|storage| storage.borrow_mut().insert(id, property.clone()));
    Ok(property)
}

#[ic_cdk::update]
fn delete_property(id: u64) -> Result<(), Error> {
    match PROPERTY_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Property with id={} not found", id),
        }),
    }
}

// Implement CRUD operations for lease agreements
#[ic_cdk::update]
fn create_lease_agreement(property_id: u64, tenant_id: u64, start_date: u64, end_date: u64) -> Result<LeaseAgreement, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let lease_agreement = LeaseAgreement {
        id,
        property_id,
        tenant_id,
        start_date,
        end_date,
    };

    LEASE_AGREEMENT_STORAGE.with(|storage| storage.borrow_mut().insert(id, lease_agreement.clone()));
    Ok(lease_agreement)
}

#[ic_cdk::update]
fn cancel_lease_agreement(id: u64) -> Result<(), Error> {
    match LEASE_AGREEMENT_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Lease agreement with id={} not found", id),
        }),
    }
}

// Implement CRUD operations for tenants
#[ic_cdk::update]
fn add_tenant(name: String) -> Result<Tenant, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let tenant = Tenant {
        id,
        name,
    };

    TENANT_STORAGE.with(|storage| storage.borrow_mut().insert(id, tenant.clone()));
    Ok(tenant)
}

#[ic_cdk::update]
fn delete_tenant(id: u64) -> Result<(), Error> {
    match TENANT_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Tenant with id={} not found", id),
        }),
    }
}

// Implement query operations for the real estate management system
#[ic_cdk::query]
fn get_property(id: u64) -> Result<Property, Error> {
    match PROPERTY_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(property) => Ok(property.clone()),
        None => Err(Error::NotFound {
            msg: format!("Property with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_lease_agreement(id: u64) -> Result<LeaseAgreement, Error> {
    match LEASE_AGREEMENT_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(lease_agreement) => Ok(lease_agreement.clone()),
        None => Err(Error::NotFound {
            msg: format!("Lease agreement with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_tenant(id: u64) -> Result<Tenant, Error> {
    match TENANT_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(tenant) => Ok(tenant.clone()),
        None => Err(Error::NotFound {
            msg: format!("Tenant with id={} not found", id),
        }),
    }
}

// Implement update operation for properties
#[ic_cdk::update]
fn update_property(id: u64, address: String, property_type: PropertyType, description: String) -> Result<Property, Error> {
    match PROPERTY_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(property) = storage.get(&id) {
            // Create a cloned copy of the property to update
            let mut updated_property = property.clone();
            // Update the property fields
            updated_property.address = address;
            updated_property.property_type = property_type;
            updated_property.description = description;
            // Replace the old property with the updated one
            storage.insert(id, updated_property.clone());
            Ok(updated_property)
        } else {
            Err(Error::NotFound {
                msg: format!("Property with id={} not found", id),
            })
        }
    }) {
        Ok(property) => Ok(property),
        Err(e) => Err(e),
    }
}

// Implement update operation for lease agreements
#[ic_cdk::update]
fn update_lease_agreement(
    id: u64,
    property_id: u64,
    tenant_id: u64,
    start_date: u64,
    end_date: u64,
) -> Result<LeaseAgreement, Error> {
    match LEASE_AGREEMENT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(lease_agreement) = storage.get(&id) {
            // Create a cloned copy of the lease agreement to update
            let mut updated_lease_agreement = lease_agreement.clone();
            // Update the lease agreement fields
            updated_lease_agreement.property_id = property_id;
            updated_lease_agreement.tenant_id = tenant_id;
            updated_lease_agreement.start_date = start_date;
            updated_lease_agreement.end_date = end_date;
            // Replace the old lease agreement with the updated one
            storage.insert(id, updated_lease_agreement.clone());
            Ok(updated_lease_agreement)
        } else {
            Err(Error::NotFound {
                msg: format!("Lease agreement with id={} not found", id),
            })
        }
    }) {
        Ok(lease_agreement) => Ok(lease_agreement),
        Err(e) => Err(e),
    }
}

// Implement update operation for tenants
#[ic_cdk::update]
fn update_tenant(id: u64, name: String) -> Result<Tenant, Error> {
    match TENANT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(tenant) = storage.get(&id) {
            // Create a cloned copy of the tenant to update
            let mut updated_tenant = tenant.clone();
            // Update the tenant fields
            updated_tenant.name = name;
            // Replace the old tenant with the updated one
            storage.insert(id, updated_tenant.clone());
            Ok(updated_tenant)
        } else {
            Err(Error::NotFound {
                msg: format!("Tenant with id={} not found", id),
            })
        }
    }) {
        Ok(tenant) => Ok(tenant),
        Err(e) => Err(e),
    }
}

// Export the Candid interface
ic_cdk::export_candid!();