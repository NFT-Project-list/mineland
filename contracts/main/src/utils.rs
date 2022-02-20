use crate::*;

impl Contract {
    // Generate random u8 number (0-254)
    pub(crate) fn random_u8(&self, index: usize) -> u8 {
        *env::random_seed().get(index).unwrap()
    }

    // Get random number from 0 to max-1
    pub(crate) fn random_u8_range(&self, index: usize, max: usize) -> u8 {
        if max > 1 {
            let rand_divider = 256 / u32::try_from(max).unwrap();
            return self.random_u8(index) / u8::try_from(rand_divider).unwrap();
        }
        0.into()
    }

    // Convert f64 to yocto NEAR Balance
    pub(crate) fn to_yocto(&self, value: &str) -> Balance {
        let values: Vec<_> = value.split('.').collect();
        let part1 = values[0].parse::<u128>().unwrap() * 10u128.pow(24);
        if values.len() > 1 {
            let power = values[1].len() as u32;
            let part2 = values[1].parse::<u128>().unwrap() * 10u128.pow(24 - power);
            part1 + part2
        } else {
            part1
        }
    }

    // Convert u64 to yocto NEAR Gas
    pub(crate) fn to_tera(&self, tokens: u64) -> Gas {
        tokens * 10u128.pow(12) as u64
    }

    // pub(crate) fn assert_caller_contract(&self, contract: AccountId) {
    //     if env::predecessor_account_id() != contract {
    //         panic!("You can't call me directly");
    //     }
    // }

    pub(crate) fn assert_contract_owner(&self, owner_id: AccountId) {
        if env::predecessor_account_id() != owner_id {
            panic!("You can't add new types!");
        }
    }
}
