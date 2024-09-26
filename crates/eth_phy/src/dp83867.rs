//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::register_bitfields;

use super::{PhyInterface, PhyReadWrite, SpecPhy};
use crate::genphy::{Bmcr, GenPhy, Mii, MmdReg, Reg, RegNum};

const DP83867_DEVADDR: u16 = 0x1f;

pub struct DP83867Conf {
    pub rx_id_delay: u16,
    pub tx_id_delay: u16,
    pub fifo_depth: u16,
    pub io_impedance: Option<u16>,
    pub rxctrl_strap_quirk: bool,
    pub port_mirroring: PortMirroring,
    pub set_clk_output: bool,
    pub clk_output_sel: Option<u16>,
    pub sgmii_ref_clk_en: bool,
    pub interface: PhyInterface,
}

pub enum PortMirroring {
    KEEP,
    ENABLE,
    DISABLE,
}

#[derive(Clone, Copy)]
pub enum Dp83867Reg {
    PhyCtrl = 0x10,
    Cfg2 = 0x14,
    Biscr = 0x16,
    Ctrl = 0x1f,
    Cfg4 = 0x0031,
    RgmiiCtl = 0x0032,
    StrapSts1 = 0x006E,
    RgmiiDCtl = 0x0086,
    IoMuxCfg = 0x0170,
    SgmiiCtl = 0x00D3,
}

// DP83867 Specific
register_bitfields![u16,
    Cfg2 [
        SPEED_OPT10M OFFSET(6) NUMBITS(1) [],
        SGMII_AUTONEGEN OFFSET(7) NUMBITS(1) [],
        SPEED_OPT_ENHANCED OFFSET(8) NUMBITS(1) [],
        SPEED_OPT OFFSET(9) NUMBITS(1) [],
        SPEED_OPT_ATTEMPT_CNT OFFSET(10) NUMBITS(2) [
            N1 = 0b00,
            N2 = 0b01,
            N4 = 0b10,
            N8 = 0b11,
        ],
        INTERRUPT_POLARITY OFFSET(13) NUMBITS(1) [
            High = 0b0,
            Low = 0b1,
        ],
    ],
    Ctrl [
        SW_RESET OFFSET(15) NUMBITS(1) [],
        SW_RESTART OFFSET(14) NUMBITS(1) [],
    ],
    PhyCr [
        DISABLE_JABBER OFFSET(0) NUMBITS(1) [],
        LINE_DRIVER_INV_EN OFFSET(1) NUMBITS(1) [],
        STANDBY OFFSET(2) NUMBITS(1) [],
        DISABLE_CLK_125 OFFSET(4) NUMBITS(1) [],
        MDI_CROSSOVER OFFSET(5) NUMBITS(2) [
            ManMDI = 0b00,
            ManMDI_X = 0b01,
            Auto = 0b10,
            Auto2 = 0b11, /* Auto and Auto2 have no functional difference */
        ],
        DEEP_POWER_DOWN OFFSET(7) NUMBITS(1) [],
        POWER_SAVE_MODE OFFSET(8) NUMBITS(2) [
            Normal = 0b00,
            Ieee = 0b01,
            Active = 0b10,
            Passive = 0b11,
        ],
        FORCE_LINK_GOOD OFFSET(10) NUMBITS(1) [],
        SGMIIEN OFFSET(11) NUMBITS(1) [],
        RX_FIFO_DEPTH OFFSET(12) NUMBITS(2) [
            Bpn3 = 0b00,
            Bpn4 = 0b01,
            Bpn6 = 0b10,
            Bpn8 = 0b11,
        ],
        TX_FIFO_DEPTH OFFSET(14) NUMBITS(2) [
            Bpn3 = 0b00,
            Bpn4 = 0b01,
            Bpn6 = 0b10,
            Bpn8 = 0b11,
        ],
    ],
    // MMD Registers
    Cfg4 [
        INT_TST_MODE_1 OFFSET(7) NUMBITS(1) [],
        PORT_MIRROR_EN OFFSET(0) NUMBITS(1) [],
    ],
    IoMuxCfg [
        CLK_O_SEL OFFSET(8) NUMBITS(5) [],
        CLK_O_DISABLE OFFSET(6) NUMBITS(1) [],
        IO_IMPEDENCE_CTRL OFFSET(0) NUMBITS(5) [],
    ],
    RgmiiCtl [
        ENABLE OFFSET(7) NUMBITS(1) [],
        RX_HALF_FULL_THR OFFSET(5) NUMBITS(2) [],
        TX_HALF_FULL_THR OFFSET(3) NUMBITS(2) [],
        TX_CLK_DELAY OFFSET(1) NUMBITS(1) [],
        RX_CLK_DELAY OFFSET(0) NUMBITS(1) [],
    ],
    RgmiiDCtl [
        TX_DELAY_CTRL OFFSET(4) NUMBITS(4) [
            Ns4_00 = 0b1111,
            Ns3_75 = 0b1110,
            Ns3_50 = 0b1101,
            Ns3_25 = 0b1100,
            Ns3_00 = 0b1011,
            Ns2_75 = 0b1010,
            Ns2_50 = 0b1001,
            Ns2_25 = 0b1000,
            Ns2_00 = 0b0111,
            Ns1_75 = 0b0110,
            Ns1_50 = 0b0101,
            Ns1_25 = 0b0100,
            Ns1_00 = 0b0011,
            Ns0_75 = 0b0010,
            Ns0_50 = 0b0001,
            Ns0_25 = 0b0000,
        ],
        RX_DELAY_CTRL OFFSET(0) NUMBITS(4) [
            Ns4_00 = 0b1111,
            Ns3_75 = 0b1110,
            Ns3_50 = 0b1101,
            Ns3_25 = 0b1100,
            Ns3_00 = 0b1011,
            Ns2_75 = 0b1010,
            Ns2_50 = 0b1001,
            Ns2_25 = 0b1000,
            Ns2_00 = 0b0111,
            Ns1_75 = 0b0110,
            Ns1_50 = 0b0101,
            Ns1_25 = 0b0100,
            Ns1_00 = 0b0011,
            Ns0_75 = 0b0010,
            Ns0_50 = 0b0001,
            Ns0_25 = 0b0000,
        ],
    ],
    SgmiiCtl [
        REF_CLK_EN OFFSET(14) NUMBITS(1) [],
    ],
    StrapSts1 [
        RESERVED OFFSET(11) NUMBITS(1) [],
    ],
];

