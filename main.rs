// rustc main.rs -o test_executeables/main.exe
// ./test_executeables/main.exe

mod kv;

fn main() {
    let mut integers = kv::create_kv::<i32>();
    
    integers = kv::add_kv::<i32>(integers.clone(), "key1", 1);

    integers = kv::remove_kv::<i32>(integers.clone(), 0);

    println!("{}", integers[0].0);
}