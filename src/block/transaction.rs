#[derive(Clone, Debug)]
pub struct Transaction {
    pub sender: [u8; 33],      // compressed public key, always 33 bytes
    pub receiver: [u8; 33],
    pub amount: u64,
    pub signature: [u8; 64],   // ECDSA signature, always 64 bytes
}
