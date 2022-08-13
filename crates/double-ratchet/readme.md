## Integration with X2DH
The Double Ratchet algorithm can be used in combination with the X2DH key agreement protocol [1]. The Double Ratchet plays the role of a “post-X3DH” protocol which takes the session key SK negotiated by X3DH and uses it as the Double Ratchet’s initial root key.

The following outputs from X2DH are used by the Double Ratchet:

- The SK output from X2DH becomes the SK input to Double Ratchet initialization (see Section 3.3).
- The AD output from X2DH becomes the AD input to Double Ratchet encryption and decryption (see Section 3.4 and Section 3.5).
- Bob’s signed prekey from X2DH (SPKB) becomes Bob’s initial ratchet public key (and corresponding key pair) for Double Ratchet initialization.
Any Double Ratchet message encrypted using Alice’s initial sending chain can serve as an “initial ciphertext” for X2DH. To deal with the possibility of lost or out-of-order messages, a recommended pattern is for Alice to repeatedly send the same X2DH initial message prepended to all of her Double Ratchet messages until she receives Bob’s first Double Ratchet response message.