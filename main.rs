// rustc main.rs -o test_executeables/main.exe
// ./test_executeables/main.exe

mod kv;

fn main() {
    let mut integers = kv::create_kv::<i32>();
    
    integers = kv::add_kv::<i32>(integers.clone(), ("key1", 1));

    let index: usize = kv::index_of_kv::<i32>(integers.clone(), "key1").unwrap();

    let key: i32 = kv::get_v::<i32>(integers.clone(), index);

    println!("{}", key);
}