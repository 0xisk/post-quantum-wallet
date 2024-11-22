use k256::ecdsa::Signature;

pub struct ECDSAInputs {
    pub owner_address: String,
    pub message: Vec<u8>,
    pub signature: Signature,
    pub rec_id_bytes: u8,
}

// impl ECDSAInputs {
//     pub fn new(
//         owner_address: String,
//         message: Vec<u8>,
//         signature: &Signature,
//         rec_id_bytes: u8,
//     ) -> Self {
//         ECDSAInputs {
//             owner_address,
//             message,
//             signature,
//             rec_id_bytes,
//         }
//     }
// }
