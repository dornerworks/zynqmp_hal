//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

use core::ops::Deref;
use tock_registers::{
    fields::FieldValue,
    interfaces::{ReadWriteable, Readable, Writeable},
};

use core::marker::PhantomData;

use zynqmp_pac::gem::*;

use eth_phy::{Duplex, PhyReadWrite, Speed};

pub struct Device<S> {
    ptr: *mut RegisterBlock,
    phantom: PhantomData<S>,
}

pub struct MacAddress([u8; 6]);

pub struct Reset;
pub struct PhyReady;
pub struct Config;
pub struct Running;

impl Device<Reset> {
    pub fn new(ptr: *mut RegisterBlock) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    pub fn init(&self) -> Device<PhyReady> {
        self.reset_dev();
        self.set_defaults();
        // TODO: I/O Configuration. Clocks and MIO. Can defer if we assume bootloader has done this.
        Device {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }

    fn reset_dev(&self) {
        // Disable interrupts
        self.int_disable.set(0xFFFF_FFFF);

        // Disable rx and tx
        self.network_control.set(0);
        self.network_control
            .write(network_control::CLEAR_ALL_STATS_REGS::SET);
        self.transmit_status.set(0xFFFF_FFFF);
        self.receive_status.set(0xFFFF_FFFF);
        self.phy_management.set(0);
        self.transmit_q1_ptr.set(0);
        self.receive_q1_ptr.set(0);

        self.tx_bd_control.set(0);
        self.rx_bd_control.set(0);

        // Clear hash registers for MAC address
        self.hash_bottom.set(0);
        self.hash_top.set(0);

        // TODO: Clear stats registers? 0x100-0x1B4
    }

    fn set_defaults(&self) {
        let net_cfg = network_config::NO_BROADCAST::CLEAR
            + network_config::DATA_BUS_WIDTH.val(1)
            + network_config::RECEIVE_CHECKSUM_OFFLOAD_ENABLE::SET
            + network_config::PAUSE_ENABLE::CLEAR;

        // TODO: FCS_REMOVE?
        // TODO: multicast_hash_en?

        // Modify here and no clear in the reset function to avoid figuring out MDC clock dividor
        self.network_config.modify(net_cfg);

        // TODO: Enable promiscuous mode here? Leave up to user?

        // 1600 bytes for RX buffer
        // Full size for RX and TX packet buffer sizes
        // INCR16 AXI Burst size for higher performance
        let dma_cfg = dma_config::RX_BUF_SIZE.val(25u32)
            + dma_config::TX_BD_EXTENDED_MODE_EN::CLEAR
            + dma_config::RX_BD_EXTENDED_MODE_EN::CLEAR
            + dma_config::DMA_ADDR_BUS_WIDTH_1::CLEAR
            + dma_config::RX_PBUF_SIZE::PBUF_32KB
            + dma_config::TX_PBUF_SIZE::SET
            // + dma_config::TX_PBUF_TCP_EN::SET
            + dma_config::ENDIAN_SWAP_PACKET::CLEAR
            + dma_config::AMBA_BURST_LENGTH.val(4u32);

        self.dma_config.write(dma_cfg);

        self.network_control
            .modify(network_control::MAN_PORT_EN::SET);

        // TODO: Disable second priority queue? Set *_q1_ptr to addresses of a single descriptor which set the wrap bit

        self.int_enable.write(
            int_enable::ENABLE_RECEIVE_COMPLETE_INTERRUPT::SET
                + int_enable::ENABLE_TRANSMIT_COMPLETE_INTERRUPT::SET,
        );
    }
}

impl MacAddress {
    pub fn new(mac: [u8; 6]) -> Self {
        MacAddress(mac)
    }

    pub fn get_bottom(&self) -> u32 {
        let mut data = [0u8; 4];
        data.copy_from_slice(&self.0[0..4]);
        u32::from_le_bytes(data)
        // data.copy_from_slice(&self.0[2..]);
        // u32::from_be_bytes(data)
    }

