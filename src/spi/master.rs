pub struct SpiMasterDrv {}

impl SpiMasterDrv {
    pub async fn send(&mut self, tx_buf: &[u8]) -> usize {
        tx_buf.len()
    }

    pub async fn xfer(&mut self, tx_buf: &[u8], rx_buf: &mut &[u8]) -> usize {
        assert!(rx_buf.len() >= tx_buf.len());

        tx_buf.len()
    }

    pub fn miso(&self) -> bool {
        true
    }
}
