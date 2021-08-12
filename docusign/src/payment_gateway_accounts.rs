use anyhow::Result;

use crate::Client;

pub struct PaymentGatewayAccounts {
    client: Client,
}

impl PaymentGatewayAccounts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        PaymentGatewayAccounts {
            client,
        }
    }

    /**
* List payment gateway accounts.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/payment_gateway_accounts` endpoint.
*
* This method returns a list of payment gateway accounts and basic information about them.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn get_all(
&self,
account_id: &str,
) -> Result<crate::types::PaymentGatewayAccountsInfo> {
let url =
format!("/v2.1/accounts/{}/payment_gateway_accounts",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.get(&url, None).await
}


}