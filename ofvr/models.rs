use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Keypair {
    id: Vec<u8>,
    private: Vec<u8>,
    public: Vec<u8>,
    lock: Vec<u8>,
    salt: t16::Data,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Axis {
    id: Signature,
    value: u64,
    appearance: t16::Data,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Plain {
    id: Signature,
    x_axis: Axis,
    y_axis: Axis,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Azimuth {
    id: Signature,
    x_axis: Axis,
    y_axis: Axis,
    z_axis: Axis,
    path: Vec<Plain>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Signature {
    id: Vec<u8>,
    salt: t16::Data,
    private: Vec<u8>,
    public: Vec<u8>,
    unlock: Keypair,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Identity {
    id: Vec<u8>,
    created: PostQuantumPosition,
    updated: PostQuantumPosition,
    email: String,
    keys: Vec<Keypair>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct PostQuantumPosition {
    id: Vec<u8>,
    published: t16::Data,
    ot_secret: Vec<u8>,
    public_key_pair: Keypair,
    private_key_pair: Keypair,
    signature: Signature,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct KeyStuff {
    id: Vec<u8>,
    current_quantum_position: PostQuantumPosition,
    quantum_positions: Vec<PostQuantumPosition>,
    public_key_pair: Keypair,
    private_key_pair: Keypair,
    signature: Signature,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Peer {
    id: Vec<u8>,
    peer_key_stuff: KeyStuff,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Authority {
    id: Vec<u8>,
    key_stuff: KeyStuff,
    public_signature: Signature,
    private_signature: Signature,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Complex {
    real: i64,
    imag: u64,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Instance {
    id: Azimuth,
    whence: Whence,
    color: Complex,
    cost: Complex,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Whence {
    id: Azimuth,
    lat: Complex,
    lng: Complex,
    when: Axis,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct TransportMedium {
    id: Instance,
    key_pair: Keypair,
    signature: Signature,
    capacity: Vec<Instance>,
    tension: Vec<Instance>,
    reflection: Vec<Instance>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct SentOcurrence {
    id: PostQuantumPosition,
    whence: Whence,
    at: t16::Data,
    source_peer_id: Vec<u8>,
    destination_peer_id: Vec<u8>,
    key_stuff: KeyStuff,
    signature: Signature,
    azimuth: Azimuth,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ReceiptOcurrence {
    id: PostQuantumPosition,
    whence: Whence,
    at: t16::Data,
    source_peer_id: Vec<u8>,
    source_peer_signature: Signature,
    destination_peer_id: Vec<u8>,
    destination_peer_signature: Signature,
    key_stuff: KeyStuff,
    azimuth: Azimuth,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum Ocurrence {
    Receipt(ReceiptOcurrence),
    Sent(SentOcurrence),
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Envelope {
    pqp: PostQuantumPosition,
    key_pair: Keypair,
    id: Vec<u8>,
    signature: Signature,
    message: Message,
    stamps: BTreeMap<Signature, Ocurrence>,
}
