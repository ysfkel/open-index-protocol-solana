use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Index {
    pub id: u64,
    pub owner: Pubkey,
    pub manager: Pubkey,
    pub initialized: bool,
    pub bump: u8,
}

impl Index {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1;
    pub fn new(id: u64, owner: Pubkey, manager: Pubkey, bump: u8) -> Self {
        Self {
            id,
            owner,
            manager,
            initialized: true,
            bump,
        }
    }
}

impl IsInitialized for Index {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}
