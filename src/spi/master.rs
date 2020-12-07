pub struct SpiMasterDrv {}

impl SpiMasterDrv {
    /// Send a buffer to the currently selected slave.
    pub async fn send(&mut self, tx_buf: &[u8]) -> usize {
        tx_buf.len()
    }

    /// Send to and receive from the currently selected slave.
    pub async fn xfer(&mut self, tx_buf: &[u8], rx_buf: &mut &[u8]) -> usize {
        assert!(rx_buf.len() >= tx_buf.len());

        tx_buf.len()
    }

    /// Read the current value of the miso pin.
    pub fn miso(&self) -> bool {
        true
    }
}