// TODO: Make this into a Macro?
// struct RegCfg2<'a, T> {
//     reg: InMemoryRegister<u16, Cfg2::Register>,
//     phy: &'a Phy<'a, T>,
// }

// impl<'a, T> RegCfg2<'a, T>
// where
//     T: PhyReadWrite,
// {
//     const REG_NUM: RegNum = RegNum::Dp83867(Dp83867Reg::Cfg2);

//     fn new(phy: &'a Phy<T>) -> RegCfg2<'a, T> {
//         Self {
//             reg: InMemoryRegister::new(0),
//             phy,
//         }
//     }

//     fn from_read(phy: &'a Phy<T>) -> RegCfg2<'a, T> {
//         let val = phy.read(Self::REG_NUM);
//         Self {
//             reg: InMemoryRegister::new(val),
//             phy,
//         }
//     }

//     fn phy_read(&self) -> &InMemoryRegister<u16, Cfg2::Register> {
//         let val = self.phy.read(Self::REG_NUM);
//         self.reg.set(val);
//         &self.reg
//     }

//     fn reg(&self) -> &InMemoryRegister<u16, Cfg2::Register> {
//         &self.reg
//     }

//     fn phy_write(&self) {
//         let val = self.reg.get();
//         self.phy.write(Self::REG_NUM, val);
//     }
// }

// type RegBmcr<'a, T> = Reg<'a, T, Bmcr::Register>;

pub struct Phy<'a, T> {
    genphy: &'a GenPhy<'a, T>,
    conf: DP83867Conf,
}

