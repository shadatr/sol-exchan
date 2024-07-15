
// use anchor_lang::{pubkey, solana_program::pubkey::Pubkey};
// use serde_json::{from_str, json, Value};
// use borsh::{BorshDeserialize, BorshSerialize};
// use anyhow::{bail, Error};
// use solana_client::{client_error::reqwest::blocking::Client, nonblocking::rpc_client::RpcClient};

// use std::str::FromStr;


// pub fn get_ray_pool_id(market_address: &Pubkey) -> Pubkey {
//     Pubkey::find_program_address(
//         &[
//             &pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").to_bytes(),
//             &market_address.to_bytes(),
//             &b"amm_associated_seed".as_slice(),
//         ],
//         &pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"),
//     )
//     .0
// }

// pub async fn query_lp_by_token(base: &str, quote: &str) -> anyhow::Result<Vec<Pubkey>> {
//     let client = Client::new();

//     let query = r#"
//     query MyQuery($where: Raydium_LiquidityPoolv4_bool_exp) {
//         Raydium_LiquidityPoolv4(where: $where) {
//             marketId
//         }
//     }"#;

//     let variables = json!({
//         "where": {
//             "_and": [
//                 {"baseMint": {"_eq": base}},
//                 {"quoteMint": {"_eq": quote}}
//             ]
//         }
//     });

//     let body = json!({
//         "query": query,
//         "variables": variables
//     });

//     let mut resp_body = String::new();
//     for _ in 0..10 {
//         let res = client
//             .post("https://programs.shyft.to/v0/graphql/?api_key=WFPB-yulsLr0VTwy")
//             .json(&body)
//             .send()
//             .expect("Failed to send request");

//         resp_body = res.text().expect("Failed to get response body");
//         // if resp_body.contains("RateLimitExceeded") {
//         //     tokio::time::sleep(Duration::from_secs(2)).await;
//         // } else {
//         //     break;
//         // }
//     }

//     if resp_body.contains("RateLimitExeceeded") {
//         bail!("Rate limit exceeded");
//     };

//     let res = from_str::<Value>(&resp_body).unwrap();

//     let market_ids = res["data"]["Raydium_LiquidityPoolv4"]
//         .as_array()
//         .unwrap_or(&Vec::new())
//         .iter()
//         .map(|pool| {
//             Pubkey::from_str(&pool["marketId"].as_str().map(String::from).unwrap()).unwrap()
//         })
//         .collect();

//     Ok(market_ids)
// }

// pub async fn get_token_price(
//     client: &RpcClient,
//     mint: Pubkey,
//     openbook_id: Option<Pubkey>,
// ) -> Result<(f64, Pubkey), Error> {
//     let mut openbook_pubkey = None;
//     let mut pool_account_info: Option<Vec<u8>> = match openbook_id {
//         None => {
//             let openbook_market_ids = query_lp_by_token(
//                 "So11111111111111111111111111111111111111112",
//                 &mint.to_string(),
//             )
//             .await
//             .unwrap();
//             let mut data = None;
//             for id in openbook_market_ids {
//                 let raydium_id = get_ray_pool_id(&id);
//                 if let Ok(account_data) = client.get_account_data(&raydium_id).await {
//                     data = Some(account_data);
//                     openbook_pubkey = Some(id);
//                     break;
//                 }
//             }
//             data
//         }
//         Some(openbook_id) => {
//             let raydium_id = get_ray_pool_id(&openbook_id);
//             openbook_pubkey = Some(openbook_id);
//             Some(client.get_account_data(&raydium_id).await.unwrap())
//         }
//     };
//     if pool_account_info.is_none() {
//         let openbook_market_ids = query_lp_by_token(
//             &mint.to_string(),
//             "So11111111111111111111111111111111111111112",
//         )
//         .await
//         .unwrap();
//         for id in openbook_market_ids {
//             let raydium_id = get_ray_pool_id(&id);
//             if let Ok(account_data) = client.get_account_data(&raydium_id).await {
//                 pool_account_info = Some(account_data);
//                 openbook_pubkey = Some(id);
//                 break;
//             }
//         }
//     }

//     if pool_account_info.is_none() {
//         return Err(Error::msg("No pool account found"));
//     }

//     let pool_info = LiquidityStateV4::try_from_slice(&pool_account_info.unwrap()).unwrap();
//     let base_decimals = 10u64.pow(pool_info.base_decimal as u32) as f64;
//     let quote_decimals = 10u64.pow(pool_info.quote_decimal as u32) as f64;

