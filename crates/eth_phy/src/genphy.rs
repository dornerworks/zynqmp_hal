//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

use crate::dp83867::Dp83867Reg;
use core::ops::BitAndAssign;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::registers::InMemoryRegister;
use tock_registers::{register_bitfields, RegisterLongName};

use super::{Duplex, PhyReadWrite, Speed, Supported};

const PHYREG_MASK: u16 = 0x1808;

/* MMD Access Control register fields */
const MII_MMD_CTRL_NOINCR: u16 = 0x4000; /* no post increment */

#[derive(Clone, Copy)]
pub(crate) enum RegNum {
    Phy(PhyReg),
    Mii(Mii),
    Dp83867(Dp83867Reg),
}

impl From<RegNum> for u32 {
    fn from(val: RegNum) -> Self {
        match val {
            RegNum::Phy(reg) => reg as u32,
            RegNum::Mii(reg) => reg as u32,
            RegNum::Dp83867(reg) => reg as u32,
        }
    }
}

impl From<RegNum> for u16 {
    fn from(val: RegNum) -> Self {
        match val {
            RegNum::Phy(reg) => reg as u16,
            RegNum::Mii(reg) => reg as u16,
            RegNum::Dp83867(reg) => reg as u16,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum PhyReg {
    DetectReg = 0x01,
}

/* Generic MII registers. */
#[derive(Clone, Copy)]
pub(crate) enum Mii {
    Bmcr = 0x00,
    Bmsr = 0x01,
    // PhysId1 = 0x02,     /* PHYS ID 1                   */
    // PhysId2 = 0x03,     /* PHYS ID 2                   */
    Advertise = 0x04, /* Advertisement control reg   */
    Lpa = 0x05,       /* Link partner ability reg    */
    // Expansion = 0x06,   /* Expansion register          */
    Ctrl1000 = 0x09, /* 1000BASE-T control          */
    Stat1000 = 0x0a, /* 1000BASE-T status           */
    MmdCtrl = 0x0d,  /* MMD Access Control Register */
    MmdData = 0x0e,  /* MMD Access Data Register */
    EStatus = 0x0f,  /* Extended Status             */
                     // DCounter = 0x12,    /* Disconnect counter          */
                     // FcsCounter = 0x13,  /* False carrier counter       */
                     // NwayTest = 0x14,    /* N-way auto-neg test reg     */
                     // RErrCounter = 0x15, /* Receive error counter       */
                     // SRevision = 0x16,   /* Silicon revision            */
                     // Resv1 = 0x17,       /* Reserved...                 */
                     // LbrError = 0x18,    /* Lpback, rx, bypass error    */
                     // PhyAddr = 0x19,     /* PHY address                 */
                     // Resv2 = 0x1a,       /* Reserved...                 */
                     // TpiStatus = 0x1b,   /* TPI status for 10mbps       */
                     // NConfig = 0x1c,     /* Network interface config    */
}

// General
register_bitfields![u16,
    pub Advertise [
        CSMA OFFSET(0) NUMBITS(1) [], /* Only selector supported     */
        HALF10 OFFSET(5) NUMBITS(1) [], /* Try for 10mbps half-duplex  */
        FULL10 OFFSET(6) NUMBITS(1) [], /* Try for 10mbps full-duplex  */
        HALF100 OFFSET(7) NUMBITS(1) [], /* Try for 100mbps half-duplex */
        FULL100 OFFSET(8) NUMBITS(1) [], /* Try for 100mbps full-duplex */
        BASE100_4 OFFSET(9) NUMBITS(1) [], /* Try for 100mbps 4k packets  */
        PAUSE_CAP OFFSET(10) NUMBITS(1) [], /* Try for pause               */
        PAUSE_ASYM OFFSET(11) NUMBITS(1) [], /* Try for asymetric pause     */
        RFAULT OFFSET(13) NUMBITS(1) [], /* Say we can detect faults    */
        LPACK OFFSET(14) NUMBITS(1) [], /* Ack link partners response  */
        NPAGE OFFSET(15) NUMBITS(1) [], /* Next page bit               */
    ],
    pub Base1000TCtrl [
        HALF OFFSET(8) NUMBITS(1) [],
        FULL OFFSET(9) NUMBITS(1) [],
    ],
    pub Bmcr [
        SPEED1000 OFFSET(6) NUMBITS(1) [],
        CTST OFFSET(7) NUMBITS(1) [],
        FULLDPLX OFFSET(8) NUMBITS(1) [],
        ANRESTART OFFSET(9) NUMBITS(1) [],
        ISOLATE OFFSET(10) NUMBITS(1) [],
        PDOWN OFFSET(11) NUMBITS(1) [],
        ANENABLE OFFSET(12) NUMBITS(1) [],
        SPEED100 OFFSET(13) NUMBITS(1) [],
        LOOPBACK OFFSET(14) NUMBITS(1) [],
        RESET OFFSET(15) NUMBITS(1) [],
    ],
    pub Bmsr [
        ERCAP OFFSET(0) NUMBITS(1) [], /* Ext-reg capability          */
        JCD OFFSET(1) NUMBITS(1) [], /* Jabber detected             */
        LSTATUS OFFSET(2) NUMBITS(1) [], /* Link status                 */
        ANEGCAPABLE OFFSET(3) NUMBITS(1) [], /* Able to do auto-negotiation */
        RFAULT OFFSET(4) NUMBITS(1) [], /* Remote fault detected       */
        ANEGCOMPLETE OFFSET(5) NUMBITS(1) [], /* Auto-negotiation complete   */
        ESTATEN OFFSET(8) NUMBITS(1) [], /* Extended Status in R15      */

    ],
    pub Estatus [
        THALF1000 OFFSET(12) NUMBITS(1) [],
        TFULL1000 OFFSET(13) NUMBITS(1) [],
        XHALF1000 OFFSET(14) NUMBITS(1) [],
        XFULL1000 OFFSET(15) NUMBITS(1) [],
    ],
    pub Stat1000 [
        MASTER_SLAVE_CFG_FAULT OFFSET(15) NUMBITS(1) [],
        MASTER_SLAVE_CFG_RES OFFSET(14) NUMBITS(1) [],
        LOCAL_RECV_STS OFFSET(13) NUMBITS(1) [],
        REMOTE_RECV_STS OFFSET(12) NUMBITS(1) [],
        BTSR_1000FD OFFSET(11) NUMBITS(1) [],
        BTSR_1000HD OFFSET(10) NUMBITS(1) [],
        IDLE_ERR_COUNTER OFFSET(0) NUMBITS(8) [],
    ],
];

pub(crate) struct MmdReg<'a, T, R: RegisterLongName> {
    reg_num: RegNum,
    reg: InMemoryRegister<u16, R>,
    mmd_addr: u16,
    phy: &'a GenPhy<'a, T>,
}

impl<'a, T, R> MmdReg<'a, T, R>
where
    R: RegisterLongName,
    T: PhyReadWrite,
{
    pub fn new(phy: &'a GenPhy<T>, reg_num: RegNum, mmd_addr: u16) -> MmdReg<'a, T, R> {
        Self {
            reg_num,
            reg: InMemoryRegister::new(0),
            mmd_addr,
            phy,
        }
    }

    pub fn from_read(phy: &'a GenPhy<T>, reg_num: RegNum, mmd_addr: u16) -> MmdReg<'a, T, R> {
        let val = phy.read_mmd(mmd_addr, reg_num);
        Self {
            reg_num,
            reg: InMemoryRegister::new(val),
            mmd_addr,
            phy,
        }
    }

    pub fn reg(&self) -> &InMemoryRegister<u16, R> {
        &self.reg
    }

    pub fn phy_write(&self) {
        let val = self.reg.get();
        self.phy.write_mmd(self.mmd_addr, self.reg_num, val);
    }

    // fn phy_read(&mut self) {
    //     let val = self.phy.read_mmd(self.reg_num);
    //     self.reg.set(val);
    // }
}

pub(crate) struct Reg<'a, T, R: RegisterLongName> {
    reg_num: RegNum,
    reg: InMemoryRegister<u16, R>,
    phy: &'a GenPhy<'a, T>,
}

impl<'a, T, R> BitAndAssign for Reg<'a, T, R>
where
    R: RegisterLongName,
    T: PhyReadWrite,
{
    fn bitand_assign(&mut self, rhs: Self) {
        let val = self.reg().get() & rhs.reg().get();
        self.reg().set(val);
    }
}

impl<'a, T, R> Reg<'a, T, R>
where
    R: RegisterLongName,
    T: PhyReadWrite,
{
    pub(crate) fn new(phy: &'a GenPhy<T>, reg_num: RegNum) -> Reg<'a, T, R> {
        Self {
            reg_num,
            reg: InMemoryRegister::new(0),
            phy,
        }
    }

    pub(crate) fn from_read(phy: &'a GenPhy<T>, reg_num: RegNum) -> Reg<'a, T, R> {
        let val = phy.read(reg_num);
        Self {
            reg_num,
            reg: InMemoryRegister::new(val),
            phy,
        }
    }

    pub(crate) fn reg(&self) -> &InMemoryRegister<u16, R> {
        &self.reg
    }

    pub(crate) fn phy_write(&self) {
        let val = self.reg.get();
        self.phy.write(self.reg_num, val);
    }

    pub(crate) fn phy_read(&mut self) {
        let val = self.phy.read(self.reg_num);
        self.reg.set(val);
    }
}

pub struct GenPhy<'a, T> {
    device: &'a T,
    addr: u32,
    supported: Supported,
}

impl<'a, T> GenPhy<'a, T>
where
    T: PhyReadWrite,
{
    pub fn new(addr: u32, device: &'a T, supported: Supported) -> GenPhy<'a, T> {
        let mut gp = Self {
            device,
            addr,
            supported,
        };
        gp.addr = gp.detect(addr).unwrap();
        gp
    }

    fn is_valid_phy_reg(&self, phy_addr: u32) -> bool {
        let phyreg = self
            .device
            .phy_read(phy_addr, RegNum::Phy(PhyReg::DetectReg).into());
        (phyreg != 0xffff) && (phyreg & PHYREG_MASK == PHYREG_MASK)
    }

    fn detect(&self, phy_addr: u32) -> Result<u32, &str> {
        if self.is_valid_phy_reg(phy_addr) {
            Ok(phy_addr)
        } else {
            for i in (0..32).rev() {
                if self.is_valid_phy_reg(i) {
                    return Ok(i);
                }
            }

            Err("No valid reg found")
        }
    }

    pub(crate) fn write(&self, regnum: RegNum, data: u16) {
        self.device.phy_write(self.addr, regnum.into(), data);
    }

    fn read(&self, regnum: RegNum) -> u16 {
        self.device.phy_read(self.addr, regnum.into())
    }

    // TODO: Can we do the mmd_addr a better way
    fn phy_mmd_start_indirect(&self, addr: u16, regnum: u16) {
        /* Write the desired MMD Devad */
        self.write(RegNum::Mii(Mii::MmdCtrl), addr);

        /* Write the desired MMD register address */
        self.write(RegNum::Mii(Mii::MmdData), regnum);

        /* Select the Function : DATA with no post increment */
        self.write(RegNum::Mii(Mii::MmdCtrl), addr | MII_MMD_CTRL_NOINCR);
    }

    fn read_mmd(&self, addr: u16, regnum: RegNum) -> u16 {
        self.phy_mmd_start_indirect(addr, regnum.into());
        self.read(RegNum::Mii(Mii::MmdData))
    }

    fn write_mmd(&self, addr: u16, regnum: RegNum, data: u16) {
        self.phy_mmd_start_indirect(addr, regnum.into());
        self.write(RegNum::Mii(Mii::MmdData), data);
    }

    /**
     * genphy_restart_aneg - Enable and Restart Autonegotiation
     * @phydev: target phy_device struct
     */
    fn restart_aneg(&self) {
        let bmcr: Reg<T, Bmcr::Register> = Reg::from_read(self, RegNum::Mii(Mii::Bmcr));

        /* Don't isolate the PHY if we're negotiating */
        bmcr.reg()
            .modify(Bmcr::ANENABLE::SET + Bmcr::ANRESTART::SET + Bmcr::ISOLATE::CLEAR);
        bmcr.phy_write();
    }

    fn config_advert(&self) -> Result<bool, &str> {
        let mut changed = false;

        /* Setup standard advertisement */
        let adv: Reg<T, Advertise::Register> = Reg::from_read(self, RegNum::Mii(Mii::Advertise));
        let init_adv = adv.reg().get();

        let mut adv_mod = match self.supported.base10_t_half {
            true => Advertise::HALF10::SET,
            false => Advertise::HALF10::CLEAR,
        };
        adv_mod += match self.supported.base10_t_full {
            true => Advertise::FULL10::SET,
            false => Advertise::FULL10::CLEAR,
        };
        adv_mod += match self.supported.base100_t_half {
            true => Advertise::HALF100::SET,
            false => Advertise::HALF100::CLEAR,
        };
        adv_mod += match self.supported.base100_t_full {
            true => Advertise::FULL100::SET,
            false => Advertise::FULL100::CLEAR,
        };
        adv_mod += match self.supported.pause {
            true => Advertise::PAUSE_CAP::SET,
            false => Advertise::PAUSE_CAP::CLEAR,
        };
        adv_mod += match self.supported.asym_pause {
            true => Advertise::PAUSE_ASYM::SET,
            false => Advertise::PAUSE_ASYM::CLEAR,
        };
        adv_mod += match self.supported.base1000_x_half {
            true => Advertise::FULL10::SET,
            false => Advertise::FULL10::CLEAR,
        };
        adv_mod += match self.supported.base1000_x_full {
            true => Advertise::HALF10::SET,
            false => Advertise::HALF10::CLEAR,
        };
        adv.reg().modify(adv_mod);
        if adv.reg().get() != init_adv {
            adv.phy_write();
            changed = true;
        }

        /* Per 802.3-2008, Section 22.2.4.2.16 Extended status all
         * 1000Mbits/sec capable PHYs shall have the Bmsr::ESTATEN bit set to a
         * logical 1.
         */
        let bmsr: Reg<T, Bmsr::Register> = Reg::from_read(self, RegNum::Mii(Mii::Bmsr));
        if !bmsr.reg().is_set(Bmsr::ESTATEN) {
            return Err("Bad BMSR Setting");
        }
        /* Configure gigabit if it's supported */
        let base_ctrl: Reg<T, Base1000TCtrl::Register> =
            Reg::from_read(self, RegNum::Mii(Mii::Ctrl1000));
        let init_base_ctrl = base_ctrl.reg().get();

        base_ctrl
            .reg()
            .modify(Base1000TCtrl::FULL::CLEAR + Base1000TCtrl::HALF::CLEAR);

        if self.supported.base1000_t_half || self.supported.base1000_t_full {
            if self.supported.base1000_t_half {
                base_ctrl.reg().modify(Base1000TCtrl::HALF::SET);
            }
            if self.supported.base1000_t_full {
                base_ctrl.reg().modify(Base1000TCtrl::FULL::SET);
            }
        }

        if base_ctrl.reg().get() != init_base_ctrl {
            base_ctrl.phy_write();
            changed = true;
        }

        Ok(changed)
    }

    pub(crate) fn config_aneg(&self) -> Result<(), &str> {
        let mut changed = self.config_advert()?;

        if !changed {
            /*
             * Advertisment hasn't changed, but maybe aneg was never on to
             * begin with?  Or maybe phy was isolated?
             */
            let bmcr: Reg<T, Bmcr::Register> = Reg::from_read(self, RegNum::Mii(Mii::Bmcr));

            if bmcr
                .reg()
                .matches_any(&[Bmcr::ISOLATE::SET, Bmcr::ANENABLE::CLEAR])
            {
                changed = true;
            }
        }

        if changed {
            self.restart_aneg();
        }

        Ok(())
    }

    pub fn startup(&self) -> (Speed, Duplex) {
        self.update_link();
        self.parse_link()
    }

    fn update_link(&self) {
        /*
         * Wait if the link is up, and autonegotiation is in progress
         * (ie - we're capable and it's not done)
         */
        let mut bmsr: Reg<T, Bmsr::Register> = Reg::from_read(self, RegNum::Mii(Mii::Bmsr));

        /*
         * If we already saw the link up, and it hasn't gone down, then
         * we don't need to wait for autoneg again
         */
        if !bmsr.reg().is_set(Bmsr::LSTATUS) {
            if bmsr.reg().is_set(Bmsr::ANEGCOMPLETE) {
                /* Read the link a second time to clear the latched state */
                bmsr.phy_read();
            } else {
                while !bmsr.reg().is_set(Bmsr::ANEGCOMPLETE) {
                    bmsr.phy_read();
                    core::hint::spin_loop();
                }
            }
        }
    }

    fn parse_link(&self) -> (Speed, Duplex) {
        let mut speed = Speed::S10;
        let mut duplex = Duplex::Half;

        /* Check for gigabit capability */
        if self.supported.base1000_t_full || self.supported.base1000_t_half {
            /* We want a list of states supported by
             * both PHYs in the link
             */
            let gblpa: Reg<T, Stat1000::Register> =
                Reg::from_read(self, RegNum::Mii(Mii::Stat1000));
            // Value read from Mii:Ctrl1000 needs to be shifted 2 to match up with Mii::Stat1000
            // TODO: Is there a better way to do this?
            let val = gblpa.reg().get() & (self.read(RegNum::Mii(Mii::Ctrl1000)) << 2);
            gblpa.reg().set(val);

            /* Check the gigabit fields */
            if gblpa
                .reg()
                .any_matching_bits_set(Stat1000::BTSR_1000FD::SET + Stat1000::BTSR_1000HD::SET)
            {
                speed = Speed::S1000;

                if gblpa.reg().is_set(Stat1000::BTSR_1000FD) {
                    duplex = Duplex::Full;
                }
                /* We're done! */
                return (speed, duplex);
            }
        }

        let mut adv: Reg<T, Advertise::Register> =
            Reg::from_read(self, RegNum::Mii(Mii::Advertise));
        let lpa: Reg<T, Advertise::Register> = Reg::from_read(self, RegNum::Mii(Mii::Lpa));
        adv &= lpa;

        if adv
            .reg()
            .any_matching_bits_set(Advertise::FULL100::SET + Advertise::HALF100::SET)
        {
            speed = Speed::S100;

            if adv.reg().is_set(Advertise::FULL100) {
                duplex = Duplex::Full;
            }
        } else if adv.reg().is_set(Advertise::FULL10) {
            duplex = Duplex::Full;
        }

        /*
         * Extended status may indicate that the PHY supports
         * 1000BASE-T/X even though the 1000BASE-T registers
         * are missing. In this case we can't tell whether the
         * peer also supports it, so we only check extended
         * status if the 1000BASE-T registers are actually
         * missing.
         */
        let bmsr: Reg<T, Bmsr::Register> = Reg::from_read(self, RegNum::Mii(Mii::Bmsr));
        if bmsr
            .reg()
            .matches_all(Bmsr::ESTATEN::SET + Bmsr::ERCAP::CLEAR)
        {
            let estatus: Reg<T, Estatus::Register> =
                Reg::from_read(self, RegNum::Mii(Mii::EStatus));

            if estatus.reg().any_matching_bits_set(
                Estatus::TFULL1000::SET
                    + Estatus::THALF1000::SET
                    + Estatus::XFULL1000::SET
                    + Estatus::XHALF1000::SET,
            ) {
                speed = Speed::S1000;
                if estatus
                    .reg()
                    .any_matching_bits_set(Estatus::TFULL1000::SET + Estatus::XFULL1000::SET)
                {
                    duplex = Duplex::Full;
                }
            }
        }

        (speed, duplex)
    }
}
