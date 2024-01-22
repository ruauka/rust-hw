#[derive(Debug)]
pub struct Account {
    pub balance: i64,
}

impl Account {
    pub fn new(balance: i64) -> Self {
        Self { balance }
    }

    pub fn change_balance(&mut self, new_balance: i64) {
        self.balance = new_balance
    }
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance == 0 {
            println!("Аккуант успешно удален!");
            return;
        }

        println!("Аккаунт живой! Баланс: {}", self.balance)
    }
}

pub fn print_balance(account: &Account) {
    println!("{}", account.balance)
}

pub fn transfer_funds(account1: &mut Account, account2: &mut Account, value: i64) {
    account1.balance += value;
    account2.balance -= value;
}

pub fn destroy_account(account1: &mut Account, mut account2: Account) {
    account1.balance += account2.balance;
    account2.balance = 0;
}