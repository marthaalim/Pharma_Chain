#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// UserRole Enum
#[derive(candid::CandidType, Serialize, Deserialize, PartialEq, Clone, Debug)]
enum UserRole {
    Admin,
    Manufacturer,
    Distributor,
    Viewer,
}

// SupplyChainEventType Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum SupplyChainEventType {
    #[default]
    Production,
    Packaging,
    Storage,
    Transportation,
    Delivery,
}

// RewardType Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum RewardType {
    #[default]
    SupplyChainEvent,
    Other,
}

// User Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone, Debug)]
struct User {
    id: u64,
    username: String,
    role: UserRole,
}

// Pharmaceutical Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default, Debug)]
struct Pharmaceutical {
    id: u64,
    user_id: u64,
    name: String,
    manufacturer: String,
    batch_number: String,
    expiry_date: u64,
}

// SupplyChainEvent Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default, Debug)]
struct SupplyChainEvent {
    id: u64,
    pharmaceutical_id: u64,
    event_type: SupplyChainEventType,
    location: String,
    date: u64,
    participant: String,
}

// Reward Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone, Default, Debug)]
struct Reward {
    id: u64,
    participant: String,
    points: u32,
    reward_type: RewardType,
}

// Payload Definitions

// User Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    username: String,
    role: UserRole,
}

// Pharmaceutical Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct PharmaceuticalPayload {
    name: String,
    user_id: u64,
    manufacturer: String,
    batch_number: String,
    expiry_date: u64,
}

// SupplyChainEvent Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct SupplyChainEventPayload {
    pharmaceutical_id: u64,
    event_type: SupplyChainEventType,
    location: String,
    participant: String,
}

// Reward Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct RewardPayload {
    participant: String,
    points: u32,
    reward_type: RewardType,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PHARMACEUTICALS_STORAGE: RefCell<StableBTreeMap<u64, Pharmaceutical, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static SUPPLY_CHAIN_EVENTS_STORAGE: RefCell<StableBTreeMap<u64, SupplyChainEvent, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static REWARDS_STORAGE: RefCell<StableBTreeMap<u64, Reward, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Pharmaceutical {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Pharmaceutical {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for SupplyChainEvent {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SupplyChainEvent {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Reward {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Reward {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Helper function to increment ID
fn increment_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value + 1
    })
}

// Function to create a new user
#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, String> {
    // Validate the user input to ensure all fields are provided
    if payload.username.is_empty() {
        return Err("Username is required".to_string());
    }

    // Generate a new ID for the user
    let id = increment_id();

    // Create a new user
    let user = User {
        id,
        username: payload.username,
        role: payload.role,
    };

    USERS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, user.clone());
    });

    Ok(user)
}

// Function to update a user's role
#[ic_cdk::update]
fn update_user_role(user_id: u64, role: UserRole) -> Result<User, String> {
    USERS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut user) = storage.remove(&user_id) {
            user.role = role.clone();
            storage.insert(user_id, user.clone());
            Ok(user)
        } else {
            Err("User not found".to_string())
        }
    })
}

// Function to delete a user
#[ic_cdk::update]
fn delete_user(user_id: u64) -> Result<(), String> {
    USERS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&user_id).is_some() {
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    })
}

// Function to fetch a user by id
#[ic_cdk::query]
fn get_user_by_id(user_id: u64) -> Result<User, Error> {
    USERS_STORAGE.with(|storage| match storage.borrow().get(&user_id) {
        Some(user) => Ok(user.clone()),
        None => Err(Error::NotFound {
            msg: "User not found.".to_string(),
        }),
    })
}

// Function to fetch users by role
#[ic_cdk::query]
fn get_users_by_role(role: UserRole) -> Result<Vec<User>, Error> {
    let users = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, user)| user.role == role)
            .map(|(_, user)| user.clone())
            .collect::<Vec<_>>()
    });

    if users.is_empty() {
        Err(Error::NotFound {
            msg: "No users found with the specified role.".to_string(),
        })
    } else {
        Ok(users)
    }
}

// Function to create a new pharmaceutical
#[ic_cdk::update]
fn create_pharmaceutical(payload: PharmaceuticalPayload) -> Result<Pharmaceutical, String> {
    // Validate the user input to ensure all fields are provided
    if payload.name.is_empty()
        || payload.manufacturer.is_empty()
        || payload.batch_number.is_empty()
        || payload.expiry_date == 0
    {
        return Err("All fields are required".to_string());
    }

    // Validate the user id to ensure the user is an admin
    let user_is_admin = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, u)| u.id == payload.user_id && u.role == UserRole::Admin)
    });

    if !user_is_admin {
        return Err("Only admins can create pharmaceuticals".to_string());
    }

    // Ensure the expiry date is in the future
    // if payload.expiry_date < time() {
    //     return Err("Expiry date must be in the future".to_string());
    // }

    // Generate a new ID for the pharmaceutical
    let id = increment_id();

    // Create a new pharmaceutical
    let pharmaceutical = Pharmaceutical {
        id,
        user_id: payload.user_id,
        name: payload.name,
        manufacturer: payload.manufacturer,
        batch_number: payload.batch_number,
        expiry_date: payload.expiry_date,
    };

    PHARMACEUTICALS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, pharmaceutical.clone());
    });

    Ok(pharmaceutical)
}

// Function to delete a pharmaceutical
#[ic_cdk::update]
fn delete_pharmaceutical(id: u64) -> Result<(), String> {
    PHARMACEUTICALS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err("Pharmaceutical not found".to_string())
        }
    })
}

