#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[cfg(test)]
mod test;

// Custom Error enum for better error handling
/// Errors returned by the `LiquidityPool` contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InsufficientLiquidity = 2,
    SlippageExceeded = 3,
    InsufficientShares = 4,
    NotInitialized = 5,
    InsufficientBalance = 6,
    Unauthorized = 7,
}

// Event structures for state-changing operations
/// Event payload emitted after a successful deposit.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositEvent {
    /// Address that supplied liquidity.
    pub user: Address,
    /// Amount of token A deposited.
    pub amount_a: i128,
    /// Amount of token B deposited.
    pub amount_b: i128,
    /// LP shares minted for the depositor.
    pub shares_minted: i128,
}

/// Event payload emitted after a successful swap.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    /// Address that executed the swap.
    pub user: Address,
    /// Token address provided by the user.
    pub token_in: Address,
    /// Token address received by the user.
    pub token_out: Address,
    /// Amount of `token_in` transferred into the pool.
    pub amount_in: i128,
    /// Amount of `token_out` transferred out of the pool.
    pub amount_out: i128,
}

/// Event payload emitted after a successful withdrawal.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawEvent {
    /// Address that withdrew liquidity.
    pub user: Address,
    /// LP shares burned for this withdrawal.
    pub shares_burned: i128,
    /// Amount of token A withdrawn.
    pub amount_a: i128,
    /// Amount of token B withdrawn.
    pub amount_b: i128,
}

// Helper function: integer square root using Newton's method
fn sqrt(x: i128) -> i128 {
    if x == 0 {
        return 0;
    }

    let mut z = (x + 1) / 2;
    let mut y = x;

    while z < y {
        y = z;
        z = (x / z + z) / 2;
    }

    y
}

/// Storage keys used by the liquidity pool contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    TokenA,
    TokenB,
    ReserveA,
    ReserveB,
    ShareToken,
    TotalShares,
    Balance(Address),
}

