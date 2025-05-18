#![no_std]
use soroban_sdk::{
    contract, contractimpl, 
    Env, String, Vec, Map,
    symbol_short, log, panic_with_error,
    xdr::{ScErrorType, ScErrorCode}
    // Eliminamos Address y BytesN que no se usan
};

#[contract]
pub struct TalentManagementContract;

#[contractimpl]
impl TalentManagementContract {
    // ============================
    // Sección de Gestión de Usuarios
    // ============================

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
            panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
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
        
        users.set(user_id.clone(), user_data);
        env.storage().persistent().set(&storage_key, &users);
        log!(&env, "Usuario registrado:", user_id);
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
            panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
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
        
        users.set(user_id.clone(), updated_data);
        env.storage().persistent().set(&storage_key, &users);
        log!(&env, "Usuario actualizado:", user_id);
    }

    pub fn delete_user(env: Env, user_id: String) {
        let storage_key = symbol_short!("USERS");
        let mut users: Map<String, Vec<String>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if !users.contains_key(user_id.clone()) {
            panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
        }

        users.remove(user_id.clone());
        env.storage().persistent().set(&storage_key, &users);
        log!(&env, "Usuario eliminado:", user_id);
    }

    pub fn list_users(env: Env) -> Map<String, Vec<String>> {
        let storage_key = symbol_short!("USERS");
        env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env))
    }

    // ==============================
    // Gestión de Encuestas
    // ==============================

    pub fn ingresar_datos_encuesta(
        env: Env,
        id_encuesta: String,
        nombre_encuesta: String,
        descripcion: String,
        fecha_creacion: String,
        fecha_culminacion: String,
        id_voto: String,
        id_candidato: String
    ) {
        let storage_key = symbol_short!("ENCUESTAS");
        let mut encuestas: Map<String, Vec<String>> = env.storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if encuestas.contains_key(id_encuesta.clone()) {
            panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
        }

        let encuesta_data = Vec::from_array(
            &env,
            [
                nombre_encuesta,
                descripcion,
                fecha_creacion,
                fecha_culminacion,
                id_voto,
                id_candidato
            ]
        );
        
        encuestas.set(id_encuesta, encuesta_data);
        env.storage().persistent().set(&storage_key, &encuestas);
    }

    pub fn registrar_candidato(env: Env, id_encuesta: String, id_candidato: String) {
        let encuestas_key = symbol_short!("ENCUESTAS");
        let mut encuestas: Map<String, Vec<String>> = env.storage()
            .persistent()
            .get(&encuestas_key)
            .unwrap_or(Map::new(&env));

        let mut encuesta_data = encuestas.get(id_encuesta.clone())
            .unwrap_or_else(|| panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput)));

        encuesta_data.set(5, id_candidato);
        encuestas.set(id_encuesta, encuesta_data);
        env.storage().persistent().set(&encuestas_key, &encuestas);
    }

    pub fn actualizar_encuesta(
        env: Env,
        id_encuesta: String,
        nombre_encuesta: String,
        descripcion: String,
        fecha_culminacion: String
    ) {
        let storage_key = symbol_short!("ENCUESTAS");
        let mut encuestas: Map<String, Vec<String>> = env.storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        let mut encuesta_data = encuestas.get(id_encuesta.clone())
            .unwrap_or_else(|| panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput)));

        encuesta_data.set(0, nombre_encuesta);
        encuesta_data.set(1, descripcion);
        encuesta_data.set(3, fecha_culminacion);

        encuestas.set(id_encuesta, encuesta_data);
        env.storage().persistent().set(&storage_key, &encuestas);
    }
}