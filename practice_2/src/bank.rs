use crate::account::Account;

#[derive(Debug)]
pub struct Bank {
    accounts: Vec<Account>,
    credit_rate: u32,
    debit_rate: u32,
}

impl Bank {
    pub fn new(accounts: Vec<Account>, credit_rate: u32, debit_rate: u32) -> Self {
        Self {
            accounts,
            credit_rate,
            debit_rate,
        }
    }
}

pub fn accrue_interest(bank: &mut Bank) { todo!() }

pub fn bank_balance(bank: &Bank) -> (u64, u64) { todo!() }

pub fn merge_banks(b1: &mut Bank, mut b2: Bank) { todo!() }