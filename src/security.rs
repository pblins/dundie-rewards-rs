use passwords::hasher;
use std::fs;

pub fn generate_salt() -> [u8; 16] {
    let salt = hasher::gen_salt();
    save_salt(salt);
    salt
}

pub fn get_salt() -> [u8; 16] {
    load_salt()
}

fn save_salt(salt: [u8; 16]) {
    let string_salt = salt.map(|x| x.to_string()).join("");
    fs::write(".secrets.toml", format!("salt = \"{}\"", string_salt)).unwrap();
}

fn load_salt() -> [u8; 16] {
    let content = fs::read_to_string(".secrets.toml").unwrap();
    let string_salt = content.split(" = ").collect::<Vec<&str>>()[1];
    let u8_salt = string_salt
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .into_iter()
        .map(|x| x as u8)
        .collect::<Vec<u8>>();

    let mut arr = [0u8; 16];

    for (place, element) in arr.iter_mut().zip(u8_salt.iter()) {
        *place = *element;
    }

    arr
}

pub fn verify_password(plain_password: &String, hashed_password: &String) -> bool {
    return plain_password == hashed_password;
}
