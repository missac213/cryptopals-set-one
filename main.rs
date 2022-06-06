extern crate base64;
extern crate hex;
fn main() {
    let inp_bytes: Vec<u8> = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    println!("{:?}", inp_bytes);

    let b64: String = base64::encode(inp_bytes);
    println!("{}", b64); 
}
