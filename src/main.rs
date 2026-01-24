mod crypto;
use crypto::hash::Hash;
mod transaction;

fn main() {
    let h = Hash::hash(b"hello blockchain");
    println!("hashkey: {}", h);
}