impl<'a, T> Phy<'a, T>
where
    T: PhyReadWrite,
{
    pub fn new(genphy: &'a GenPhy<'a, T>, conf: DP83867Conf) -> Phy<'a, T> {
        Self { genphy, conf }
    }

    fn rgmii_config(&self) {
        let phy_ctrl: Reg<T, PhyCr::Register> =
            Reg::from_read(self.genphy, RegNum::Dp83867(Dp83867Reg::PhyCtrl));

        // Do not force link good
        phy_ctrl
            .reg()
            .modify(PhyCr::TX_FIFO_DEPTH.val(self.conf.fifo_depth) + PhyCr::FORCE_LINK_GOOD::CLEAR);

        // The code below checks if "port mirroring" N/A MODE4 has been
        // enabled during power on bootstrap.
        //
        // Such N/A mode enabled by mistake can put PHY IC in some
        // internal testing mode and disable RGMII transmission.
        //
        // In this particular case one needs to check STRAP_STS1
        // register's bit 11 (marked as RESERVED).

        let strap_sts1: MmdReg<T, StrapSts1::Register> = MmdReg::from_read(
            self.genphy,
            RegNum::Dp83867(Dp83867Reg::StrapSts1),
            DP83867_DEVADDR,
        );

        if strap_sts1.reg().is_set(StrapSts1::RESERVED) {
            phy_ctrl.reg().modify(PhyCr::SGMIIEN::CLEAR);
        }
        phy_ctrl.phy_write();

        let rgmii_ctl: MmdReg<T, RgmiiCtl::Register> = MmdReg::from_read(
            self.genphy,
            RegNum::Dp83867(Dp83867Reg::RgmiiCtl),
            DP83867_DEVADDR,
        );
        let val = match self.conf.interface {
            PhyInterface::RgmiiId => RgmiiCtl::TX_CLK_DELAY::SET + RgmiiCtl::RX_CLK_DELAY::SET,
            PhyInterface::RgmiiTxid => RgmiiCtl::TX_CLK_DELAY::SET + RgmiiCtl::RX_CLK_DELAY::CLEAR,
            PhyInterface::RgmiiRxid => RgmiiCtl::TX_CLK_DELAY::CLEAR + RgmiiCtl::RX_CLK_DELAY::SET,
            _ => RgmiiCtl::TX_CLK_DELAY::CLEAR + RgmiiCtl::RX_CLK_DELAY::CLEAR,
        };
        rgmii_ctl.reg().modify(val);
        rgmii_ctl.phy_write();

        let rgmiid_ctl: MmdReg<T, RgmiiDCtl::Register> = MmdReg::new(
            self.genphy,
            RegNum::Dp83867(Dp83867Reg::RgmiiDCtl),
            DP83867_DEVADDR,
        );
        rgmiid_ctl.reg().write(
            RgmiiDCtl::RX_DELAY_CTRL.val(self.conf.rx_id_delay)
                + RgmiiDCtl::TX_DELAY_CTRL.val(self.conf.tx_id_delay),
        );
        rgmiid_ctl.phy_write();
    }

    fn sgmii_config(&self) {
        if self.conf.sgmii_ref_clk_en {
            let sgmii_ctl: MmdReg<T, SgmiiCtl::Register> = MmdReg::new(
                self.genphy,
                RegNum::Dp83867(Dp83867Reg::SgmiiCtl),
                DP83867_DEVADDR,
            );
            sgmii_ctl.reg().write(SgmiiCtl::REF_CLK_EN::SET);
            sgmii_ctl.phy_write();
        }

        let bmcr: Reg<T, Bmcr::Register> = Reg::new(self.genphy, RegNum::Mii(Mii::Bmcr));
        bmcr.reg()
            .write(Bmcr::ANENABLE::SET + Bmcr::FULLDPLX::SET + Bmcr::SPEED1000::SET);
        bmcr.phy_write();

        // let cfg2 = RegCfg2::from_read(self);
        let cfg2: Reg<T, Cfg2::Register> =
            Reg::from_read(self.genphy, RegNum::Dp83867(Dp83867Reg::Cfg2));
        cfg2.reg().modify(
            Cfg2::SPEED_OPT10M::SET
                + Cfg2::SGMII_AUTONEGEN::SET
                + Cfg2::SPEED_OPT_ENHANCED::SET
                + Cfg2::SPEED_OPT_ATTEMPT_CNT::N4
                + Cfg2::INTERRUPT_POLARITY::Low,
        );
        cfg2.phy_write();

        let rgmii_ctl: MmdReg<T, RgmiiCtl::Register> = MmdReg::new(
            self.genphy,
            RegNum::Dp83867(Dp83867Reg::RgmiiCtl),
            DP83867_DEVADDR,
        );
        rgmii_ctl.reg().set(0);
        rgmii_ctl.phy_write();

        let phy_ctrl: Reg<T, PhyCr::Register> =
            Reg::new(self.genphy, RegNum::Dp83867(Dp83867Reg::PhyCtrl));
        phy_ctrl.reg().write(
            PhyCr::SGMIIEN::SET
                + PhyCr::MDI_CROSSOVER::Auto
                + PhyCr::RX_FIFO_DEPTH.val(self.conf.fifo_depth)
                + PhyCr::TX_FIFO_DEPTH.val(self.conf.fifo_depth),
        );
        phy_ctrl.phy_write();

        self.genphy.write(RegNum::Dp83867(Dp83867Reg::Biscr), 0x0);
    }

    fn config_port_mirroring(&self) {
        let val = match self.conf.port_mirroring {
            PortMirroring::ENABLE => Cfg4::PORT_MIRROR_EN::SET,
            PortMirroring::DISABLE => Cfg4::PORT_MIRROR_EN::CLEAR,
            PortMirroring::KEEP => return,
        };
        let cfg4: MmdReg<T, Cfg4::Register> = MmdReg::from_read(
            self.genphy,
            RegNum::Dp83867(Dp83867Reg::Cfg4),
            DP83867_DEVADDR,
        );
        cfg4.reg().modify(val);
        cfg4.phy_write();
    }
}

