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
// ==============================
// Gestión de Candidatos (Simplificada)
// ==============================

pub fn add_candidate(
    env: Env,
    candidate_id: String,
    user_id: String,
    rfc: String
) {
    // Verificar que el usuario existe
    let users_key = symbol_short!("USERS");
    let users: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&users_key)
        .unwrap_or(Map::new(&env));

    if !users.contains_key(user_id.clone()) {
        panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
    }

    let storage_key = symbol_short!("CANDIDATO");
    let mut candidates: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&storage_key)
        .unwrap_or(Map::new(&env));

    if candidates.contains_key(candidate_id.clone()) {
        panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
    }

    let candidate_data = Vec::from_array(
        &env,
        [
            user_id,
            rfc
        ]
    );
    
    candidates.set(candidate_id.clone(), candidate_data);
    env.storage().persistent().set(&storage_key, &candidates);
    log!(&env, "Candidato registrado:", candidate_id);
}

pub fn authenticate_candidate(
    env: Env,
    candidate_id: String,
    rfc: String
) -> bool {
    let storage_key = symbol_short!("CANDIDATO");
    let candidates: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&storage_key)
        .unwrap_or(Map::new(&env));

    let candidate_data = match candidates.get(candidate_id) {
        Some(data) => data,
        None => return false,
    };

    // El RFC está en la posición 1 del vector de datos
    let stored_rfc = candidate_data.get(1).unwrap_or_else(|| String::from_str(&env, ""));
    
    // Comparación directa (case-sensitive)
    stored_rfc == rfc
}
// ==============================
// Gestión de Votos
// ==============================

pub fn votar_por_candidato(
    env: Env,
    id_voto: String,
    id_usuario: String,
    id_candidato: String,
    fecha_voto: String  // Cambiado a String
) {
    // 1. Verificar que el usuario existe
    let users_key = symbol_short!("USERS");
    let users: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&users_key)
        .unwrap_or(Map::new(&env));
    
    if !users.contains_key(id_usuario.clone()) {
        panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
    }

    // 2. Verificar que el candidato existe
    let candidates_key = symbol_short!("CANDIDATO");
    let candidates: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&candidates_key)
        .unwrap_or(Map::new(&env));
    
    if !candidates.contains_key(id_candidato.clone()) {
        panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
    }

    // 3. Verificar que el voto no existe ya
    let votes_key = symbol_short!("VOTOS");
    let mut votes: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&votes_key)
        .unwrap_or(Map::new(&env));
    
    if votes.contains_key(id_voto.clone()) {
        panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
    }

    // 4. Registrar el voto (sin conversión necesaria)
    let vote_data = Vec::from_array(
        &env,
        [
            id_usuario,
            id_candidato,
            fecha_voto  // Usamos el String directamente
        ]
    );
    
    votes.set(id_voto.clone(), vote_data);
    env.storage().persistent().set(&votes_key, &votes);
    log!(&env, "Voto registrado:", id_voto);
}


pub fn obtener_voto(env: Env, id_voto: String) -> Vec<String> {
    let votes_key = symbol_short!("VOTOS");
    let votes: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&votes_key)
        .unwrap_or(Map::new(&env));
    
    votes.get(id_voto).unwrap_or_else(|| Vec::new(&env))
}

pub fn listar_votos(env: Env) -> Map<String, Vec<String>> {
    let votes_key = symbol_short!("VOTOS");
    env
        .storage()
        .persistent()
        .get(&votes_key)
        .unwrap_or(Map::new(&env))
}
// ==============================
// Gestión de Administradores
// ==============================

pub fn agregar_administrador(
    env: Env,
    id_administrador: String,
    id_usuario: String,
    id_encuesta: String,
    rfc: String
) {
    // 1. Verificar que el usuario existe
    let users_key = symbol_short!("USERS");
    let users: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&users_key)
        .unwrap_or(Map::new(&env));
    
    if !users.contains_key(id_usuario.clone()) {
        panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
    }

    // 2. Verificar que el administrador no existe
    let admins_key = symbol_short!("ADMIN");
    let mut admins: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&admins_key)
        .unwrap_or(Map::new(&env));
    
    if admins.contains_key(id_administrador.clone()) {
        panic_with_error!(&env, (ScErrorType::Context, ScErrorCode::InvalidInput));
    }

    // 3. Registrar el administrador
    let admin_data = Vec::from_array(
        &env,
        [
            id_usuario,
            id_encuesta,
            rfc
        ]
    );
    
    admins.set(id_administrador.clone(), admin_data);
    env.storage().persistent().set(&admins_key, &admins);
    log!(&env, "Administrador registrado:", id_administrador);
}

pub fn autenticar_administrador(
    env: Env,
    id_administrador: String,
    rfc: String
) -> bool {
    let admins_key = symbol_short!("ADMIN");
    let admins: Map<String, Vec<String>> = env
        .storage()
        .persistent()
        .get(&admins_key)
        .unwrap_or(Map::new(&env));
    
    let admin_data = match admins.get(id_administrador) {
        Some(data) => data,
        None => return false,
    };

    // Comparación directa del RFC (case-sensitive)
    let stored_rfc = admin_data.get(2).unwrap_or_else(|| String::from_str(&env, ""));
    stored_rfc == rfc
}

}