#![allow(unused_imports)]
#![allow(dead_code)]

use crate::virtio::VirtIoHalImpl;
use driver_virtio::MmioTransport;
use super::prelude::*;
use cfg_if::cfg_if;

pub struct NetFilter<T> {
    pub inner: T,
}

cfg_if! {
    if #[cfg(feature = "net")] {
        impl driver_common::BaseDriverOps for NetFilter<driver_virtio::VirtIoNetDev<VirtIoHalImpl, MmioTransport, 64>> 
        {
            fn device_name(&self) -> &str {
                self.inner.device_name()
            }

            fn device_type(&self) -> driver_common::DeviceType {
                self.inner.device_type()
            }

        }

        impl driver_net::NetDriverOps for NetFilter<driver_virtio::VirtIoNetDev<VirtIoHalImpl, MmioTransport, 64>> 
        {
            fn mac_address(&self) -> driver_net::EthernetAddress {
                self.inner.mac_address()
            }

            fn can_transmit(&self) -> bool {
                self.inner.can_transmit()
            }

            fn can_receive(&self) -> bool {
                self.inner.can_receive()
            }

            fn rx_queue_size(&self) -> usize {
                self.inner.rx_queue_size()
            }

            fn tx_queue_size(&self) -> usize {
                self.inner.tx_queue_size()
            }

            fn recycle_rx_buffer(&mut self, rx_buf: driver_net::NetBufPtr) -> driver_net::DevResult {
                self.inner.recycle_rx_buffer(rx_buf)
            }

            fn recycle_tx_buffers(&mut self) -> driver_net::DevResult {
                self.inner.recycle_tx_buffers()
            }

            fn transmit(&mut self, tx_buf: driver_net::NetBufPtr) -> driver_net::DevResult {
                log::warn!("Filter: transmit len [{}]", tx_buf.packet_len());
                self.inner.transmit(tx_buf)
            }

            fn receive(&mut self) -> driver_net::DevResult<driver_net::NetBufPtr> {
                let recv = self.inner.receive()?;
                log::warn!("Filter: receive len [{}]", recv.packet_len());
                Ok(recv)
            }

            fn alloc_tx_buffer(&mut self, size: usize) -> driver_net::DevResult<driver_net::NetBufPtr> {
                self.inner.alloc_tx_buffer(size)
            }

        }

    }
}

