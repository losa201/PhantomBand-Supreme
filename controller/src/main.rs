// controller/src/main.rs

use common::crypto;

fn main() {
    println!("PhantomBand Controller starting...");
    let keypair = crypto::generate_keypair();
    println!("Generated keypair: {}", keypair);
    // TODO: Implement controller logic
}