    pub fn get_top(&self) -> u16 {
        let mut data = [0u8; 2];
        data.copy_from_slice(&self.0[4..]);
        u16::from_le_bytes(data)
        // data.copy_from_slice(&self.0[0..2]);
        // u16::from_be_bytes(data)
    }

    pub fn inner(&self) -> [u8; 6] {
        self.0
    }
}

impl From<(u32, u16)> for MacAddress {
    fn from(mac: (u32, u16)) -> MacAddress {
        let (bottom, top) = mac;
        let mut data = [0u8; 6];
        data[0..4].copy_from_slice(&bottom.to_le_bytes());
        data[4..6].copy_from_slice(&top.to_le_bytes());
        // data[0..2].copy_from_slice(&top.to_be_bytes());
        // data[2..].copy_from_slice(&bottom.to_be_bytes());
        MacAddress(data)
    }
}

#[derive(Debug)]
pub enum RecvStatus {
    FrameReceived,
    RespNotOk,
    FifoOverflow,
    UnavailableBuffer,
    Unknown,
}

impl PhyReadWrite for Device<PhyReady> {
    fn phy_write(&self, phy_addr: u32, regnum: u32, data: u16) {
        self.phy_setup_op(phy_addr, regnum, phy_management::OPERATION::Write, data);
    }

    fn phy_read(&self, phy_addr: u32, regnum: u32) -> u16 {
        self.phy_setup_op(phy_addr, regnum, phy_management::OPERATION::Read, 0u16);

        self.phy_management
            .read(phy_management::PHY_WRITE_READ_DATA) as u16
    }
}

impl Device<PhyReady> {
    fn phy_setup_op(
        &self,
        phy_addr: u32,
        regnum: u32,
        op: FieldValue<u32, phy_management::Register>,
        data: u16,
    ) {
        // TODO: Need a timeout??
        // TODO: Write as an inline always function?
        while self
            .network_status
            .matches_all(network_status::MAN_DONE::CLEAR)
        {
            core::hint::spin_loop();
        }

        let mgt_cmd = op
            + phy_management::WRITE1::SET      // Clause 22 Frame
            + phy_management::WRITE10::Always  // Always set
            + phy_management::PHY_ADDRESS.val(phy_addr)
            + phy_management::REGISTER_ADDRESS.val(regnum)
            + phy_management::PHY_WRITE_READ_DATA.val(data.into());

        self.phy_management.write(mgt_cmd);

        while self
            .network_status
            .matches_all(network_status::MAN_DONE::CLEAR)
        {
            core::hint::spin_loop();
        }
    }

