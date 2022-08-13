// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::chain_key::ChainKey;
use crate::chains::Chains;
use crate::message_key::MessageKey;
use crate::session_key::SessionKey;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use rand_core::{CryptoRng, OsRng, RngCore};

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Formatter};
use x25519_dalek::{PublicKey, StaticSecret};

/// Implementation of the DR protocol between 2 parties.
/// Initialized externally. Has no persistence.
/// The methods are not concurrency safe. They are designed to use by a client
// such as an actor who ensures serialized access to the methods.

/// In a Double Ratchet session between Alice and Bob each party stores a KDF key for three chains: a root chain, a sending chain, and a receiving chain
/// (Alice’s sending chain matches Bob’s receiving chain, and vice versa).
/// As Alice and Bob exchange messages they also exchange new Diffie-Hellman
/// public keys, and the Diffie-Hellman output secrets become the inputs to the root chain. The output keys from the root chain become new KDF keys for the sending and receiving chains. This is called the Diffie-Hellman ratchet.
/// The sending and receiving chains advance as each message is sent and received. Their output keys are used to encrypt and decrypt messages. This is called the symmetric-key ratchet

/*
5.1. Integration with X2DH
The Double Ratchet algorithm can be used in combination with the X2DH key agreement protocol [1].
The Double Ratchet plays the role of a “post-X2DH” protocol which takes the session key SK negotiated by X3DH and uses
it as the Double Ratchet’s initial root key.
The following outputs from X2DH are used by the Double Ratchet:
• The SK output from X2DH becomes the SK input to Double Ratchet initialization (see Section 3.3).
• The AD output from X2DH becomes the AD input to Double Ratchet encryption and decryption (see Section 3.4 and Section 3.5).
• Bob’s signed prekey from X2DH (SPKB) becomes Bob’s initial ratchet public key (and corresponding key pair) for Double Ratchet initialization.
Any Double Ratchet message encrypted using Alice’s initial sending chain can serve as an “initial ciphertext” for X2DH.
To deal with the possibility of lost or out-of-order messages, a recommended pattern is for Alice to repeatedly send the
same X2DH initial message prepended to all of her Double Ratchet messages until she receives Bob’s first Double Ratchet response message.
*/

