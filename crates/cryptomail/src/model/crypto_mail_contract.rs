pub use cryptomailcore_mod::*;
#[allow(clippy::too_many_arguments)]
mod cryptomailcore_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{parse_abi, Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::Middleware,
    };
    #[doc = "CryptoMailCore was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CRYPTOMAILCORE_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"depositor\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"DepositEvent\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"InvalidMessageId\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"depositor\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"RefundEvent\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"receipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"WithdrawEvent\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"getDeposit\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"depositor\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"recipient\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"block\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct CryptoMailCore.Deposit\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getTimeout\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getFee\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"fee\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"timeout\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"updateConfig\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"deposit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"refund\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  }\n]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct CryptoMailCore<M>(Contract<M>);
    impl<M> std::ops::Deref for CryptoMailCore<M> {
        type Target = Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: Middleware> std::fmt::Debug for CryptoMailCore<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CryptoMailCore))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: Middleware> CryptoMailCore<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>>(address: T, client: Arc<M>) -> Self {
            let contract = Contract::new(address.into(), CRYPTOMAILCORE_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `deposit` (0xd1f008a5) function"]
        pub fn deposit(&self, recipient: Address, message_id: [u8; 16]) -> ContractCall<M, bool> {
            self.0
                .method_hash([209, 240, 8, 165], (recipient, message_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDeposit` (0xaf3d4f84) function"]
        pub fn get_deposit(
            &self,
            message_id: [u8; 16],
        ) -> ContractCall<M, (U256, Address, Address, U256)> {
            self.0
                .method_hash([175, 61, 79, 132], message_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFee` (0xced72f87) function"]
        pub fn get_fee(&self) -> ContractCall<M, u8> {
            self.0
                .method_hash([206, 215, 47, 135], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTimeout` (0x3499ba95) function"]
        pub fn get_timeout(&self) -> ContractCall<M, u16> {
            self.0
                .method_hash([52, 153, 186, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(&self) -> ContractCall<M, Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refund` (0xdfdecfaf) function"]
        pub fn refund(&self, message_id: [u8; 16]) -> ContractCall<M, ()> {
            self.0
                .method_hash([223, 222, 207, 175], message_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateConfig` (0xd669d303) function"]
        pub fn update_config(&self, fee: u8, timeout: u16) -> ContractCall<M, ()> {
            self.0
                .method_hash([214, 105, 211, 3], (fee, timeout))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x36e3a811) function"]
        pub fn withdraw(&self, message_id: [u8; 16]) -> ContractCall<M, ()> {
            self.0
                .method_hash([54, 227, 168, 17], message_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `DepositEvent` event"]
        pub fn deposit_event_filter(&self) -> Event<M, DepositEventFilter> {
            self.0
                .event("DepositEvent")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `InvalidMessageId` event"]
        pub fn invalid_message_id_filter(&self) -> Event<M, InvalidMessageIdFilter> {
            self.0
                .event("InvalidMessageId")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(&self) -> Event<M, OwnershipTransferredFilter> {
            self.0
                .event("OwnershipTransferred")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `RefundEvent` event"]
        pub fn refund_event_filter(&self) -> Event<M, RefundEventFilter> {
            self.0
                .event("RefundEvent")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `WithdrawEvent` event"]
        pub fn withdraw_event_filter(&self) -> Event<M, WithdrawEventFilter> {
            self.0
                .event("WithdrawEvent")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DepositEventFilter {
        pub message_id: [u8; 16],
        pub depositor: Address,
        pub recipient: Address,
        pub amount: U256,
    }
    impl DepositEventFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                248, 49, 126, 133, 34, 97, 240, 79, 1, 229, 15, 23, 56, 138, 91, 87, 80, 101, 91,
                230, 31, 170, 17, 150, 171, 187, 138, 167, 124, 242, 215, 158,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`DepositEvent(bytes16,address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "DepositEvent(bytes16,address,address,uint256)"
        }
    }
    impl Detokenize for DepositEventFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 4 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    4,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let message_id =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let depositor =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let recipient =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(DepositEventFilter {
                message_id,
                depositor,
                recipient,
                amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct InvalidMessageIdFilter {
        pub message_id: [u8; 16],
    }
    impl InvalidMessageIdFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                21, 216, 133, 165, 137, 238, 55, 212, 67, 255, 64, 32, 126, 124, 99, 148, 110, 142,
                70, 215, 168, 181, 77, 185, 79, 146, 3, 220, 254, 174, 56, 60,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`InvalidMessageId(bytes16)`"]
        pub const fn abi_signature() -> &'static str {
            "InvalidMessageId(bytes16)"
        }
    }
    impl Detokenize for InvalidMessageIdFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 1 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    1,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let message_id =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(InvalidMessageIdFilter { message_id })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct OwnershipTransferredFilter {
        pub previous_owner: Address,
        pub new_owner: Address,
    }
    impl OwnershipTransferredFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                139, 224, 7, 156, 83, 22, 89, 20, 19, 68, 205, 31, 208, 164, 242, 132, 25, 73, 127,
                151, 34, 163, 218, 175, 227, 180, 24, 111, 107, 100, 87, 224,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`OwnershipTransferred(address,address)`"]
        pub const fn abi_signature() -> &'static str {
            "OwnershipTransferred(address,address)"
        }
    }
    impl Detokenize for OwnershipTransferredFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 2 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    2,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let previous_owner =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_owner =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(OwnershipTransferredFilter {
                previous_owner,
                new_owner,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct RefundEventFilter {
        pub message_id: [u8; 16],
        pub depositor: Address,
        pub amount: U256,
        pub fee: U256,
    }
    impl RefundEventFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                3, 182, 222, 217, 209, 251, 93, 171, 206, 23, 16, 15, 49, 246, 104, 11, 57, 136,
                228, 79, 232, 227, 134, 224, 157, 55, 133, 169, 31, 145, 233, 4,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`RefundEvent(bytes16,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "RefundEvent(bytes16,address,uint256,uint256)"
        }
    }
    impl Detokenize for RefundEventFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 4 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    4,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let message_id =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let depositor =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fee = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(RefundEventFilter {
                message_id,
                depositor,
                amount,
                fee,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct WithdrawEventFilter {
        pub message_id: [u8; 16],
        pub receipient: Address,
        pub amount: U256,
        pub fee: U256,
    }
    impl WithdrawEventFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                211, 186, 46, 145, 64, 59, 60, 181, 100, 5, 42, 0, 208, 159, 145, 83, 181, 186,
                112, 142, 35, 188, 216, 122, 133, 253, 85, 156, 3, 248, 146, 158,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`WithdrawEvent(bytes16,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "WithdrawEvent(bytes16,address,uint256,uint256)"
        }
    }
    impl Detokenize for WithdrawEventFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 4 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    4,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let message_id =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let receipient =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let fee = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(WithdrawEventFilter {
                message_id,
                receipient,
                amount,
                fee,
            })
        }
    }
}
