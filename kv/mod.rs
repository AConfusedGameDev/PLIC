#![allow(dead_code)]

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

pub fn find_kv<Value: 'static>(vector: Vec<(String, Value)>, key: &str) -> Option<usize> {
    let mut iter = vector.into_iter();
    let position = iter.position(|x| x.0 == key.to_string());
    if position.is_some() {
        return Some(position.unwrap());
    }

    return None;
}

pub fn get_kv<Value: 'static + Clone>(vector: Vec<(String, Value)>, index: usize) -> (String, Value) {
    let key: String = (&vector[index].0).clone();
    let value: Value = (&vector[index].1).clone();
    return (key, value);
}

pub fn set_kv<Value: 'static>(mut vector: Vec<(String, Value)>, value: Value, index: usize) -> Vec<(String, Value)> {
    vector[index].1 = value;
    return vector;
}