//     let base_token_amount_future = client.get_token_account_balance(&pool_info.base_vault);
//     let quote_token_amount_future = client.get_token_account_balance(&pool_info.quote_vault);
//     let open_orders_future = fetch_open_orders(client, pool_info.open_orders);

//     let (base_token_amount_res, quote_token_amount_res, open_orders) = join!(
//         base_token_amount_future,
//         quote_token_amount_future,
//         open_orders_future
//     );

//     let base_token_amount = base_token_amount_res.expect("Failed to get base token amount");
//     let quote_token_amount = quote_token_amount_res.expect("Failed to get quote token amount");

//     let base_pnl = pool_info.base_need_take_pnl as f64 / base_decimals;
//     let quote_pnl = pool_info.quote_need_take_pnl as f64 / quote_decimals;
//     let open_orders_base_token_total = open_orders.native_coin_total as f64;
//     let open_orders_quote_token_total = open_orders.native_pc_total as f64;

//     let base_token_amount =
//         u64::from_str(&base_token_amount.amount).unwrap() as f64 / base_decimals;
//     let quote_token_amount =
//         u64::from_str(&quote_token_amount.amount).unwrap() as f64 / quote_decimals;

//     let base = base_token_amount + open_orders_base_token_total - base_pnl;
//     let quote = quote_token_amount + open_orders_quote_token_total - quote_pnl;

//     Ok((base / quote, openbook_pubkey.unwrap()))
// }

// pub async fn fetch_open_orders(client: &RpcClient, open_orders: Pubkey) -> OpenOrders {
//     let open_orders = client.get_account_data(&open_orders).await.unwrap();

//     let data = open_orders.split_at(3216).0;

//     OpenOrders::try_from_slice(data).unwrap()
// }

// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
// pub struct OpenOrders {
//     pub account_flags: u64, 
//     pub market: [u64; 4],
//     pub owner: [u64; 4],

//     pub native_coin_free: u64,
//     pub native_coin_total: u64,

//     pub native_pc_free: u64,
//     pub native_pc_total: u64,

//     pub free_slot_bits: u128,
//     pub is_bid_bits: u128,
//     pub orders: [u128; 128],
//     // Using Option<NonZeroU64> in a pod type requires nightly
//     pub client_order_ids: [u64; 128],
//     pub referrer_rebates_accrued: u64,
// }

// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
// pub struct LiquidityStateV4 {
//     pub status: u64,
//     pub nonce: u64,
//     pub max_order: u64,
//     pub depth: u64,
//     pub base_decimal: u64,
//     pub quote_decimal: u64,
//     pub state: u64,
//     pub reset_flag: u64,
//     pub min_size: u64,
//     pub vol_max_cut_ratio: u64,
//     pub amount_wave_ratio: u64,
//     pub base_lot_size: u64,
//     pub quote_lot_size: u64,
//     pub min_price_multiplier: u64,
//     pub max_price_multiplier: u64,
//     pub system_decimal_value: u64,
//     pub min_separate_numerator: u64,
//     pub min_separate_denominator: u64,
//     pub trade_fee_numerator: u64,
//     pub trade_fee_denominator: u64,
//     pub pnl_numerator: u64,
//     pub pnl_denominator: u64,
//     pub swap_fee_numerator: u64,
//     pub swap_fee_denominator: u64,
//     pub base_need_take_pnl: u64,
//     pub quote_need_take_pnl: u64,
//     pub quote_total_pnl: u64,
//     pub base_total_pnl: u64,
//     pub pool_open_time: u64,
//     pub punish_pc_amount: u64,
//     pub punish_coin_amount: u64,
//     pub orderbook_to_init_time: u64,
//     pub swap_base_in_amount: u128,
//     pub swap_quote_out_amount: u128,
//     pub swap_base2quote_fee: u64,
//     pub swap_quote_in_amount: u128,
//     pub swap_base_out_amount: u128,
//     pub swap_quote2base_fee: u64,
//     pub base_vault: Pubkey,
//     pub quote_vault: Pubkey,
//     pub base_mint: Pubkey,
//     pub quote_mint: Pubkey,
//     pub lp_mint: Pubkey,
//     pub open_orders: Pubkey,
//     pub market_id: Pubkey,
//     pub market_program_id: Pubkey,
//     pub target_orders: Pubkey,
//     pub withdraw_queue: Pubkey,
//     pub lp_vault: Pubkey,
//     pub owner: Pubkey,
//     pub lp_reserve: u64,
//     pub padding: [u64; 3],
// }