    pub fn phy_complete(&self) -> Device<Config> {
        Device {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<S> Device<S> {
    fn ptr(&self) -> *mut RegisterBlock {
        self.ptr
    }
}

impl Device<Config> {
    pub fn set_speed(&self, speed: Speed) {
        match speed {
            Speed::S1000 => self
                .network_config
                .modify(network_config::GIGABIT_MODE_ENABLE::SET),
            Speed::S100 => self
                .network_config
                .modify(network_config::GIGABIT_MODE_ENABLE::CLEAR + network_config::SPEED::SET),
            Speed::S10 => self
                .network_config
                .modify(network_config::GIGABIT_MODE_ENABLE::CLEAR + network_config::SPEED::CLEAR),
        }
    }

    pub fn set_duplex(&self, duplex: Duplex) {
        match duplex {
            Duplex::Half => self
                .network_config
                .modify(network_config::FULL_DUPLEX::CLEAR),
            Duplex::Full => self.network_config.modify(network_config::FULL_DUPLEX::SET),
        }
    }

    pub fn enable_promiscuous_mode(&self) {
        self.network_config
            .modify(network_config::COPY_ALL_FRAMES::SET);
    }

    pub fn disable_promiscuous_mode(&self) {
        self.network_config
            .modify(network_config::COPY_ALL_FRAMES::CLEAR);
    }

    pub fn set_mac_address(&self, mac: MacAddress) {
        self.spec_add1_bottom
            .write(spec_add1_bottom::ADDRESS.val(mac.get_bottom()));
        self.spec_add1_top
            .write(spec_add1_top::ADDRESS.val(mac.get_top().into()));
    }

    pub fn split_mac_address(&self) -> (u32, u32) {
        let bottom = self.spec_add1_bottom.read(spec_add1_bottom::ADDRESS);
        let top = self.spec_add1_top.read(spec_add1_top::ADDRESS);
        (bottom, top)
    }

    pub fn set_tx_desc(&self, desc: u32) {
        self.transmit_q_ptr
            .write(transmit_q_ptr::DMA_TX_Q_PTR.val(desc));
        self.upper_tx_q_base_addr
            .write(upper_tx_q_base_addr::UPPER_TX_Q_BASE_ADDR.val(0));
    }

    pub fn set_tx_q1_desc(&self, desc: u32) {
        self.transmit_q1_ptr
            .write(transmit_q1_ptr::DMA_TX_Q_PTR.val(desc));
    }

    pub fn set_rx_desc(&self, desc: u32) {
        self.receive_q_ptr
            .write(receive_q_ptr::DMA_RX_Q_PTR.val(desc));
        self.upper_rx_q_base_addr
            .write(upper_rx_q_base_addr::UPPER_RX_Q_BASE_ADDR.val(0));
    }

    fn enable_tx(&self) {
        self.network_control
            .modify(network_control::ENABLE_TRANSMIT::SET);
    }

    fn enable_rx(&self) {
        self.network_control
            .modify(network_control::ENABLE_RECEIVE::SET);
    }

    pub fn run(&self) -> Device<Running> {
        self.enable_tx();
        self.enable_rx();
        Device {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl Device<Running> {
    pub fn clear_all_interrupts(&self) {
        self.int_status.set(0xFFFFFFFF);
    }

    pub fn tx_is_complete(&self) -> bool {
        self.int_status.is_set(int_status::TRANSMIT_COMPLETE)
    }

    pub fn rx_is_complete(&self) -> bool {
        self.int_status.is_set(int_status::RECEIVE_COMPLETE)
    }

    pub fn int_status(&self) -> u32 {
        self.int_status.get()
    }

    pub fn get_rx_desc(&self) -> u32 {
        self.receive_q_ptr.get()
    }

    pub fn get_tx_desc(&self) -> u32 {
        self.transmit_q_ptr.get()
    }

    pub fn get_transmit_status(&self) -> u32 {
        let val = self.transmit_status.get();
        self.transmit_status.set(val);
        val
    }

    pub fn get_receive_status(&self) -> RecvStatus {
        let val = self.receive_status.extract();
        let bits = val.get();

        // Clear status
        self.receive_status.set(bits);

        if val.is_set(receive_status::FRAME_RECEIVED) {
            RecvStatus::FrameReceived
        } else if val.is_set(receive_status::RESP_NOT_OK) {
            RecvStatus::RespNotOk
        } else if val.is_set(receive_status::RECEIVE_OVERRUN) {
            RecvStatus::FifoOverflow
        } else if val.is_set(receive_status::BUFFER_NOT_AVAILABLE) {
            RecvStatus::UnavailableBuffer
        } else {
            RecvStatus::Unknown
        }
    }

    pub fn transmit(&self) {
        while self
            .transmit_status
            .matches_all(transmit_status::TRANSMIT_GO::SET)
        {
            core::hint::spin_loop();
        }

        self.network_control
            .modify(network_control::TX_START_PCLK::SET);
    }

    pub fn stop(&self) -> Device<Config> {
        self.disable_tx();
        self.disable_rx();
        Device {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }

    fn disable_tx(&self) {
        self.network_control
            .modify(network_control::ENABLE_TRANSMIT::CLEAR);
    }

    fn disable_rx(&self) {
        self.network_control
            .modify(network_control::ENABLE_RECEIVE::CLEAR);
    }

    pub fn mac_address(&self) -> MacAddress {
        let bottom = self.spec_add1_bottom.read(spec_add1_bottom::ADDRESS);
        let top = self.spec_add1_top.read(spec_add1_top::ADDRESS) as u16;
        MacAddress::from((bottom, top))
    }
}

impl<S> Deref for Device<S> {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}
