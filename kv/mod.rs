use std::convert::TryInto;

pub fn create_kv<Value: 'static>() -> Vec<(String, Value)> {
    let vector: Vec<(String, Value)> = Vec::new();
    return vector;
}

pub fn add_kv<Value: 'static>(mut vector: Vec<(String, Value)>, key: &str, value: Value) -> Vec<(String, Value)> {
    vector.push((key.to_string(), value));
    return vector;
}

pub fn remove_kv<Value: 'static>(mut vector: Vec<(String, Value)>, index: i32) -> Vec<(String, Value)> {
    vector.remove(index.try_into().unwrap());
    return vector;
}