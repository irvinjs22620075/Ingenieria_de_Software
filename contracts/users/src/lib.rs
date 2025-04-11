#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec, Map, symbol_short};

#[contract]
pub struct UserManagementContract;

#[contractimpl]
impl UserManagementContract {
    pub fn add_user(
        env: Env,
        user_id: String,
        first_name: String,
        paternal_last_name: String,
        maternal_last_name: String,
        phone: String,
        email: String
    ) {
        let storage_key = symbol_short!("USERS");
        let mut users: Map<String, Vec<String>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if users.contains_key(user_id.clone()) {
            panic!("Usuario ya existe");
        }

        let user_data = Vec::from_array(
            &env,
            [
                first_name,
                paternal_last_name,
                maternal_last_name,
                phone,
                email
            ]
        );
        
        users.set(user_id, user_data);
        env.storage().persistent().set(&storage_key, &users);
    }

    pub fn get_user(env: Env, user_id: String) -> Vec<String> {
        let storage_key = symbol_short!("USERS");
        let users: Map<String, Vec<String>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        users.get(user_id).unwrap_or_else(|| Vec::new(&env))
    }

    pub fn update_user(
        env: Env,
        user_id: String,
        new_first_name: String,
        new_paternal_last_name: String,
        new_maternal_last_name: String,
        new_phone: String,
        new_email: String
    ) {
        let storage_key = symbol_short!("USERS");
        let mut users: Map<String, Vec<String>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if !users.contains_key(user_id.clone()) {
            panic!("Usuario no encontrado");
        }

        let updated_data = Vec::from_array(
            &env,
            [
                new_first_name,
                new_paternal_last_name,
                new_maternal_last_name,
                new_phone,
                new_email
            ]
        );
        
        users.set(user_id, updated_data);
        env.storage().persistent().set(&storage_key, &users);
    }

    pub fn delete_user(env: Env, user_id: String) {
        let storage_key = symbol_short!("USERS");
        let mut users: Map<String, Vec<String>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if !users.contains_key(user_id.clone()) {
            panic!("Usuario no encontrado");
        }

        users.remove(user_id);
        env.storage().persistent().set(&storage_key, &users);
    }

    pub fn list_users(env: Env) -> Map<String, Vec<String>> {
        let storage_key = symbol_short!("USERS");
        env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env))
    }
}
