//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]

pub mod dp83867;
mod genphy;
pub use genphy::GenPhy;

pub trait PhyReadWrite {
    fn phy_write(&self, phy_addr: u32, regnum: u32, data: u16);
    fn phy_read(&self, phy_addr: u32, regnum: u32) -> u16;
}

pub trait SpecPhy {
    fn config(&self);
}

#[derive(Default)]
pub struct Supported {
    pub base10_t_half: bool,
    pub base10_t_full: bool,
    pub base100_t_half: bool,
    pub base100_t_full: bool,
    pub base1000_t_half: bool,
    pub base1000_t_full: bool,
    pub autoneg: bool,
    pub tp: bool,
    pub aui: bool,
    pub mii: bool,
    pub fibre: bool,
    pub bnc: bool,
    pub base10000_t_full: bool,
    pub pause: bool,
    pub asym_pause: bool,
    pub base2500_x_full: bool,
    pub backplane: bool,
    pub base1000_kx_full: bool,
    pub base10000_kx4_full: bool,
    pub base10000_kr_full: bool,
    pub base10000_r_fec: bool,
    pub base1000_x_half: bool,
    pub base1000_x_full: bool,
}

#[derive(Debug)]
pub enum Speed {
    S10,
    S100,
    S1000,
}

#[derive(Debug)]
pub enum Duplex {
    Half,
    Full,
}

#[derive(PartialEq)]
pub enum PhyInterface {
    Na,
    Internal,
    Mii,
    Gmii,
    Sgmii,
    Tbi,
    Revmii,
    Rmii,
    Revrmii,
    Rgmii,
    RgmiiId,
    RgmiiRxid,
    RgmiiTxid,
    Rtbi,
    Smii,
    Xgmii,
    Xlgmii,
    Moca,
    Qsgmii,
    Trgmii,
}

impl PhyInterface {
    pub(crate) fn is_rgmii(&self) -> bool {
        matches!(
            self,
            Self::Rgmii | Self::RgmiiId | Self::RgmiiRxid | Self::RgmiiTxid
        )
    }
}

pub fn configure_phy<'a, T: PhyReadWrite, P: SpecPhy>(
    gen_phy: &'a GenPhy<'a, T>,
    phy: &'a P,
) -> (Speed, Duplex) {
    phy.config();
    gen_phy.startup()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
