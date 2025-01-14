// to allow unused variables
#[allow(unused_variables)]

// a function, parameter name has a type (&str) and a it's size of output (u32) to avoid errors
fn read_version(transaction_hex: &str) -> u32 {
    return 1;
}
fn main() {
    // obtained the hex fro here (https://blockstream.info/testnet/tx/3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2?expand)
    let version: u32 = read_version(transaction_hex:"3c1804567a336c3944e30b3c2593970bfcbf5b15a40f4fc6b626a360ee0507f2")
    println!("version:{}", version);
}
