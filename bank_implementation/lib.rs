#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bank {
    use ink::storage::Mapping;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Bank {
        balances: Mapping<AccountId, Balance>,
        admin: AccountId
    }

    impl Bank {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self {
                balances: Mapping::default(),
                admin
            }
        }


        #[ink(message, payable)]
        pub fn send(&mut self) {
            let caller = self.env().caller(); // this is the current caller
            let balance = self.balances.get(caller).unwrap_or(0);  // this is the amount of native token this user has sent into the contract
            let endowment = self.env().transferred_value(); // this is how much the user is coming with now
            self.balances.insert(caller, &(balance + endowment)); // updating the balance for this user 
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: u128) {
            let caller = self.env().caller(); // obtaining the current caller 
            let pre_balance = self.balances.get(caller).unwrap(); // obtained the user balance this contract is storing
            self.balances.insert(caller, &(pre_balance - amount)); // updating the balance for this user 
            self.env().transfer(caller, amount).unwrap() // making the withdraw transfer and using the unwrap to be sure there is no error
        }



        #[ink(message)]
        pub fn get_balance(&self) -> u128 {
            let caller = self.env().caller();
            self.balances.get(caller).unwrap()
        }
    }
}
