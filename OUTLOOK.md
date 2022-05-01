# VoydWalkr Vanguard
*The Vanguard of the VoydWalkr fleet* courageously storms ahead to charter and conquer new territory.

*Vanguard* is an [L2](#what-is-layer-2) sitting atop the [Terra](https://terra.money) blockchain.

As an L2 network, *VoydWalkr Vanguard*'s objective is similar to [Solana's Snowflake Protocol](https://snowflake.sol): **Automation**. Due to limited system resources, contemporary blockchain solutions require explicit execution of transactions, meaning it is impossible to schedule a future transaction or to react to changes in the ecosystem. *Vanguard* addresses this pain point.

A brief overview of Vanguard's operation:
1. User creates a *MicroVault*.
2. User issues *Orders* & assigns *Execution Conditions*.
3. *Vanguard Runner* continuously observes market.
4. *Vanguard Runner* detects an *Order*'s *Conditions* are met & initiates execution.
5. *Vanguard Collective* confirms or denies market conditions.
6. If the conditions are indeed met, *Runner* executes *Order* through *MicroVault*.
7. *MicroVault* ascertains *Orders* are signed by legitimate owner to enforce safety of funds on-chain.

*[TODO: create a nice graphic :)]*

# What is Layer 2?
*Layer 2*, commonly abbreviated as *L2*, much like *Layer 1*, is a decentralized, permissionless network sitting atop usually one specific *Layer 1* blockchain network. Two prominent *L2* solutions are Bitcoin's [Lightning Network](https://lightning.network) and Ethereum's [Optimism](https://optimism.io). These differ from sidechains like [Polygon (Ethereum Sidechain)](https://polygon.technology) or [RSK (Bitcoin Sidechain)](https://rsk.co), which are independent blockchains connecting to the parent chain through a two-way peg and leverage its security features for fully finalized transactions.

Unlike a sidechain, an L2 is not necessarily even a blockchain of its own. Point in case: The *Lightning Network* is a network brokering microtransactions directly between peers which are eventually finalized on the Bitcoin blockchain. Similarly, *Vanguard* handles various challenges off-chain to lighten the overall footprint on the blockchain.

For instance, in *Vanguard*'s case, some *Order Execution Conditions* depend on price movements or ranges. Within a blockchain, only data stored on-chain is accessible. In order to access asset prices, we would require a [Price Oracle](https://whiteboardcrypto.com/what-are-oracles-in-crypto/). Bringing these prices on-chain is subject to the underlying blockchain's systemic conditions:

- Prices cannot be updated in imminent realtime,
- Prices occupy permanent storage of the network,
- Executing Smart Contracts incurrs gas fees, and
- The network may become congested.

*Vanguard* is only secondarily interested in prices. We only require them to verify *order execution conditions* and may even lose interest in an asset pair once its last associated order has been fulfilled. Thus, by shifting price consensus off-chain, we take only what data we need, when we need it, collectively find an agreement, and fulfill our purpose without creating the additional overhead above.
