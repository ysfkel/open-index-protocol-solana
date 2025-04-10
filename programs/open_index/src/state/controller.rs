use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Controller {
    pub id: u64,
    pub owner: Pubkey,
    pub next_index_id: u64,
    pub initialized: bool,
    pub bump: u8,
}

impl Controller {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1;

    pub fn new(id: u64, owner: Pubkey, bump: u8) -> Self {
        Self {
            id,
            owner,
            bump,
            initialized: true,
            next_index_id: 1,
        }
    }

    pub fn increment_next_index_id(&mut self) {
        self.next_index_id += 1;
    }

    pub fn get_next_index_id(&self) -> u64 {
        self.next_index_id
    }
}

impl IsInitialized for Controller {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}
