// to allow unused variables
#[allow(unused_variables)]

// a function, parameter name has a type (&str) and a it's size of output (u32) to avoid errors
fn read_version(transaction_hex: &str) -> u32 {
    //Convert Hex string to vector using hex crate. In terminal, do 'cargo add hex' - google hex crates
    let transaction_bytes = hex::decode(transaction_hex);
    let version_bytes = transaction_bytes[0..4];
    println!("version bytes:{:?}", version_bytes);
    // will return FromHexError but we'll resolve this using enums
}
fn main() {
    // obtained the hex fro here (https://blockstream.info/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2?expand)
    // let version: u32 = read_version(transaction_hex:"3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2")
    let version: u32 = read_version("3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2");
    println!("version:{}", version);
}
