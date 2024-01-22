mod bank;
mod account;

use account::{Account, print_balance, transfer_funds, destroy_account};
use bank::{Bank, accrue_interest};

fn main() {
    let mut acc1: Account = Account::new(10);
    let mut acc2: Account = Account::new(20);

    let mut bank1: Bank = Bank::new(vec![acc1, acc2], 20, 10);
    println!("{:?}", bank1);

    accrue_interest(&mut bank1)

    // print_balance(&acc1);
    // print_balance(&acc2);
    //
    // transfer_funds(&mut acc1, &mut acc2, 1);
    //
    // print_balance(&acc1);
    // print_balance(&acc2);
    //
    // destroy_account(&mut acc1, acc2)
}
