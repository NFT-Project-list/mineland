use crate::*;

impl Contract {
    pub(crate) fn add_token_storage(&mut self, account_id: &AccountId) {
        let ft_mint_deposit: Balance = self.to_yocto("0.001");
        let ft_mint_gas: Gas = self.to_tera(10);

        Promise::new(self.contract_ft.clone()).function_call(
            b"ft_mint".to_vec(),
            json!({
            "receiver_id": account_id,
            "amount": "0"
        }).to_string().as_bytes().to_vec(),
            ft_mint_deposit,
            ft_mint_gas,
        );

        self.ft_storage_accounts.insert(account_id);
    }
}