impl<'a, T> SpecPhy for Phy<'a, T>
where
    T: PhyReadWrite,
{
    fn config(&self) {
        // Restart the PHY
        let ctrl: Reg<T, Ctrl::Register> =
            Reg::from_read(self.genphy, RegNum::Dp83867(Dp83867Reg::Ctrl));
        ctrl.reg().modify(Ctrl::SW_RESTART::SET);
        ctrl.phy_write();

        // Mode 1 or 2 workaround
        if self.conf.rxctrl_strap_quirk {
            let cfg4: MmdReg<T, Cfg4::Register> = MmdReg::from_read(
                self.genphy,
                RegNum::Dp83867(Dp83867Reg::Cfg4),
                DP83867_DEVADDR,
            );
            cfg4.reg().modify(Cfg4::INT_TST_MODE_1::CLEAR);
            cfg4.phy_write();
        }

        if self.conf.interface.is_rgmii() {
            self.rgmii_config();
        } else if self.conf.interface == PhyInterface::Sgmii {
            self.sgmii_config();
        }

        if let Some(impedance) = self.conf.io_impedance {
            let io_mux: MmdReg<T, IoMuxCfg::Register> = MmdReg::from_read(
                self.genphy,
                RegNum::Dp83867(Dp83867Reg::IoMuxCfg),
                DP83867_DEVADDR,
            );
            io_mux
                .reg()
                .modify(IoMuxCfg::IO_IMPEDENCE_CTRL.val(impedance));
            io_mux.phy_write();
        }

        self.config_port_mirroring();

        /* Clock output selection if muxing property is set */
        if self.conf.set_clk_output {
            let io_mux: MmdReg<T, IoMuxCfg::Register> = MmdReg::from_read(
                self.genphy,
                RegNum::Dp83867(Dp83867Reg::IoMuxCfg),
                DP83867_DEVADDR,
            );

            let val = match self.conf.clk_output_sel {
                None => IoMuxCfg::CLK_O_DISABLE::SET,
                Some(clk_output_sel) => {
                    IoMuxCfg::CLK_O_DISABLE::CLEAR + IoMuxCfg::CLK_O_SEL.val(clk_output_sel)
                }
            };
            io_mux.reg().modify(val);
            io_mux.phy_write();
        }

        // TODO: Does this always need to be called after a config?
        //       If so, then it should be done by the genphy
        self.genphy.config_aneg().unwrap();
    }
}
