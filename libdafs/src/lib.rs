use std::collections::HashMap;

struct PublicKey(u64);
struct PrivateKey(u64);

// Asymmetric Key Pair
struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

struct FolderCollection {
    id: PublicKey,
}

// to be determined...