//! The `config` module contains the [DriverConfig].

use ethers::{
    prelude::{k256::ecdsa::SigningKey, ContractCall, SignerMiddleware},
    providers::{Provider, Ws},
    signers::Wallet,
    types::Address,
};
use tokio::sync::{mpsc, Mutex};

/// The [PreparedCall] type is a [ContractCall] that uses the [SignerMiddleware] to sign transactions.
type PreparedCall = ContractCall<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>, Address>;

/// The [DriverConfig] struct contains the configuration for the [Driver].
#[derive(Debug)]
pub struct DriverConfig {
    /// The URL of the RPC endpoint used to index and send transactions.
    pub ws_endpoint: String,
    /// The sending handle of the MPSC channel used to send transactions.
    pub tx_sender: mpsc::Sender<PreparedCall>,
    /// The receiving handle of the MPSC channel used to send transactions.
    pub tx_receiver: Mutex<mpsc::Receiver<PreparedCall>>,
    /// The address of the dispute game factory contract.
    pub dispute_game_factory: Address,
    /// The address of the L2OutputOracle contract.
    pub l2_output_oracle: Address,
}

impl DriverConfig {
    /// Creates a new [DriverConfig] with the given configuration.
    pub fn new(
        ws_endpoint: String,
        dispute_game_factory: Address,
        l2_output_oracle: Address,
    ) -> Self {
        // Create a new MPSC channel for sending transactions from the drivers.
        let (tx_sender, tx_receiver) = mpsc::channel(128);

        Self {
            ws_endpoint,
            tx_sender,
            tx_receiver: Mutex::new(tx_receiver),
            dispute_game_factory,
            l2_output_oracle,
        }
    }
}
