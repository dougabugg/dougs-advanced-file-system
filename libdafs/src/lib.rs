use std::collections::{HashMap, HashSet};

// placeholders u64 values for public-private keys
struct PublicKey(u64);
struct PrivateKey(u64);

// Asymmetric Key Pair
struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

// an account managed by a single user or organization
struct Account {
    account_key: KeyPair,
    device_profiles: HashSet<(PublicKey, u32)>,
}

// a device can have multiple accounts associated with it
struct Device {
    device_key: KeyPair,
    account_profiles: HashSet<(PublicKey, u32)>,
    profiles: HashMap<u32, DeviceProfile>,
}

// a profile configuration on a device, owned by an account
struct DeviceProfile {
    // must be unique within a single device
    profile_id: u32,
    account_id: PublicKey,
    device_id: PublicKey,
    profile_index: ProfileIndex,
}

enum ProfileIndex {
    FromSharedFolder {
        account_id: PublicKey,
        folder_id: u32,
    },
}

struct SharedFolder {
    account_id: PublicKey,
    folder_id: u32,
    write_access: SharedFolderWriteAccess,
    read_access: SharedFolderReadAccess,
    folder_items: Vec<FolderItem>,
    data_blocks: Vec<DataBlock>,
}
/*
do we really want to have a ProfileIndex::FromSharedFolder?

a Device should just have a bunch of SharedFolders that it attempts to sychronize. maybe we
should "encode" device ownership into the protocol? It makes sense to do that, since
if we have a FromSharedFolder, what about race conditions? wouldn't that open a whole
other can of security worms?
*/