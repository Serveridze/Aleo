use snarkvm_dpc::{parameters::testnet2::Testnet2Parameters, PrivateKey, ViewKey};

fn main() {
    let mut rng = rand::thread_rng();
    let private_key = PrivateKey::<Testnet2Parameters>::new(&mut rng);
    let view_key = ViewKey::from_private_key(&private_key).expect("Unable to derive the view key from private key");
    print!("{}", view_key);
}
