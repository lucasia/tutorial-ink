#![cfg_attr(not(feature = "std"), no_std)]

/// Example:
/// #[ink::contract]
/// mod mycontract {
///     use ink::storage::Mapping;
///
///     #[ink(storage)]
///     pub struct MyContract {
///         owner: ink::primitives::AccountId,
///
///         /// Store a mapping from AccountIds to a u32
///         my_map: Mapping<ink::primitives::AccountId, u32>,
///     }
///
///     impl MyContract {
///         #[ink(constructor)]
///         pub fn new(count: u32) -> Self {
///             let mut my_map = Mapping::default();
///
///             let caller = Self::env().caller();
///             my_map.insert(&caller, &count);
///
///             Self {
///                 owner: Self::env().caller(),
///                 my_map,
///             }
///         }
///
///         /// Get the number associated with the caller's AccountId, if it exists
///         #[ink(message)]
///         pub fn get(&self) -> u32 {
///             let caller = Self::env().caller();
///             self.my_map.get(&caller).unwrap_or_default()
///         }
///     }
///
///     #[cfg(test)]
///     mod tests {
///         use super::*;
///
///         #[ink::test]
///         fn constructor_works() {
///             let my_contract = MyContract::new(1);
///             assert_eq!(1, my_contract.get());
///         }
///     }
/// }
#[ink::contract]
mod incrementer {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Incrementer {
        // Storage Declaration
        value: i32,
        my_map: Mapping<ink::primitives::AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            let mut new_map = Mapping::default();
            let caller = Self::env().caller();
            new_map.insert(caller, &0);

            // Contract Constructor
            Incrementer {
                value: init_value,
                my_map: new_map,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Incrementer {
                value: Default::default(),
                my_map: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by:i32) {
            self.value += by
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller: ink::primitives::AccountId = Self::env().caller();

            self.my_map.get(caller).unwrap_or_default()
        }

        #[ink(message)]
        pub fn set_mine(&mut self, new_val: i32) {
            let caller = Self::env().caller();
            self.my_map.insert(caller, &new_val);
        }

        #[ink(message)]
        pub fn remove_mine(&self) {
            let caller = Self::env().caller();

            self.my_map.remove(caller);
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            assert_eq!(0, Incrementer::default().get())
        }

        #[ink::test]
        fn constructor_works() {
            let val = 309;
            assert_eq!(val, Incrementer::new(val).get());
        }

        #[test]
        fn test_inc() {
            let mut inc1 = Incrementer::default();

            assert_eq!(0, inc1.get());

            inc1.inc(1);
            assert_eq!(1, inc1.get());

            inc1.inc(-10);
            assert_eq!(-9, inc1.get());
        }

        #[ink::test]
        fn get_mine_works() {
            let contract = Incrementer::new(10);

            assert_eq!(10, contract.get());
            assert_eq!(0, contract.get_mine());
        }

        #[ink::test]
        fn set_mine_works() {
            let mut contract = Incrementer::default();
            assert_eq!(0, contract.get_mine());

            let new_val = 10;
            contract.set_mine(new_val);
            assert_eq!(new_val, contract.get_mine());
        }

        #[ink::test]
        fn remove_mine_works() {
            let mut contract = Incrementer::default();

            contract.set_mine(10);

            contract.remove_mine();
            assert_eq!(0, contract.get_mine());
        }

    }
}