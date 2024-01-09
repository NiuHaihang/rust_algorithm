use hmac::{Hmac,Mac};
use sha2::Sha256;
use base64;

fn main(){
    let t :i64 = 1704807050;
    let str_to_sign = format!("{}\n{}",t.to_string(),"fGn4MgckPni3okA7zEtf8c");

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(b"fGn4MgckPni3okA7zEtf8c").expect("Hmac can take key of any size!");

    mac.update(str_to_sign.as_bytes());
    let res = mac.finalize();

    let sig = base64::encode(res.into_bytes());
    println!("{sig}");
}