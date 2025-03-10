use anchor_lang::prelude::{borsh, Pubkey};
#[cfg(feature = "idl-build")]
use anchor_lang::IdlBuild;
#[cfg(feature = "idl-build")]
use std::fmt;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::mem;

pub const HANDLE_PROPOSAL_DISCRIMINATOR: [u8; 8] = [0xe3, 0x94, 0x48, 0xc2, 0x4b, 0xca, 0x9d, 0x29];

const DISCRIMINATOR_SIZE: usize = 8;
pub const CAMBRIAN_ID: Pubkey = Pubkey::new_from_array([
    212u8, 7u8, 216u8, 101u8, 36u8, 59u8, 253u8, 23u8, 121u8, 238u8, 180u8, 56u8, 185u8, 234u8,
    101u8, 19u8, 235u8, 62u8, 91u8, 33u8, 49u8, 149u8, 1u8, 100u8, 212u8, 83u8, 181u8, 13u8, 237u8,
    239u8, 242u8, 36u8,
]);

pub struct ProposalStorage {
    pub voters: Vec<Pubkey>,
}

impl borsh::ser::BorshSerialize for ProposalStorage
where
    Vec<Pubkey>: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.voters, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for ProposalStorage
where
    Vec<Pubkey>: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            voters: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl Clone for ProposalStorage {
    #[inline]
    fn clone(&self) -> ProposalStorage {
        ProposalStorage {
            voters: ::core::clone::Clone::clone(&self.voters),
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for ProposalStorage {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
        if writer
            .write_all(&[89, 153, 129, 114, 0, 247, 159, 182])
            .is_err()
        {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        if AnchorSerialize::serialize(self, writer).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for ProposalStorage {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        if buf.len() < [89, 153, 129, 114, 0, 247, 159, 182].len() {
            return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[89, 153, 129, 114, 0, 247, 159, 182] != given_disc {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.name(),
                    error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .into(),
                    error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/solana-threshold-signature-program/src/lib.rs",
                            line: 901u32,
                        },
                    )),
                    compared_values: None,
                })
                .with_account_name("ProposalStorage"),
            );
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for ProposalStorage {
    const DISCRIMINATOR: [u8; 8] = [89, 153, 129, 114, 0, 247, 159, 182];
}
#[automatically_derived]
impl anchor_lang::Owner for ProposalStorage {
    fn owner() -> Pubkey {
        CAMBRIAN_ID
    }
}

impl ProposalStorage {
    pub fn get_voter_storage(
        &self,
        voter: &Pubkey,
        account_space: usize,
        threshold: usize,
    ) -> Option<(usize, usize)> {
        let voter_space = account_space
            - (DISCRIMINATOR_SIZE
                + (mem::size_of::<usize>() + (threshold * mem::size_of::<Pubkey>())));
        self.voters
            .iter()
            .position(|key| key == voter)
            .map(|position| {
                let start = DISCRIMINATOR_SIZE
                    + (mem::size_of::<usize>() + (threshold * mem::size_of::<Pubkey>()))
                    + (voter_space / threshold) * position;
                let end = start + voter_space / threshold;
                (start, end)
            })
    }
}

pub struct PoAState {
    pub threshold: u64,
    pub admin: Pubkey,
    pub ncn: Pubkey,
    pub supported_token: Pubkey,
    pub stake_threshold: u64,
    pub bump: u8,
    pub executor_bump: u8,
}
impl borsh::ser::BorshSerialize for PoAState
where
    u64: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.threshold, writer)?;
        borsh::BorshSerialize::serialize(&self.admin, writer)?;
        borsh::BorshSerialize::serialize(&self.ncn, writer)?;
        borsh::BorshSerialize::serialize(&self.supported_token, writer)?;
        borsh::BorshSerialize::serialize(&self.stake_threshold, writer)?;
        borsh::BorshSerialize::serialize(&self.bump, writer)?;
        borsh::BorshSerialize::serialize(&self.executor_bump, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for PoAState
where
    u64: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            threshold: borsh::BorshDeserialize::deserialize_reader(reader)?,
            admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ncn: borsh::BorshDeserialize::deserialize_reader(reader)?,
            supported_token: borsh::BorshDeserialize::deserialize_reader(reader)?,
            stake_threshold: borsh::BorshDeserialize::deserialize_reader(reader)?,
            bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            executor_bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PoAState {
    #[inline]
    fn clone(&self) -> PoAState {
        PoAState {
            threshold: ::core::clone::Clone::clone(&self.threshold),
            admin: ::core::clone::Clone::clone(&self.admin),
            ncn: ::core::clone::Clone::clone(&self.ncn),
            supported_token: ::core::clone::Clone::clone(&self.supported_token),
            stake_threshold: ::core::clone::Clone::clone(&self.stake_threshold),
            bump: ::core::clone::Clone::clone(&self.bump),
            executor_bump: ::core::clone::Clone::clone(&self.executor_bump),
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for PoAState {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
        if writer
            .write_all(&[94, 100, 2, 153, 9, 183, 61, 141])
            .is_err()
        {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        if AnchorSerialize::serialize(self, writer).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for PoAState {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        if buf.len() < [94, 100, 2, 153, 9, 183, 61, 141].len() {
            return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[94, 100, 2, 153, 9, 183, 61, 141] != given_disc {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.name(),
                    error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .into(),
                    error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                        .to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/solana-threshold-signature-program/src/lib.rs",
                            line: 785u32,
                        },
                    )),
                    compared_values: None,
                })
                .with_account_name("PoAState"),
            );
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for PoAState {
    const DISCRIMINATOR: [u8; 8] = [94, 100, 2, 153, 9, 183, 61, 141];
}
#[automatically_derived]
impl anchor_lang::Owner for PoAState {
    fn owner() -> Pubkey {
        CAMBRIAN_ID
    }
}

impl PoAState {
    pub const SEED: &'static [u8] = b"STATE";
}

#[cfg(feature = "idl-build")]
impl IdlBuild for PoAState {
    fn create_type() -> Option<anchor_lang::idl::types::IdlTypeDef> {
        Some(anchor_lang::idl::types::IdlTypeDef {
            name: Self::get_full_path(),
            docs: Vec::new(),
            serialization: anchor_lang::idl::types::IdlSerialization::default(),
            repr: None,
            generics: Vec::new(),
            ty: anchor_lang::idl::types::IdlTypeDefTy::Struct {
                fields: Some(anchor_lang::idl::types::IdlDefinedFields::Named(
                    <[_]>::into_vec(Box::new([
                        anchor_lang::idl::types::IdlField {
                            name: "threshold".into(),
                            docs: Vec::new(),
                            ty: anchor_lang::idl::types::IdlType::U64,
                        },
                        anchor_lang::idl::types::IdlField {
                            name: "admin".into(),
                            docs: Vec::new(),
                            ty: anchor_lang::idl::types::IdlType::Pubkey,
                        },
                        anchor_lang::idl::types::IdlField {
                            name: "ncn".into(),
                            docs: Vec::new(),
                            ty: anchor_lang::idl::types::IdlType::Pubkey,
                        },
                        anchor_lang::idl::types::IdlField {
                            name: "supported_token".into(),
                            docs: Vec::new(),
                            ty: anchor_lang::idl::types::IdlType::Pubkey,
                        },
                        anchor_lang::idl::types::IdlField {
                            name: "stake_threshold".into(),
                            docs: Vec::new(),
                            ty: anchor_lang::idl::types::IdlType::U64,
                        },
                        anchor_lang::idl::types::IdlField {
                            name: "bump".into(),
                            docs: Vec::new(),
                            ty: anchor_lang::idl::types::IdlType::U8,
                        },
                        anchor_lang::idl::types::IdlField {
                            name: "executor_bump".into(),
                            docs: Vec::new(),
                            ty: anchor_lang::idl::types::IdlType::U8,
                        },
                    ])),
                )),
            },
        })
    }
    fn insert_types(
        types: &mut std::collections::BTreeMap<String, anchor_lang::idl::types::IdlTypeDef>,
    ) {
    }
    fn get_full_path() -> String {
        {
            let res = fmt::format(format_args!(
                "{0}::{1}",
                "solana_threshold_signature_program", "PoAState",
            ));
            res
        }
    }
}
