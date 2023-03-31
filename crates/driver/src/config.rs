//! The `config` module contains the [DriverConfig].

use ethers::types::{Address, Bytes};
use tokio::sync::{mpsc, Mutex};

/// The [DriverConfig] struct contains the configuration for the [Driver].
#[derive(Debug)]
pub struct DriverConfig {
    /// The URL of the RPC endpoint used to index and send transactions.
    pub ws_endpoint: String,
    /// The sending handle of the MPSC channel used to send transactions.
    pub tx_sender: mpsc::Sender<Bytes>,
    /// The receiving handle of the MPSC channel used to send transactions.
    pub tx_receiver: Mutex<mpsc::Receiver<Bytes>>,
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