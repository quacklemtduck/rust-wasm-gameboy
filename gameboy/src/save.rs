use base64::{engine::general_purpose, Engine as _};

pub fn set_item(name: &str, data: &Vec<u8>) -> Result<(), wasm_bindgen::JsValue> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    let s = general_purpose::STANDARD.encode(data);
    local_storage.set_item(name, &s)
}

pub fn get_item(name: &str) -> Option<Vec<u8>> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    let value = match local_storage.get_item(name) {
        Ok(val) => val,
        Err(_) => None,
    };

    let data = match value {
        Some(s) => {
            let val = general_purpose::STANDARD.decode(s);
            match val {
                Ok(x) => Some(x),
                Err(_) => None,
            }
        },
        None => None,
    };
    
    return data
}