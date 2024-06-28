use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};


#[derive(Debug)]
pub struct Token {
    _owner: Pubkey,
    _name: String,
    _total_supply: u64,
}

impl Token {
    pub fn new(_owner: Pubkey, _name: String, _total_supply: u64) -> Self {
        Self {
            _owner,
            _name,
            _total_supply,
        }
    }

    pub fn get_balance(&self, account: &AccountInfo) -> Result<u64, ProgramError> {
        let account_data = account.try_borrow_data()?;
        

        if account_data.len() != 8 {
            return Err(ProgramError::InvalidAccountData);
        }
    

        let account_data_array: [u8; 8] = account_data[..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
    
  
        let balance = u64::from_le_bytes(account_data_array);
    
        Ok(balance)
    }

    pub fn set_balance(
        &mut self,
        account: &AccountInfo,
        balance: u64,
    ) -> Result<(), ProgramError> {
        let mut account_data = account.try_borrow_mut_data()?;
        if account_data.len() != 8 {
            return Err(ProgramError::InvalidAccountData);
        }

        account_data.copy_from_slice(&balance.to_le_bytes());
        Ok(())
    }


    pub fn transfer(
        &mut self,
        _from: &AccountInfo,
        _to: &AccountInfo,
        _amount: u64,
    ) -> ProgramResult {
        Ok(())
    }

   
    pub fn approve(
        &mut self,
        _owner: &AccountInfo,
        _spender: &AccountInfo,
        _amount: u64,
    ) -> ProgramResult {
        Ok(())
    }

  
    pub fn transfer_from(
        &mut self,
        _owner: &AccountInfo,
        _spender: &AccountInfo,
        _to: &AccountInfo,
        _amount: u64,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn mint(&mut self, _recipient: &AccountInfo, _amount: u64) -> ProgramResult {
        let recipient_balance = self.get_balance(_recipient)?;
        let new_balance = recipient_balance.checked_add(_amount).ok_or(ProgramError::InvalidInstructionData)?;
        self.set_balance(_recipient, new_balance)?;
        Ok(())
    }
}

enum Instruction {
    Transfer,
    Approve,
    TransferFrom,
    Mint,
}


entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = match instruction_data.get(0) {
        Some(&0) => Instruction::Transfer,
        Some(&1) => Instruction::Approve,
        Some(&2) => Instruction::TransferFrom,
        Some(&3) => Instruction::Mint,
        _ => return Err(ProgramError::InvalidInstructionData),
    };

    
    
    let mut token = Token::new(
        *accounts[0].owner,
        "ExampleToken".to_string(),
        1_000_000_000, 
    );
    
    match instruction {
        Instruction::Transfer => {
            token.transfer(&accounts[1], &accounts[2], 100)?; 
        }
        Instruction::Approve => {
            token.approve(&accounts[1], &accounts[2], 100)?; 
        }
        Instruction::TransferFrom => {
            token.transfer_from(&accounts[1], &accounts[2], &accounts[3], 100)?; 
        }
        Instruction::Mint => {
            token.mint(&accounts[1], 100)?; 
        }
    }
    msg!("program_id: {}: token: {:?} data: {:?}", _program_id, token, instruction_data);

    msg!("Success!");
    Ok(())

    
}