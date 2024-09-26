//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

use tock_registers::registers::{ReadOnly, ReadWrite};
use tock_registers::{register_bitfields, register_structs};

register_structs! {
    pub RegisterBlock {
        (0x00 => pub control: ReadWrite<u32, Control::Register>),
        (0x04 => pub mode: ReadWrite<u32, Mode::Register>),
        (0x08 => pub intrpt_en: ReadWrite<u32, Intrpts::Register>),
        (0x0C => pub intrpt_dis: ReadWrite<u32, Intrpts::Register>),
        (0x10 => pub intrpt_mask: ReadWrite<u32, Intrpts::Register>),
        (0x14 => pub chnl_int_sts: ReadWrite<u32, Intrpts::Register>),
        (0x18 => pub baud_rate_gen: ReadWrite<u32, Baud_rate_gen::Register>),
        (0x1C => pub rcvr_timeout: ReadWrite<u32, Rcvr_timeout::Register>),
        (0x20 => pub rcvr_fifo_trigger_level: ReadWrite<u32, Rcvr_FIFO_trigger_level::Register>),
        (0x24 => pub modem_ctrl: ReadWrite<u32, Modem_ctrl::Register>),
        (0x28 => pub modem_sts: ReadWrite<u32, Modem_sts::Register>),
        (0x2C => pub channel_sts: ReadOnly<u32, Channel_sts::Register>),
        (0x30 => pub tx_rx_fifo: ReadWrite<u8, TX_RX_FIFO::Register>),
        (0x31 => _reserved0),
        (0x34 => pub baud_rate_divider: ReadWrite<u32, Baud_rate_divider::Register>),
        (0x38 => pub flow_delay: ReadWrite<u32, Flow_delay::Register>),
        (0x3C => _reserved1),
        (0x44 => pub tx_fifo_trigger_level: ReadWrite<u32, Tx_FIFO_trigger_level::Register>),
        (0x48 => pub rx_fifo_byte_status: ReadWrite<u32, Rx_FIFO_byte_status::Register>),
        (0x4C => @END),
    }
}

register_bitfields! {
    u8,
    pub TX_RX_FIFO [
        FIFO OFFSET(0) NUMBITS(8) [],
    ],
}

register_bitfields! {
    u32,
    pub Control [
        STPBRK OFFSET(8) NUMBITS(1) [],
        STTBRK OFFSET(7) NUMBITS(1) [],
        RSTTO OFFSET(6) NUMBITS(1) [],
        TXDIS OFFSET(5) NUMBITS(1) [],
        TXEN OFFSET(4) NUMBITS(1) [],
        RXDIS OFFSET(3) NUMBITS(1) [],
        RXEN OFFSET(2) NUMBITS(1) [],
        TXRES OFFSET(1) NUMBITS(1) [],
        RXRES OFFSET(0) NUMBITS(1) [],
    ],
    pub Mode [
        WSIZE OFFSET(12) NUMBITS(2) [],
        CHMODE OFFSET(8) NUMBITS(2) [],
        NBSTOP OFFSET(6) NUMBITS(2) [],
        PAR OFFSET(3) NUMBITS(3) [],
        CHRL OFFSET(1) NUMBITS(2) [],
        CLKS OFFSET(0) NUMBITS(1) [],
    ],
    pub Intrpts [
        RBRK OFFSET(13) NUMBITS(1) [],
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        DMSI OFFSET(9) NUMBITS(1) [],
        TIMEOUT OFFSET(8) NUMBITS(1) [],
        PARE OFFSET(7) NUMBITS(1) [],
        FRAME OFFSET(6) NUMBITS(1) [],
        ROVR OFFSET(5) NUMBITS(1) [],
        TFUL OFFSET(4) NUMBITS(1) [],
        TEMPTY OFFSET(3) NUMBITS(1) [],
        RFUL OFFSET(2) NUMBITS(1) [],
        REMPTY OFFSET(1) NUMBITS(1) [],
        RTRIG OFFSET(0) NUMBITS(1) [],
    ],
    pub Baud_rate_gen [
        CD OFFSET(0) NUMBITS(16) [],
    ],
    pub Rcvr_timeout [
        RTO OFFSET(0) NUMBITS(8) [],
    ],
    pub Rcvr_FIFO_trigger_level [
        RTRIG OFFSET(0) NUMBITS(6) [],
    ],
    pub Modem_ctrl [
        FCM OFFSET(5) NUMBITS(1) [],
        RTS OFFSET(1) NUMBITS(1) [],
        DTR OFFSET(0) NUMBITS(1) [],
    ],
    pub Modem_sts [
        FCMS OFFSET(8) NUMBITS(1) [],
        DCD OFFSET(7) NUMBITS(1) [],
        RI OFFSET(6) NUMBITS(1) [],
        DSR OFFSET(5) NUMBITS(1) [],
        CTS OFFSET(4) NUMBITS(1) [],
        DDCD OFFSET(3) NUMBITS(1) [],
        TERI OFFSET(2) NUMBITS(1) [],
        DDSR OFFSET(1) NUMBITS(1) [],
        DCTS OFFSET(0) NUMBITS(1) [],
    ],
    pub Channel_sts [
        TNFUL OFFSET(14) NUMBITS(1) [],
        TTRIG OFFSET(13) NUMBITS(1) [],
        FDELT OFFSET(12) NUMBITS(1) [],
        TACTIVE OFFSET(11) NUMBITS(1) [],
        RACTIVE OFFSET(10) NUMBITS(1) [],
        TFUL OFFSET(4) NUMBITS(1) [],
        TEMPTY OFFSET(3) NUMBITS(1) [],
        RFUL OFFSET(2) NUMBITS(1) [],
        REMPTY OFFSET(1) NUMBITS(1) [],
        RTRIG OFFSET(0) NUMBITS(1) [],
    ],
    pub Baud_rate_divider [
        BDIV OFFSET(0) NUMBITS(8) [],
    ],
    pub Flow_delay [
        FDEL OFFSET(0) NUMBITS(6) [],
    ],
    pub Tx_FIFO_trigger_level [
        TTRIG OFFSET(0) NUMBITS(6) [],
    ],
    pub Rx_FIFO_byte_status [
        BYTE3_BREAK OFFSET(11) NUMBITS(1) [],
        BYTE3_FRM_ERR OFFSET(10) NUMBITS(1) [],
        BYTE3_PAR_ERR OFFSET(9) NUMBITS(1) [],
        BYTE2_BREAK OFFSET(8) NUMBITS(1) [],
        BYTE2_FRM_ERR OFFSET(7) NUMBITS(1) [],
        BYTE2_PAR_ERR OFFSET(6) NUMBITS(1) [],
        BYTE1_BREAK OFFSET(5) NUMBITS(1) [],
        BYTE1_FRM_ERR OFFSET(4) NUMBITS(1) [],
        BYTE1_PAR_ERR OFFSET(3) NUMBITS(1) [],
        BYTE0_BREAK OFFSET(2) NUMBITS(1) [],
        BYTE0_FRM_ERR OFFSET(1) NUMBITS(1) [],
        BYTE0_PAR_ERR OFFSET(0) NUMBITS(1) [],
    ],
}
