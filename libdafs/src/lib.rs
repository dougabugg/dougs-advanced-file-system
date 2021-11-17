use std::collections::HashMap;

// placeholders u64 values for public-private keys
struct PublicKey(u64);
struct PrivateKey(u64);

// Asymmetric Key Pair
struct KeyPair {
    public_key: PublicKey,
    private_key: PrivateKey,
}

struct DataQuota {
    community_id: PublicKey,
    member_id: PublicKey,
    data_quota: u64,
}

enum DeviceMode {
    // the device is private, and only mirrors data from its owner.
    Private,
    // the device is public, and has extra space to mirror data from other owners.
    Public,
}

struct PeerDevice {
    owner_id: PublicKey,
    machine_id: u64,
    mode: DeviceMode,
}

/*
so, here we are again. I think this "project" needs to be better scoped, and the
features should be designed and implemented in stages, after "lower level features"
are "finished".

for example, the Minimum Viable Product (MVP) should be a simple app that lets you
sync data between devices, and also allow you to remotely "configure" devices, as
long as you have "ownership/access" to them.

next, I can work on a new "super product" that enables communities to allow "members"
to "push" data to be mirrored on the communities "swarm/network" of devices, with
settings for controlling space/bandwidth quotas, etc.

finally, there will be another "super feature" that enables the collective seeding
of public data, like the bittorrent network allows, while allowing the "author" to
push updates out to other peers who chose to seed the data.

each stage of features will build off the work done in the previous stage, tweaking
certain behaviors of the program to enable new features, while still being compatible
with other "simpler" features of the program. eventually, all the features may exist
in a single program/library, but the features will be isolated and implemented to
cooperate, rather than me trying to figure out how to implement every feature I want
right now, during this initial design phase.
*/