#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// Create storage for a simple ERC-20 contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: Balance,
        /// Mapping from owner to number of owned tokens.
        balances: Mapping<ink::primitives::AccountId, Balance>,
    }

    impl Erc20 {
        /// Create a new ERC-20 contract with an initial supply.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();

            balances.insert(caller, &total_supply);

            Self {
                total_supply,
                balances,
            }
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        pub fn balance_of(&self, owner: ink::primitives::AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// transfer `value` to the `to` account
        #[ink(message)]
        pub fn transfer(&mut self, to:AccountId, value:Balance) -> Result<()> {
            let from = self.env().caller();
            self.transform_from_to(&from, &to, value)
        }


        fn transform_from_to(&mut self, from:&AccountId, to:&AccountId, value: Balance) -> Result<()> {

            // ---- deduct the value from `from` account balance ----
            let from_balance = self.balances.get(from).unwrap_or_default();

            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }

            // from_balance = from_balance - value;
            self.balances.insert(&from, &(from_balance - value));

            // ---- end block ----

            // ---- add the value to the `to` account balance ----
            let to_balance = self.balances.get(to).unwrap_or_default();
            self.balances.insert(to, &(to_balance + value));

            Ok(())
        }
    }

    /// Specify ERC-20 error type.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Return if the balance cannot fulfill a request.
        InsufficientBalance,
    }

    /// Specify the ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[cfg(test)]
    mod tests {
        use super::*;

        // We define some helper Accounts to make our tests more readable
        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn alice() -> ink::primitives::AccountId {
            default_accounts().alice
        }

        fn bob() -> ink::primitives::AccountId {
            default_accounts().bob
        }

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let orig_caller = ::ink::env::caller::<Environment>();
            assert_eq!(orig_caller, alice()); // alice is the original caller

            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(alice()), 100);
            assert_eq!(contract.balance_of(bob()), 0);
        }

        #[ink::test]
        fn change_caller() {
            let orig_caller = ::ink::env::caller::<Environment>();
            assert_eq!(orig_caller, alice()); // alice is the original caller

            ink::env::test::set_caller::<Environment>(bob()); // change our caller to bob
            assert_eq!(::ink::env::caller::<Environment>(), bob()); // verify
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);

            assert_eq!(contract.balance_of(alice()), 100);
            assert!(contract.transfer(bob(), 10).is_ok());
            assert_eq!(contract.balance_of(bob()), 10);
            assert!(contract.transfer(bob(), 100).is_err());
        }
    }
}