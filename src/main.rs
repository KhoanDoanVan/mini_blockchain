mod crypto;
use crypto::hash::Hash;

fn main() {
    let h = Hash::hash(b"hello blockchain");
    println!("hashkey: {}", h);
}