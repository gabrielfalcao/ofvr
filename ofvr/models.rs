use serde::{Deserialize, Serialize};

pub struct Keypair {
    id: Vec<u8>,
    private: Vec<u8>,
    public: Vec<u8>,
    lock: Vec<u8>,
    salt: t16::Data,
}

pub struct Axis {
    id: Signature,
    value: u64,
    appearance: t16::Data,
}

pub struct Plain {
    id: Signature,
    x_axis: Axis,
    y_axis: Axis,
}

pub struct Azimuth {
    id: Signature,
    x_axis: Axis,
    y_axis: Axis,
    z_axis: Axis,
    path: Vec<Plain>,
}

pub struct Signature {
    id: Vec<u8>,
    salt: t16::Data,
    private: Vec<u8>,
    public: Vec<u8>,
    unlock: Keypair,
}

pub struct Identity {
    id: Vec<u8>,
    created: PostQuantumPosition,
    updated: PostQuantumPosition,
    email: String,
    keys: Vec<Keypair>,
}

pub struct PostQuantumPosition {
    id: Vec<u8>,
    published: t16::Data,
    ot_secret: Vec<u8>,
    public_key_pair: Keypair,
    private_key_pair: Keypair,
    signature: Signature,
}

pub struct KeyStuff {
    id: Vec<u8>,
    current_quantum_position: PostQuantumPosition,
    quantum_positions: Vec<PostQuantumPosition>,
    public_key_pair: Keypair,
    private_key_pair: Keypair,
    signature: Signature,
}

pub struct Peer {
    id: Vec<u8>,
    peer_key_stuff: KeyStuff,
}

pub struct Authority {
    id: Vec<u8>,
    key_stuff: KeyStuff,
    public_signature: Signature,
    private_signature: Signature,
}

pub struct SentOcurrence {
    id: PostQuantumPosition,
    at: t16::Data,
    source_peer_id: Vec<u8>,
    destination_peer_id: Vec<u8>,
    key_stuff: KeyStuff,
    signature: Signature,
    azimuth: Azimuth,
}
pub struct ReceiptOcurrence {
    id: PostQuantumPosition,
    at: t16::Data,
    source_peer_id: Vec<u8>,
    source_peer_signature: Signature,
    destination_peer_id: Vec<u8>,
    destination_peer_signature: Signature,
    key_stuff: KeyStuff,
    azimuth: Azimuth,
}

pub enum Ocurrence {
    Receipt(ReceiptOcurrence),
    Sent(SentOcurrence),
}

pub struct Message {
    id: Vec<u8>,
    published: PostQuantumPosition,
    message_key_pair: Keypair,
    peer_key_stuff: KeyStuff,
    sent: SentOcurrence,
    receipt: ReceiptOcurrence,
    source: Peer,
    destination: Peer,
    azimuth: Azimuth,
}

pub struct Envelope {
    pqp: PostQuantumPosition,
    key_pair: Keypair,
    id: Vec<u8>,
    signature: Signature,
    message: Message,
    stamps: HashTable<Signature, Ocurrence>,
}
