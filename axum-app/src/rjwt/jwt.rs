#[allow(unused)]
use serde::{Serialize, Deserialize};
use hmac::Mac;
use sha2::Sha256;
use data_encoding::BASE64URL;
use rand::Rng;
use uuid::Uuid;
use std::time::{SystemTime, Duration};
use chrono::{DateTime, Local, FixedOffset};

// Header.Payload.Signature
#[derive(Serialize, Deserialize, Debug)]
struct Header {
    algorithm: &'static str,
    ty: &'static str,
}
const JWT_HEADER: Header = Header {
    algorithm: "HS256",
    ty: "JWT",
}; 
#[derive(Serialize, Deserialize, Debug)]
struct Playload {
    issuer: String,
    expirationtime: u128,
    notbefore: u128,
    iat: String,
    jti: Uuid,
}

pub fn get_signture(username: String)  -> String {
    //first to generate secret
    let secret: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    
    //localtime (milliseconds)
    let now = SystemTime::now();
    let milliseconds = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get timestamp")
        .as_millis();
    //expiration
    let expiration = now
        .checked_add(Duration::from_secs(3600))
        .and_then(|expire_time| expire_time.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|expire_duration| expire_duration.as_millis())
        .expect("Failed to compute expiration time");

    let local: DateTime<Local> = Local::now();
    // 转换为GMT时间
    // let gmt: DateTime<FixedOffset> = local.with_timezone(&FixedOffset::west(0));
    let gmt: DateTime<FixedOffset> = local.with_timezone(&FixedOffset::west_opt(0).expect("Invalid offset"));

    // 格式化时间
    let date_header_value = gmt.format("%a, %d %b %Y %H:%M:%S GMT").to_string();


    let id = Uuid::new_v4();
    let playload = Playload{
        issuer: username,
        expirationtime: expiration,
        notbefore: milliseconds,
        iat: date_header_value,
        jti: id,
    };
    //serialize
    let header = serde_json::to_string(&JWT_HEADER).unwrap();
    let playload = serde_json::to_string(&playload).unwrap();
    //Base64url 
    let se_string = BASE64URL.encode(header.as_bytes()) + "." + &BASE64URL.encode(playload.as_bytes());
    //signture
    let signture_result = hmac_sha256(&se_string, secret.as_bytes());
    //finally
    let jwt_result = se_string +  "." + &signture_result;

    jwt_result


}


fn hmac_sha256(input: &str, key: &[u8]) -> String {
    let mut mac = hmac::Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(input.as_bytes());
    let bytes = mac.finalize().into_bytes();
    // hex::encode(bytes)
    BASE64URL.encode(&bytes)
}