use driver_common::{BaseDriverOps, DeviceType, DevResult};
#[cfg(feature = "net")]
use driver_net::NetDriverOps;
#[cfg(feature = "net")]
use driver_net::{EthernetAddress, NetBuf, NetBufBox, NetBufPool, NetBufPtr};

pub struct NetFilter<T:BaseDriverOps> {
    pub inner: T,
}

impl <T: BaseDriverOps> BaseDriverOps for NetFilter<T> {
    fn device_name(&self) -> &str {
        self.inner.device_name()
    }
    fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }
}

#[cfg(feature = "net")]
impl <T: BaseDriverOps + NetDriverOps> NetDriverOps for NetFilter<T> {
    fn mac_address(&self) -> EthernetAddress { self.inner.mac_address() }
    fn can_transmit(&self) -> bool { self.inner.can_transmit() }
    fn can_receive(&self) -> bool { self.inner.can_receive() }
    fn rx_queue_size(&self) -> usize { self.inner.rx_queue_size() }
    fn tx_queue_size(&self) -> usize { self.inner.tx_queue_size() }
    fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult { self.inner.recycle_rx_buffer(rx_buf) }
    fn recycle_tx_buffers(&mut self) -> DevResult { self.inner.recycle_tx_buffers() }
    fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        warn!("Filter: transmit len[{}]", tx_buf.packet_len());
        self.inner.transmit(tx_buf)
    }
    fn receive(&mut self) -> DevResult<NetBufPtr> {
        let devResult = self.inner.receive();
        if let Ok( ref r ) = self.inner.receive() { // here, ref is important to not 'move' ownership to 'r'
            warn!("Filter: receive len[{:?}]", r.packet_len());
        }
        devResult
    }
    fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> { self.inner.alloc_tx_buffer(size) }
}