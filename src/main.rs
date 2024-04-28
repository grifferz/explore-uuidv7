use num_format::{Locale, ToFormattedString};
use uuid::Uuid;

fn main() {
    println!("Full UUID:");
    let uuid = Uuid::now_v7();
    println!("{:>9} ({}): {uuid}", "uuid v7", uuid.to_string().len());
    let b64u = data_encoding::BASE64URL_NOPAD.encode(uuid.as_bytes());
    println!("{:>9} ({}): {b64u}", "Base64URL", b64u.len());
    let b58 = bs58::encode(uuid.as_bytes()).into_string();
    println!("{:>9} ({}): {b58}", "Base58", b58.len());

    println!("\nBase58 of bottom 64 bits:");
    let lower_bytes = &uuid.as_bytes()[8..];
    println!("{:>23}: {:x?}\n", "Hex bytes", lower_bytes);

    let bits_per_char = 58f64.log2();
    println!(
        "Base58 encodes log2(58) = {} bits per character\n",
        bits_per_char
    );

    println!("IDs from…   Max chars Base58          Can store");
    for i in 0..7 {
        let wanted_bits = 8 * (8 - i);
        let max_chars = (wanted_bits as f64 / bits_per_char).ceil();
        let b58 = bs58::encode(&lower_bytes[i..]).into_string();
        println!(
            "{} {:>2}        {:11} = {:>33}",
            "…bottom ".to_owned() + &wanted_bits.to_string() + "b",
            max_chars,
            b58,
            (2u128.pow(8 * (8 - i as u32))).to_formatted_string(&Locale::en) + " keys"
        );
    }
}
