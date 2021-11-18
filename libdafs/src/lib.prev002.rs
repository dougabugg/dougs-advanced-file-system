use std::collections::{HashMap, HashSet};

// placeholders u64 values for public-private keys
struct PublicKey(u64);
struct PrivateKey(u64);

// Asymmetric Key Pair
struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

/*
Stage 1: allow a user to configure all their devices, from any device. The idea will be to
have a single SharedFolder (which is private, but "synchronized" with all their devices),
with individual files within the folder representing the various devices the user wants
to be able to configure. The setup of new devices is a manual process, requiring the user
to create a DeviceProfile, then add it the SharedFolder, and setting the profile_index
to that SharedFolder. From there, the file in the SharedFolder corresponding to that
particular DeviceProfile, can specify other SharedFolders the device should synchronize.
The "FromSharedFolder" profile_index is intended to be use by devices that have a single
"owner/user". In the future, other types of profile_index's will be added, to enable
the later Stage features to be implemented. But for now, that will be the only
profile_index type, to make the design easier to wrap my head around.

the plan is for the application interface to be able to manage the "FromSharedFolder"
files (that configure devices) without manually editing files (but that will still be
a valid method of configuring devices)
*/

// an account managed by a single user or organization
struct Account {
    account_key: KeyPair,
    device_profiles: HashSet<(PublicKey, u32)>,
}

// a device can have multiple accounts associated with it
struct Device {
    device_key: KeyPair,
    account_profiles: HashSet<(PublicKey, u32)>,
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

enum SharedFolderWriteAccess {
    // a single account has write access
    Private,
    // multiple other accounts have write access
    Shared {
        account_ids: Vec<PublicKey>,
    }
}

enum SharedFolderReadAccess {
    // only accounts with write access can read data
    Private,
    // any device mirroring the folder can read data
    Public,
}

enum FolderItem {
    NamedFile {
        name: String,
        // TODO: should be an id
        data: DataBlock,
    },
    SubFolder {
        name: String,
        folder_items: Vec<FolderItem>,
    }
}

struct DataBlock {
    // TODO
}