#[contract]
/// Constant-product AMM liquidity pool with LP share accounting.
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPool {
    /// Initializes the liquidity pool once with token pair addresses.
    ///
    /// # Parameters
    /// - `e`: Soroban environment.
    /// - `token_a`: Contract address of token A.
    /// - `token_b`: Contract address of token B.
    ///
    /// # Returns
    /// - `Ok(())` when initialization succeeds.
    /// - `Err(Error::AlreadyInitialized)` if the pool was already initialized.
    pub fn initialize(e: Env, token_a: Address, token_b: Address) -> Result<(), Error> {
        if e.storage().instance().has(&DataKey::TokenA) {
            return Err(Error::AlreadyInitialized);
        }
        e.storage().instance().set(&DataKey::TokenA, &token_a);
        e.storage().instance().set(&DataKey::TokenB, &token_b);
        e.storage().instance().set(&DataKey::ReserveA, &0i128);
        e.storage().instance().set(&DataKey::ReserveB, &0i128);
        e.storage().instance().set(&DataKey::TotalShares, &0i128);
        Ok(())
    }

    /// Deposits token A and token B into the pool and mints LP shares.
    ///
    /// The caller (`to`) must authorize the transfer. For first liquidity,
    /// shares are minted as `sqrt(amount_a * amount_b)`. For subsequent
    /// deposits, shares are minted proportionally to existing reserves.
    ///
    /// # Parameters
    /// - `e`: Soroban environment.
    /// - `to`: Liquidity provider address receiving LP shares.
    /// - `amount_a`: Amount of token A to deposit.
    /// - `amount_b`: Amount of token B to deposit.
    ///
    /// # Returns
    /// - `Ok(i128)`: Number of LP shares minted.
    /// - `Err(Error::NotInitialized)`: Pool tokens were not configured.
    /// - `Err(Error::InsufficientLiquidity)`: Arithmetic failed (for example overflow).
    pub fn deposit(e: Env, to: Address, amount_a: i128, amount_b: i128) -> Result<i128, Error> {
        to.require_auth();

        // Transfer tokens to the contract
        let token_a_addr: Address = e
            .storage()
            .instance()
            .get(&DataKey::TokenA)
            .ok_or(Error::NotInitialized)?;
        let token_b_addr: Address = e
            .storage()
            .instance()
            .get(&DataKey::TokenB)
            .ok_or(Error::NotInitialized)?;

        // Soroban token interface standard: transfer(from, to, amount)
        let client_a = soroban_sdk::token::Client::new(&e, &token_a_addr);
        let client_b = soroban_sdk::token::Client::new(&e, &token_b_addr);

        client_a.transfer(&to, &e.current_contract_address(), &amount_a);
        client_b.transfer(&to, &e.current_contract_address(), &amount_b);

        let reserve_a: i128 = e.storage().instance().get(&DataKey::ReserveA).unwrap_or(0);
        let reserve_b: i128 = e.storage().instance().get(&DataKey::ReserveB).unwrap_or(0);
        let total_shares: i128 = e
            .storage()
            .instance()
            .get(&DataKey::TotalShares)
            .unwrap_or(0);

        let shares: i128 = if total_shares == 0 {
            // Initial liquidity: use sqrt(amount_a * amount_b) for proper CPMM formula
            // Check for overflow
            let product = amount_a
                .checked_mul(amount_b)
                .ok_or(Error::InsufficientLiquidity)?;
            sqrt(product)
        } else {
            // Proportional shares based on existing reserves
            let share_a = amount_a
                .checked_mul(total_shares)
                .ok_or(Error::InsufficientLiquidity)?
                / reserve_a;
            let share_b = amount_b
                .checked_mul(total_shares)
                .ok_or(Error::InsufficientLiquidity)?
                / reserve_b;
            if share_a < share_b {
                share_a
            } else {
                share_b
            }
        };

        // Mint shares (store balance in PERSISTENT storage)
        let user_share_key = DataKey::Balance(to.clone());
        let current_user_share: i128 = e.storage().persistent().get(&user_share_key).unwrap_or(0);
        e.storage()
            .persistent()
            .set(&user_share_key, &(current_user_share + shares));
        // Extend TTL for 100 ledgers max
        e.storage()
            .persistent()
            .extend_ttl(&user_share_key, 100, 100);

        e.storage()
            .instance()
            .set(&DataKey::TotalShares, &(total_shares + shares));

        // Update reserves
        e.storage()
            .instance()
            .set(&DataKey::ReserveA, &(reserve_a + amount_a));
        e.storage()
            .instance()
            .set(&DataKey::ReserveB, &(reserve_b + amount_b));

        // Emit deposit event
        e.events().publish(
            (String::from_str(&e, "deposit"), to.clone()),
            DepositEvent {
                user: to,
                amount_a,
                amount_b,
                shares_minted: shares,
            },
        );

        Ok(shares)
    }

    /// Swaps into one side of the pool using constant-product pricing with a 0.3% fee.
    ///
    /// If `buy_a` is `true`, the user buys token A by paying token B.
    /// Otherwise, the user buys token B by paying token A.
    ///
    /// # Parameters
    /// - `e`: Soroban environment.
    /// - `to`: Trader address performing the swap.
    /// - `buy_a`: Direction flag; `true` buys token A, `false` buys token B.
    /// - `out`: Exact amount of output token requested.
    /// - `in_max`: Maximum input amount the trader allows (slippage guard).
    ///
    /// # Returns
    /// - `Ok(i128)`: Actual input amount charged.
    /// - `Err(Error::NotInitialized)`: Pool tokens were not configured.
    /// - `Err(Error::InsufficientLiquidity)`: Requested `out` exceeds available reserve.
    /// - `Err(Error::SlippageExceeded)`: Required input is greater than `in_max`.
    pub fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128) -> Result<i128, Error> {
        to.require_auth();

        let token_a: Address = e
            .storage()
            .instance()
            .get(&DataKey::TokenA)
            .ok_or(Error::NotInitialized)?;
        let token_b: Address = e
            .storage()
            .instance()
            .get(&DataKey::TokenB)
            .ok_or(Error::NotInitialized)?;
        let reserve_a: i128 = e.storage().instance().get(&DataKey::ReserveA).unwrap_or(0);
        let reserve_b: i128 = e.storage().instance().get(&DataKey::ReserveB).unwrap_or(0);

        let (reserve_in, reserve_out, token_in, token_out) = if buy_a {
            (reserve_b, reserve_a, token_b.clone(), token_a.clone()) // Buying A means paying with B
        } else {
            (reserve_a, reserve_b, token_a.clone(), token_b.clone()) // Buying B means paying with A
        };

        // K = Rin * Rout
        // (Rin + AmountIn) * (Rout - AmountOut) = K
        // Rin * Rout + AmountIn * Rout - Rin * AmountOut - AmountIn * AmountOut = Rin * Rout
        // AmountIn * (Rout - AmountOut) = Rin * AmountOut
        // AmountIn = (Rin * AmountOut) / (Rout - AmountOut)
        // Add 0.3% fee: AmountInWithFee = AmountIn * 1000 / 997

        if out >= reserve_out {
            return Err(Error::InsufficientLiquidity);
        }

        let numerator = reserve_in * out * 1000;
        let denominator = (reserve_out - out) * 997;
        let amount_in = (numerator / denominator) + 1;

        if amount_in > in_max {
            return Err(Error::SlippageExceeded);
        }

        // Transfer In
        let client_in = soroban_sdk::token::Client::new(&e, &token_in);
        client_in.transfer(&to, &e.current_contract_address(), &amount_in);

        // Transfer Out
        let client_out = soroban_sdk::token::Client::new(&e, &token_out);
        client_out.transfer(&e.current_contract_address(), &to, &out);

        // Update Reserves
        if buy_a {
            e.storage()
                .instance()
                .set(&DataKey::ReserveA, &(reserve_a - out));
            e.storage()
                .instance()
                .set(&DataKey::ReserveB, &(reserve_b + amount_in));
        } else {
            e.storage()
                .instance()
                .set(&DataKey::ReserveA, &(reserve_a + amount_in));
            e.storage()
                .instance()
                .set(&DataKey::ReserveB, &(reserve_b - out));
        }

        // Emit swap event
        e.events().publish(
            (String::from_str(&e, "swap"), to.clone()),
            SwapEvent {
                user: to,
                token_in,
                token_out,
                amount_in,
                amount_out: out,
            },
        );

        Ok(amount_in)
    }

    /// Burns LP shares and withdraws proportional token A and token B reserves.
    ///
    /// # Parameters
    /// - `e`: Soroban environment.
    /// - `to`: Liquidity provider address receiving withdrawn tokens.
    /// - `share_amount`: Number of LP shares to burn.
    ///
    /// # Returns
    /// - `Ok((i128, i128))`: Tuple `(amount_a, amount_b)` withdrawn.
    /// - `Err(Error::InsufficientShares)`: User does not own enough LP shares.
    /// - `Err(Error::NotInitialized)`: Pool state is incomplete or not initialized.
    pub fn withdraw(e: Env, to: Address, share_amount: i128) -> Result<(i128, i128), Error> {
        to.require_auth();

        let user_share_key = DataKey::Balance(to.clone());
        let current_user_share: i128 = e.storage().persistent().get(&user_share_key).unwrap_or(0);
        if share_amount > current_user_share {
            return Err(Error::InsufficientShares);
        }

        let total_shares: i128 = e
            .storage()
            .instance()
            .get(&DataKey::TotalShares)
            .ok_or(Error::NotInitialized)?;
        let reserve_a: i128 = e.storage().instance().get(&DataKey::ReserveA).unwrap_or(0);
        let reserve_b: i128 = e.storage().instance().get(&DataKey::ReserveB).unwrap_or(0);

        let amount_a = share_amount * reserve_a / total_shares;
        let amount_b = share_amount * reserve_b / total_shares;

        // Burn shares (persistent storage)
        e.storage()
            .persistent()
            .set(&user_share_key, &(current_user_share - share_amount));
        e.storage()
            .persistent()
            .extend_ttl(&user_share_key, 100, 100);

        e.storage()
            .instance()
            .set(&DataKey::TotalShares, &(total_shares - share_amount));

        // Update reserves
        e.storage()
            .instance()
            .set(&DataKey::ReserveA, &(reserve_a - amount_a));
        e.storage()
            .instance()
            .set(&DataKey::ReserveB, &(reserve_b - amount_b));

        // Transfer tokens back
        let token_a: Address = e
            .storage()
            .instance()
            .get(&DataKey::TokenA)
            .ok_or(Error::NotInitialized)?;
        let token_b: Address = e
            .storage()
            .instance()
            .get(&DataKey::TokenB)
            .ok_or(Error::NotInitialized)?;

        let client_a = soroban_sdk::token::Client::new(&e, &token_a);
        let client_b = soroban_sdk::token::Client::new(&e, &token_b);

        client_a.transfer(&e.current_contract_address(), &to, &amount_a);
        client_b.transfer(&e.current_contract_address(), &to, &amount_b);

        // Emit withdraw event
        e.events().publish(
            (String::from_str(&e, "withdraw"), to.clone()),
            WithdrawEvent {
                user: to,
                shares_burned: share_amount,
                amount_a,
                amount_b,
            },
        );

        Ok((amount_a, amount_b))
    }

    // ========== Token Interface Methods ==========
    // Make LP shares compatible with Soroban Token standard

    /// Returns the LP token display name.
    pub fn name(e: Env) -> String {
        String::from_str(&e, "Liquidity Pool Share")
    }

    /// Returns the LP token symbol.
    pub fn symbol(e: Env) -> String {
        String::from_str(&e, "LPS")
    }

    /// Returns the LP token decimals.
    pub fn decimals(_e: Env) -> u32 {
        7
    }

    /// Returns the LP token balance of `id`.
    pub fn balance(e: Env, id: Address) -> i128 {
        let key = DataKey::Balance(id);
        e.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Returns total outstanding LP token supply.
    pub fn total_supply(e: Env) -> i128 {
        e.storage()
            .instance()
            .get(&DataKey::TotalShares)
            .unwrap_or(0)
    }

    /// Transfers LP shares from `from` to `to`.
    ///
    /// Returns `Err(Error::InsufficientBalance)` when `from` lacks enough shares.
    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();

        let from_key = DataKey::Balance(from.clone());
        let to_key = DataKey::Balance(to.clone());

        let from_balance = e.storage().persistent().get(&from_key).unwrap_or(0);
        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        e.storage()
            .persistent()
            .set(&from_key, &(from_balance - amount));
        e.storage().persistent().extend_ttl(&from_key, 100, 100);

        let to_balance = e.storage().persistent().get(&to_key).unwrap_or(0);
        e.storage()
            .persistent()
            .set(&to_key, &(to_balance + amount));
        e.storage().persistent().extend_ttl(&to_key, 100, 100);

        Ok(())
    }
}