// Function to fetch a pharmaceutical by id
#[ic_cdk::query]
fn get_pharmaceutical_by_id(id: u64) -> Result<Pharmaceutical, Error> {
    PHARMACEUTICALS_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(pharmaceutical) => Ok(pharmaceutical.clone()),
        None => Err(Error::NotFound {
            msg: "Pharmaceutical not found.".to_string(),
        }),
    })
}

// Function to create a new supply chain event
#[ic_cdk::update]
fn create_supply_chain_event(payload: SupplyChainEventPayload) -> Result<SupplyChainEvent, String> {
    // Validate the user input to ensure all fields are provided
    if payload.location.is_empty() || payload.participant.is_empty() {
        return Err("All fields are required".to_string());
    }

    // Validate the pharmaceutical ID to ensure it exists
    let pharmaceutical_exists = PHARMACEUTICALS_STORAGE
        .with(|storage| storage.borrow().contains_key(&payload.pharmaceutical_id));

    if !pharmaceutical_exists {
        return Err("Pharmaceutical with the provided ID does not exist.".to_string());
    }

    // Generate a new ID for the supply chain event
    let id = increment_id();

    // Create a new supply chain event
    let event = SupplyChainEvent {
        id,
        pharmaceutical_id: payload.pharmaceutical_id,
        event_type: payload.event_type,
        location: payload.location,
        date: time(),
        participant: payload.participant.clone(),
    };

    // Store the event
    SUPPLY_CHAIN_EVENTS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, event.clone());
    });

    // Add reward points for the participant
    let points = 10; // Example point allocation
    let reward = Reward {
        id: increment_id(),
        participant: payload.participant.clone(),
        points,
        reward_type: RewardType::SupplyChainEvent,
    };

    REWARDS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(reward.id, reward.clone());
    });

    Ok(event)
}

// Function to delete a supply chain event
#[ic_cdk::update]
fn delete_supply_chain_event(id: u64) -> Result<(), String> {
    SUPPLY_CHAIN_EVENTS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err("Supply chain event not found".to_string())
        }
    })
}

// Function to create a new reward
#[ic_cdk::update]
fn create_reward(payload: RewardPayload) -> Result<Reward, String> {
    // Validate the user input to ensure all fields are provided
    if payload.participant.is_empty() || payload.points == 0 {
        return Err("All fields are required".to_string());
    }

    // Generate a new ID for the reward
    let id = increment_id();

    // Create a new reward
    let reward = Reward {
        id,
        participant: payload.participant.clone(),
        points: payload.points,
        reward_type: payload.reward_type.clone(),
    };

    REWARDS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, reward.clone());
    });

    Ok(reward)
}

// Function to delete a reward
#[ic_cdk::update]
fn delete_reward(id: u64) -> Result<(), String> {
    REWARDS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err("Reward not found".to_string())
        }
    })
}

// Function to retrieve the event history of a pharmaceutical
#[ic_cdk::query]
fn get_pharmaceutical_history(pharmaceutical_id: u64) -> Result<Vec<SupplyChainEvent>, String> {
    let events = SUPPLY_CHAIN_EVENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, e)| e.pharmaceutical_id == pharmaceutical_id)
            .map(|(_, e)| e.clone())
            .collect::<Vec<_>>()
    });

    if events.is_empty() {
        Err("No events found for the provided pharmaceutical ID.".to_string())
    } else {
        Ok(events)
    }
}

// Function to retrieve all pharmaceuticals
#[ic_cdk::query]
fn get_all_pharmaceuticals() -> Result<Vec<Pharmaceutical>, String> {
    let pharmaceuticals = PHARMACEUTICALS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, p)| p.clone())
            .collect::<Vec<_>>()
    });

    if pharmaceuticals.is_empty() {
        Err("No pharmaceuticals found.".to_string())
    } else {
        Ok(pharmaceuticals)
    }
}

// Function to retrieve all supply chain events
#[ic_cdk::query]
fn get_all_supply_chain_events() -> Result<Vec<SupplyChainEvent>, String> {
    let events = SUPPLY_CHAIN_EVENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, e)| e.clone())
            .collect::<Vec<_>>()
    });

    if events.is_empty() {
        Err("No supply chain events found.".to_string())
    } else {
        Ok(events)
    }
}

// Function to retrieve all rewards
#[ic_cdk::query]
fn get_all_rewards() -> Result<Vec<Reward>, String> {
    let rewards = REWARDS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, r)| r.clone())
            .collect::<Vec<_>>()
    });

    if rewards.is_empty() {
        Err("No rewards found.".to_string())
    } else {
        Ok(rewards)
    }
}

// Function to retrieve a supply chain event by id
#[ic_cdk::query]
fn get_supply_chain_event_by_id(id: u64) -> Result<SupplyChainEvent, Error> {
    SUPPLY_CHAIN_EVENTS_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(event) => Ok(event.clone()),
        None => Err(Error::NotFound {
            msg: "Supply chain event not found.".to_string(),
        }),
    })
}

// Function to retrieve a reward by id
#[ic_cdk::query]
fn get_reward_by_id(id: u64) -> Result<Reward, Error> {
    REWARDS_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(reward) => Ok(reward.clone()),
        None => Err(Error::NotFound {
            msg: "Reward not found.".to_string()),
        }),
    })
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
