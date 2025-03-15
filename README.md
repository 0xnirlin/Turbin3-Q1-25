# ⚡ Turbine OTC Capstone

<div align="center">
  <p><em>A Decentralized Over-The-Counter Trading Protocol on Solana</em></p>
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Anchor](https://img.shields.io/badge/Built%20with-Anchor-blue)](https://www.anchor-lang.com/)
  [![Solana](https://img.shields.io/badge/Solana-Devnet-purple)](https://solana.com/)
</div>

---

## 🔍 Overview

Turbine OTC enables secure peer-to-peer token trades with fair, oracle-backed pricing. Create custom orders with negotiable premiums, target specific sellers, and trade with confidence.


## 💡 Origin
The idea for Turbine OTC came from being in VIP groups of late-stage meme coin communities. A common problem was devs working hard to grow market cap, but every time the price pumped, someone dumped. Devs often offered to buy OTC, but trust was always an issue. Existing solutions require manual price management due to fluctuations, making them impractical. With Switchboard’s on-demand price feeds, we can generate real-time pricing for any meme token, making OTC trading seamless and trustless.

## ✨ Features

- **🔓 Permissionless Trading** - Create buy orders for any whitelisted token
- **💰 Premium Negotiation** - Set custom premiums within protocol bounds
- **📊 Oracle Integration** - Uses Switchboard price feeds for real-time market pricing
- **🎯 Seller Targeting** - Optionally target specific sellers for private deals
- **⏱️ Time-Bound Orders** - Set expiration timestamps for automatic order invalidation
- **✅ Token Whitelisting** - Only verified tokens can be traded for enhanced security

## 🔄 How It Works

<div align="center">
  <table>
    <tr>
      <th width="33%">📝 Create Order</th>
      <th width="33%">🤝 Fulfill Order</th>
      <th width="33%">❌ Cancel Order</th>
    </tr>
    <tr>
      <td>
        <ul>
          <li>Specify token, SOL amount</li>
          <li>Set premium & expiration</li>
          <li>SOL is locked in vault</li>
        </ul>
      </td>
      <td>
        <ul>
          <li>Oracle provides price</li>
          <li>Premium is applied</li>
          <li>Assets are exchanged</li>
        </ul>
      </td>
      <td>
        <ul>
          <li>Only creator can cancel</li>
          <li>SOL is fully returned</li>
          <li>Order is invalidated</li>
        </ul>
      </td>
    </tr>
  </table>
</div>

## 📊 Core Components

### OrderMaker

```
buyer: Pubkey              // Order creator's address
token_mint: Pubkey         // Token to purchase
sol_amount: u64            // Locked SOL amount
seller: Option<Pubkey>     // Optional seller address
premium: u16               // Discount percentage from market price
expiration_timestamp: u64  // Order validity period
seed: u64                  // Unique order identifier
```

### TurbineConfig

```
fee_percentage: u16       // Protocol fee
max/min_fee_percentage: u16  // Fee bounds
max/min_premium: u16      // Premium bounds
whitelisted_tokens: Vec<Pubkey>  // Approved tokens
feed_address: Vec<Pubkey>  // Price oracle addresses
```

## 📋 Instructions

### Initialize Config
```rust
pub fn init_config(
    ctx: Context<InitConfig>,
    fee_percentage: u16,
    max_fee_percentage: u16,
    min_fee_percentage: u16,
    max_premium: u16,
    min_premium: u16,
    owner: Pubkey,
    listing_fee: u16,
) -> Result<()>
```

### Create OTC Order
```rust
pub fn create_otc_order(
    ctx: Context<CreateOTCOrder>, 
    amount: u64, 
    seed: u64, 
    expiry_timestamp: u64, 
    premium: u16
) -> Result<Pubkey>
```

### Take OTC Order
```rust
pub fn take_otc_order(
    ctx: Context<TakeOTCOrder>
) -> Result<()>
```

### Cancel OTC Order
```rust
pub fn cancel_otc_order(
    ctx: Context<CreateOTCOrder>, 
    amount: u64, 
    seed: u64
) -> Result<()>
```

## 🔒 Security Considerations

- Program-derived address vaults secure all funds
- Oracle integration prevents price manipulation
- Configurable premium limits protect traders
- Expiration timestamps prevent stale orders



<div align="center">
  <p>Built with ❤️ by the Nirlin</p>
</div>
