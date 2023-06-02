#![cfg_attr(not(feature = "std"), no_std, no_main)]

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
        allowances: Mapping<(ink::primitives::AccountId, ink::primitives::AccountId), Balance>,
    }

    impl Erc20 {
        /// Create a new ERC-20 contract with an initial supply.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            let transfer_event = Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            };

            Self::env().emit_event(transfer_event);

            Self {
                total_supply,
                balances,
                allowances: Mapping::default(),
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

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner: AccountId = self.env().caller();
            let owner_balance = self.balances.get(owner).unwrap_or_default();

            if owner_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.allowances.insert((owner, spender), &value);

            let approval_event = Approval {
                owner: Some(owner),
                spender: Some(spender),
                allowance: value,
            };

            self.env().emit_event(approval_event);

            Ok(())
        }

        #[ink(message)]
        pub fn get_allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// transfer `value` to the `to` account
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transform_from_to(&from, &to, value)
        }


        /// transfer `value` to the `to` account
        #[ink(message)]
        pub fn transfer_from(&mut self, owner: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let spender = self.env().caller();

            let allowance_balance = self.allowances.get((owner, spender)).unwrap_or_default();

            if allowance_balance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.allowances.insert((owner, spender), &(allowance_balance - value));

            self.transform_from_to(&owner, &to, value)
        }

        fn transform_from_to(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
            // ---- deduct the value from `from` account balance ----
            let from_balance = self.balances.get(from).unwrap_or_default();

            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // from_balance = from_balance - value;
            self.balances.insert(&from, &(from_balance - value));

            // ---- end block ----

            // ---- add the value to the `to` account balance ----
            let to_balance = self.balances.get(to).unwrap_or_default();
            self.balances.insert(to, &(to_balance + value));

            let transfer_event = Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            };

            self.env().emit_event(transfer_event);

            Ok(())
        }
    }

    /// Specify ERC-20 error type.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Return if the balance cannot fulfill a request.
        InsufficientBalance,
        InsufficientAllowance,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,

        #[ink(topic)]
        to: Option<AccountId>,

        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,

        #[ink(topic)]
        spender: Option<AccountId>,

        allowance: Balance, // value approved to trande on behalf
    }

    /// Specify the ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;


/// Useful test methods, see: https:///use.ink/ink-vs-solidity/#unit-testing-off-chain
/// get the default accounts (alice, bob, ...)
/// let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
/// accounts.alice ///usage example
///
/// /// set which account calls the contract
/// ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);
///
/// /// get the contract's address
/// let callee = ink_env::account_id::<ink_env::DefaultEnvironment>();
///
/// /// set the contracts address.
/// /// by default, this is alice's account
/// ink_env::test::set_callee::<ink_env::DefaultEnvironment>(callee);
///
/// /// transfer native currency to the contract
/// ink_env::test::set_value_transferred::<ink_env::DefaultEnvironment>(2);
///
/// /// increase block number (and block timestamp).
/// /// this can be placed in a loop to advance the block many times
/// ink_env::test::advance_block::<ink_env::DefaultEnvironment>();
///
/// /// generate arbitrary AccountId
/// AccountId::from([0x01; 32]);
///
/// /// generate arbitrary Hash
/// Hash::from([0x01; 32])
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

        #[ink::test]
        fn allowance_works() {
            let mut contract = Erc20::new(100);

            assert_eq!(::ink::env::caller::<Environment>(), alice()); // alice is the original caller

            assert_eq!(contract.get_allowance(alice(), alice()), 0); // a bit funny, we aren't approving ourself

            // approve bob and assert
            contract.approve(bob(), 100);
            assert_eq!(contract.get_allowance(alice(), bob()), 100);

            // ovwerrite bob to 0 and assert
            contract.approve(bob(), 0);
            assert_eq!(contract.get_allowance(alice(), bob()), 0);
        }


        #[ink::test]
        fn approval_and_transfer_works() {
            let mut contract = Erc20::new(100);

            assert_eq!(::ink::env::caller::<Environment>(), alice()); // alice is the original caller

            // approve bob and assert
            contract.approve(bob(), 100);
            assert_eq!(contract.get_allowance(alice(), bob()), 100);

            assert_eq!(contract.balance_of(alice()), 100);
            assert_eq!(contract.balance_of(bob()), 0);


            // change to be bob !
            ink::env::test::set_caller::<Environment>(bob()); // change our caller to bob
            assert_eq!(::ink::env::caller::<Environment>(), bob()); // verify

            // bob approves 50 transfer to bob
            let _ = contract.approve(bob(), 100);
            let _ = contract.transfer_from(alice(), bob(), 50);

            assert_eq!(contract.balance_of(alice()), 50);
            assert_eq!(contract.balance_of(bob()), 50);
        }
    }
}