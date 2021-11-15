#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::collections::HashMap;

struct Community {
    // TODO: is this a cryptographic digest?
    id: u128,
    admins: Vec<u128>,
    collections: HashMap<(u128, u32), Collection>,

}

struct CommunityMember {
    community_id: u128,
    user_id: u128,
    // TODO: permissions here
}

// can be associated with multiple "devices" (TODO: how?)
struct User {
    // a cryptographic digest
    id: u128,
    // collections owned
    collections: HashMap<u32, Collection>,
}

// basically, a "folder"
struct Collection {
    // unique 32-bit value per owner/user.
    id: u32,
    owner: u128,
    // TODO: meta data, etc
}

/*
the design phase is always the most difficult for me. how do I structure the program?
how do I want to handle shared and public folders/data? do we want to track who last
modified a file in a shared folder?

the design is going to be complicated, no matter how I implement it. for example,
consider the following scenario: Alice, Bob and Carol want to create a "community"
together, and help mirror each other's data privately. The idea is that Alice can
trust that a block of data she gets from Bob or Carol, if it is for a "private"
collection or folder, is up to date, and not "malicious". In a way, we have to sort
of implement version tracking to make sure that data was actually deleted by Alice,
and not just "missing" from Bob or Carol's mirror.

Maybe this design is too ambitious; I mean, the use case I specifically had in mind
was to have a "makerspace server" that could securely mirror the private files of
members, and also share certain files with other members. there's no expectation that
members of the space need to mirror each others private files, but shouldn't they
be given the option? would that be a good idea to allow to happen?

consider this: when you create a folder, you can choose to "push" a mirror onto a
community server, and optionally have a flag that allows others to also mirror it.
that way, if someone just wants the main community device to mirror their data,
they set the flag to false, but if they set the flag to true, other members in the
community can "join the swarm" of devices sharing the data.

in a way, it's sort of like the bittorrent protocol, with a flag that determines if
a torrent can be seeded by other members of a community, while also having a way to
update the contents of a torrent. But it's not really like the bittorrent protocol,
because a "torrent" or "folder collection" is volatile, but in bittorrent, a torrent
is non-volatile (and can be identified by a hash).

I suppose each "folder collection" (FC) should have its own asymmetric-key-pair (AKP)
(which may be the same as a user's AKP if they are the sole owner). All the meta
data about the data in the FC will be signed by the AKP; this does sort of constrain
the "size" of an FC, but I suppose multiple FCs could be "embedded" within each other
to get around that.

but now how do we handle a "public folder"? see, this rabbit hole of properly
specifying the entire behavior of the program during the design phase is never
ending...

part of the issue with using a single AKP for a folder, is you can't remove a user's
access once they have it...

any design that I come up with, there will always be some specific scenario that
it won't be able to handle "elegantly". I guess I have to decide: do we want version
tracking, or not? I feel like version tracking would just "over complicate" the
scope of this project, which is to make a method of mirroring data between devices.
*/

/*
I need to take a break and come back and look at this later.
*/