/// The state of one end of a double ratchet session.
///
/// Maintains the several KDF chains used in the double ratchet algorithm,
/// advancing each as necessary with the appropriate keys.
///
/// There is an asymmetry to the protocol startup.  The participant who
/// initiates the session needs to have the public key for the other
/// participant. The session can then be created with:
///
/// ```ignore
/// # extern crate curve25519_dalek;
/// # extern crate double_ratchet;
/// # extern crate rand;
/// # use curve25519_dalek::edwards::CompressedEdwardsY;
/// # use double_ratchet::keys::{ChainKey, RatchetKeyPublic};
/// # use double_ratchet::ratchet::DoubleRatchet;
/// # use rand::OsRng;
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let bob_public_key = RatchetKeyPublic::from(&CompressedEdwardsY::from_slice(&[0; 32][..]).decompress().unwrap().to_montgomery());
/// # let mut csprng = OsRng::new().unwrap();
/// #
/// let mut alice = DoubleRatchet::<OsRng, _>::with_peer(&[1, 2, 3][..], root_key, &mut csprng, &bob_public_key);
/// ```
///
/// The participant receiving the first message must have the secret key
/// corresponding to the public key that the other participant has.  The
/// second session can be created with:
///
/// ```ignore
/// # extern crate double_ratchet;
/// # extern crate rand;
/// # use double_ratchet::keys::{ChainKey, RatchetKeyPair};
/// # use double_ratchet::ratchet::DoubleRatchet;
/// # use rand::OsRng;
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let bob_keypair = RatchetKeyPair::generate(&mut OsRng::new().unwrap());
/// # let mut csprng = OsRng::new().unwrap();
/// #
/// let mut bob = DoubleRatchet::<OsRng, _>::with_keypair(&[1, 2, 3][..], root_key, &mut csprng, bob_keypair);
/// ```
///
/// After that the participants operate the same. Upon receipt of a
/// message with a new public key, perform a Diffie-Hellman ratchet.
/// Then get the message key to decrypt with:
///
/// ```ignore
/// # extern crate curve25519_dalek;
/// # extern crate double_ratchet;
/// # extern crate rand;
/// # use curve25519_dalek::edwards::CompressedEdwardsY;
/// # use double_ratchet::keys::{ChainKey, RatchetKeyPair, RatchetKeyPublic};
/// # use double_ratchet::ratchet::DoubleRatchet;
/// # use rand::OsRng;
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let mut csprng = OsRng::new().unwrap();
/// # let bob_keypair = RatchetKeyPair::generate(&mut csprng);
/// # let mut csprng = OsRng::new().unwrap();
/// # let mut bob = DoubleRatchet::<OsRng, _>::with_keypair(&[1, 2, 3][..], root_key, &mut csprng, bob_keypair);
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let mut csprng = OsRng::new().unwrap();
/// # let mut alice = DoubleRatchet::<OsRng, _>::with_peer(&[1, 2, 3][..], root_key, &mut csprng, bob.public());
/// #
/// bob.ratchet(alice.public());
/// let message_key = bob.next_receiving_key();
///
/// alice.ratchet(bob.public());
/// let message_key = alice.next_receiving_key();
/// ```
///
/// Upon receipt of a message with the same public key as a prior one,
/// perform a symmetric ratchet by just asking for the next key:
///
/// ```ignore
/// # extern crate curve25519_dalek;
/// # extern crate double_ratchet;
/// # extern crate rand;
/// # use curve25519_dalek::edwards::CompressedEdwardsY;
/// # use double_ratchet::keys::{ChainKey, RatchetKeyPair, RatchetKeyPublic};
/// # use double_ratchet::ratchet::DoubleRatchet;
/// # use rand::OsRng;
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let mut csprng = OsRng::new().unwrap();
/// # let bob_keypair = RatchetKeyPair::generate(&mut csprng);
/// # let mut csprng = OsRng::new().unwrap();
/// # let mut bob = DoubleRatchet::<OsRng, _>::with_keypair(&[1, 2, 3][..], root_key, &mut csprng, bob_keypair);
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let mut csprng = OsRng::new().unwrap();
/// # let mut alice = DoubleRatchet::<OsRng, _>::with_peer(&[1, 2, 3][..], root_key, &mut csprng, bob.public());
/// # bob.ratchet(alice.public());
/// #
/// let message_key = alice.next_receiving_key();
/// let message_key = alice.next_receiving_key();
/// let message_key = alice.next_receiving_key();
/// ```
///
/// To send a message, perform a symmetric ratchet on the sending chain
/// by requesting the next sending key:
///
/// ```ignore
/// # extern crate curve25519_dalek;
/// # extern crate double_ratchet;
/// # extern crate rand;
/// # use curve25519_dalek::edwards::CompressedEdwardsY;
/// # use double_ratchet::keys::{ChainKey, RatchetKeyPair, RatchetKeyPublic};
/// # use double_ratchet::ratchet::DoubleRatchet;
/// # use rand::OsRng;
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let bob_keypair = RatchetKeyPair::generate(&mut OsRng::new().unwrap());
/// # let mut csprng = OsRng::new().unwrap();
/// # let mut bob = DoubleRatchet::<OsRng, _>::with_keypair(&[1, 2, 3][..], root_key, &mut csprng, bob_keypair);
/// # let root_key = ChainKey::from(&[0; 32][..]);
/// # let mut csprng = OsRng::new().unwrap();
/// # let mut alice = DoubleRatchet::<OsRng, _>::with_peer(&[1, 2, 3][..], root_key, &mut csprng, bob.public());
/// # bob.ratchet(alice.public());
/// #
/// let message_key = bob.next_sending_key();
/// ```

/// DoubleRatchet is a stateful double ratchet session between 2 parties.
#[derive(Clone, Serialize, Deserialize)]
pub struct DoubleRatchet {
    chains: Chains,
    key: Option<StaticSecret>, // local side ratchet key. Public can be extracted from private
    pub ad: Option<Bytes>, // AD - see DR algo and X2DH - we need to store the ad generated between alice and bob in the key exchange phase that generated this session
    pub session_id: u64, // unique session id created by the session initiator and stored by the 2 parties.
}

impl Debug for DoubleRatchet {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl DoubleRatchet {
    /// Create a new DoubleRatchet with a remote peer who has shares with us (info, root_key, peer_pub_dr_key).
    /// Bob receives from Alice published peer_public_key and computes root_key session key based on info from A.
    /// Bob sends to alice his public ratchet key so alice can init with him using new_with_keys
    /// This method is used when an entity wants to initiates a new DR session with another peer
    pub fn new_with_peer<R: CryptoRng + RngCore>(
        input: SessionKey,  // static salt shared between all peers on the network
        root_key: ChainKey, // the dr root key is a shared secret between this and remote peer computed via another protocol such X2DH dh key exchange.
        csprng: &mut R,
        peer_pub_key: &PublicKey, // Public ratchet key used by the other peer and received from it
        ad: Bytes,                // AD used to encrypt/decrypt
    ) -> Result<DoubleRatchet> {
        let mut session = DoubleRatchet {
            chains: Chains::init(input, root_key),
            key: None,
            ad: Some(ad),
            session_id: OsRng.next_u64(),
        };

        // Initialize the dr session by doing a half-ratchet
        session.half_ratchet(csprng, peer_pub_key)?;
        Ok(session)
    }

