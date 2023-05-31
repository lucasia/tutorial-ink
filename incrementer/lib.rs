#![cfg_attr(not(feature = "std"), no_std)]

/// Exmaple:
/// #[ink::contract]
/// mod my_contract {
///     use super::*;
/// 
///     #[ink(storage)]
///     pub struct MyContract {
///         my_bool: bool,
///         my_number: u32,
/// 
///         my_account: Option<ink::primitives::AccountId>,
///         my_balance: Option<Balance>,
///     }
/// 
///     impl MyContract {
///         #[ink(constructor)]
///         pub fn new() -> Self {
///             MyContract {
///                 my_bool: false,
///                 my_number: 0,
///                 my_account: None,
///                 my_balance: None,
///             }
///         }
/// 
///         #[ink(message)]
///         pub fn get(&self) {
///             /// Contract Message
///         }
///     }
/// }
#[ink::contract]
mod incrementer {

    #[ink(storage)]
    pub struct Incrementer {
        // Storage Declaration
        value: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor
            Incrementer {
                value: init_value,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Incrementer {
                value: Default::default(),
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

    }
}

