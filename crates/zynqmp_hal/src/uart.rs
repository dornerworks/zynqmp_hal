//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

use core::ops::Deref;

use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

use zynqmp_pac::uart::{
    Channel_sts, Control, Intrpts, Rcvr_FIFO_trigger_level, RegisterBlock, TX_RX_FIFO,
};

pub struct Device {
    ptr: *mut RegisterBlock,
}

impl Device {
    pub unsafe fn new(ptr: *mut RegisterBlock) -> Self {
        Self { ptr }
    }

    fn ptr(&self) -> *mut RegisterBlock {
        self.ptr
    }

    pub fn init(&self) {
        self.rcvr_fifo_trigger_level
            .write(Rcvr_FIFO_trigger_level::RTRIG.val(1));
        self.intrpt_en.modify(Intrpts::RTRIG::SET);
        self.intrpt_dis.modify(Intrpts::RTRIG::CLEAR);
        // TODO: Check interrupt mask is correct now
        self.reset_paths();
        self.enable_tx();
        self.enable_rx();
        self.control.modify(Control::RSTTO::SET);
        self.control.modify(Control::STTBRK::CLEAR);
        self.control.modify(Control::STPBRK::SET);
        self.rcvr_timeout.set(0);
    }

    pub fn put_char(&self, c: u8) {
        self.tx_rx_fifo.write(TX_RX_FIFO::FIFO.val(c));
        if c == b'\n' {
            self.tx_rx_fifo.write(TX_RX_FIFO::FIFO.val(b'\r'));
        }
        while self.channel_sts.matches_all(Channel_sts::TEMPTY::CLEAR) {
            core::hint::spin_loop();
        }
    }

    pub fn clear_all_interrupts(&self) {
        self.chnl_int_sts.set(0xFFFFFFFF);
    }

    pub fn get_char(&self) -> Option<u8> {
        if self.channel_sts.matches_all(Channel_sts::REMPTY::CLEAR) {
            Some(self.tx_rx_fifo.read(TX_RX_FIFO::FIFO))
        } else {
            None
        }
    }

    fn reset_paths(&self) {
        self.control.modify(Control::TXRES::SET);
        self.control.modify(Control::RXRES::SET);
    }

    fn enable_tx(&self) {
        self.control.modify(Control::TXDIS::CLEAR);
        self.control.modify(Control::TXEN::SET);
    }

    fn enable_rx(&self) {
        self.control.modify(Control::RXDIS::CLEAR);
        self.control.modify(Control::RXEN::SET);
    }
}

impl Deref for Device {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}
