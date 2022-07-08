use core::{
    cmp::min,
    iter::repeat,
    time::Duration,
};
use rusb::{Device, DeviceHandle, EndpointDescriptor, Interface, InterfaceDescriptor, UsbContext};

pub trait Transport {
    type Error: std::error::Error;
    fn write(&self, msg: &[u8], timeout: Duration) -> Result<usize, Self::Error>;
    fn read(&self, buf: &mut Vec<u8>, timeout: Duration) -> Result<(), Self::Error>;
}

pub struct UsbTransport<T: UsbContext> {
    handle: DeviceHandle<T>,
    endpoint_index: u8,
    packet_size: usize,
}

impl<T: UsbContext> UsbTransport<T> {
    pub fn new(device: Device<T>) -> Result<Self, rusb::Error> {
        let config_descriptor = device.active_config_descriptor().unwrap();
        let mut handle = device.open()?;

        let interfaces = config_descriptor.interfaces().collect::<Box<[Interface]>>();
        assert!(interfaces.len() > 1);
        let interface = &interfaces[0];
        handle.claim_interface(interface.number()).unwrap();

        let interface_descriptors = interface
            .descriptors()
            .collect::<Box<[InterfaceDescriptor]>>();
        assert_eq!(interface_descriptors.len(), 1);
        let interface_descriptor = &interface_descriptors[0];

        let endpoint_descriptors = interface_descriptor
            .endpoint_descriptors()
            .collect::<Box<[EndpointDescriptor]>>();
        // assert_eq!(endpoint_descriptors.len(), 1);
        let endpoint_descriptor = &endpoint_descriptors[0];

        Ok(Self {
            handle,
            endpoint_index: 1,
            packet_size: endpoint_descriptor.max_packet_size().into(),
        })
    }
    fn read_packet(
        &self,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> Result<(), rusb::Error> {
        let mut packet = vec![0u8; self.packet_size];
        let len = self
            .handle
            .read_interrupt(0x80 | self.endpoint_index, &mut packet, timeout)?;
        assert_eq!(len, packet.len());
        if !(len >= 1 && packet[0] == '?' as u8) {
            return Err(rusb::Error::Other);
        }
        buf.extend_from_slice(&packet[1..]);
        Ok(())
    }
}

impl<T: UsbContext> Transport for UsbTransport<T> {
    type Error = rusb::Error;
    fn write(&self, msg: &[u8], timeout: Duration) -> Result<usize, Self::Error> {
        let mut packet = Vec::<u8>::with_capacity(self.packet_size);
        for chunk in msg.chunks(self.packet_size - 1) {
            packet.clear();
            packet.push('?' as u8);
            packet.extend_from_slice(chunk);
            packet.extend(repeat(0).take(self.packet_size - packet.len()));
            assert_eq!(packet.len(), self.packet_size);

            let written_len = self
                .handle
                .write_interrupt(self.endpoint_index, &packet, timeout)?;
            assert_eq!(written_len, packet.len());
        }
        Ok(msg.len())
    }
    fn read(
        &self,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> Result<(), Self::Error> {
        let mut packet = Vec::<u8>::with_capacity(self.packet_size);
        self.read_packet(&mut packet, timeout)?;

        if !(packet.len() >= 8 && packet[0] == '#' as u8 && packet[1] == '#' as u8) {
            return Err(rusb::Error::Other);
        }
        let msg_len: usize = u32::from_be_bytes(packet[4..8].try_into().unwrap())
            .try_into()
            .unwrap();

        let mut len_remaining = 8 + msg_len;
        loop {
            buf.extend_from_slice(&packet[..min(len_remaining, packet.len())]);
            len_remaining = len_remaining.saturating_sub(packet.len());

            if len_remaining == 0 { break }

            packet.clear();
            self.read_packet(&mut packet, timeout)?;
        }

        Ok(())
    }
}
