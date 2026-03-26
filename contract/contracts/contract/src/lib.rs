#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, String};

// Key lưu greeting / message
const MESSAGE: Symbol = symbol_short!("MSG");

#[contract]
pub struct RemittanceVisualizerContract;

#[contractimpl]
impl RemittanceVisualizerContract {

    // Gửi "remittance" (mô phỏng) + lưu message
    pub fn send_remittance(env: Env, sender: String, receiver: String, amount: i128) -> String {
        // Lưu thông tin giao dịch đơn giản
        let info = String::from_str(
            &env,
            "Remittance sent via Stellar (testnet)"
        );

        env.storage().instance().set(&MESSAGE, &info);

        info
    }

    // Lấy thông tin giao dịch gần nhất
    pub fn get_last_remittance(env: Env) -> String {
        env.storage()
            .instance()
            .get(&MESSAGE)
            .unwrap_or(String::from_str(&env, "No transaction yet"))
    }

    // So sánh phí SWIFT vs Stellar
    pub fn compare_fee(env: Env, amount: i128) -> String {
        // SWIFT ~5%
        let swift_fee = amount * 5 / 100;

        // Stellar fee ~0.000003 XLM ≈ gần 0 với số nhỏ
        let stellar_fee = 1; // giả lập rất nhỏ

        let result = String::from_str(
            &env,
            "SWIFT fee ~5%, Stellar fee ~0.000003 XLM"
        );

        result
    }
}