    /// Create a new DoubleRatchet using a root key and a key pair
    /// Bob uses this function when it wants to create a DR session with Alice.
    /// It create a new ratchet keypair to be used and computes root_key based on B pre-key and identity.
    /// The public key should be sent to B who use new_with_peer() to create the same ratchet locally.
    /// This is used by the receiving side to a new DR session reuqest sent by another peer that used new_with_peer()
    pub fn new_with_keys(
        input: SessionKey,  // 32 bytes shared by all peers on a network
        root_key: ChainKey, // this is a shared secret between the 2 parties, created with another protocol such as X2DH.
        key: StaticSecret,  // this is generated by Alice (ratchet root key pair)
        ad: Bytes,          // AD used to encrypt/decrypt
        session_id: u64,    // Unique session id provided by the other party
    ) -> DoubleRatchet {
        DoubleRatchet {
            chains: Chains::init(input, root_key),
            key: Some(key),
            ad: Some(ad),
            session_id,
        }
    }

    pub fn get_ad(&self) -> Result<&[u8]> {
        let ad = self.ad.as_ref().ok_or_else(|| anyhow!("missing ad"))?;
        Ok(ad.as_ref())
    }

    /// Returns the current DR ratchet public key
    pub fn get_public_key(&self) -> Option<PublicKey> {
        match self.key.as_ref() {
            Some(s) => {
                let k: PublicKey = s.into();
                Some(k)
            }
            None => None,
        }
    }

    /// Returns the current DR ratchet private key
    pub fn get_private_key(&self) -> Option<StaticSecret> {
        match self.key.as_ref() {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    /// Perform a full ratchet step.
    /// This is called when a new peer's public ratchet key is received.
    /// Both sending and receiving chains are updated.
    pub fn ratchet<R: CryptoRng + RngCore>(
        &mut self,
        csprng: &mut R,
        peer_pub_ratchet_key: &PublicKey,
        pn: u32, // other user's previous send chain index
    ) -> Result<()> {
        if self.key.is_none() {
            return Err(anyhow!("expected to have a ratchet key"));
        }

        let sk = self.diffie_hellman(peer_pub_ratchet_key);
        self.chains.next_receiving_chain(sk, pn)?;

        self.generate_keypair(csprng);

        let sk_1 = self.diffie_hellman(peer_pub_ratchet_key);
        self.chains.next_sending_chain(sk_1)
    }

    /// Advance the sending chain and return its symmetric MessageKey output.
    pub fn next_sending_key(&mut self) -> Result<(u32, MessageKey)> {
        self.chains.next_sending_key()
    }

    /// Get receiving key at a specific index - store all skipped keys if any in this session
    /// Receiving chain wil advance if needed. Supports getting old receiving key in the current receiving chain.
    pub fn get_receiving_key(&mut self, index: u32) -> Result<MessageKey> {
        self.chains.get_receiving_key(index)
    }

    /// Performs diffie hellman using a peer's ratchet public key and our ratchet private key
    /// and return the shared secret output.
    fn diffie_hellman(&self, peer_pub_key: &PublicKey) -> SessionKey {
        let shared_secret = self.key.as_ref().unwrap().diffie_hellman(peer_pub_key);
        let bytes = shared_secret.as_bytes();
        SessionKey::from(&bytes[..])
    }

    /// Generate a new DR keypair and set it as the public and private keys for the ratchet
    fn generate_keypair<R: CryptoRng + RngCore>(&mut self, csprng: &mut R) {
        self.key = Some(StaticSecret::new(csprng));
    }

    /// Perform a half-ratchet step - advances the root chain with a new key
    /// and advances only the sending chain with the new root chain public key
    pub(crate) fn half_ratchet<R: CryptoRng + RngCore>(
        &mut self,
        csprng: &mut R,
        peer_pub_key: &PublicKey,
    ) -> Result<()> {
        if self.key.is_some() {
            return Err(anyhow!(
                "can only perform half-ratchet on init. Expected no stored private dr key."
            ));
        }
        self.generate_keypair(csprng);
        let sk = self.diffie_hellman(peer_pub_key);
        self.chains.next_sending_chain(sk)
    }
}
