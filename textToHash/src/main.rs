use openssl::hash::{hash, MessageDigest};

fn main() {
    let data = "test";
    let res = hash(MessageDigest::md5(), data.as_bytes())
        .unwrap()
        .to_vec();

    //let s = String::from_utf8_lossy(res);
    let mut s: String = format!("{:x?}", res);
    println!("{}", s);
}
