use self::config::*;

pub mod config {
    pub use crate::sdrampins::*;

    pub enum Bank {
        Bank1,
        Bank2,
    }

    pub struct SdRamCfg {
        /// The number of column bits.
        pub col_bits: u32,
        /// The number of row bits.
        pub row_bits: u32,
        /// The memory width.
        pub mem_width: u32,
        /// The number of bank.
        pub bank_count: u32,
        /// The number of rows.
        pub row_count: u32,
        /// The refresh period.
        pub refresh_period: Timing,
        /// Row address to column address delay.
        pub t_rcd: Timing,
        /// Row precharge time.
        pub t_rp: Timing,
        /// Min row active time.
        pub t_ras_min: Timing,
        /// Write recovery time.
        pub t_wr: Timing,
        /// Row cycle delay.
        pub t_rc: Timing,
        /// Exit self refresh ro active time.
        pub t_xsr: Timing,
        /// loadModeRegisterToActive
        pub t_mrd: Timing,
        /// autoRefreshCycles
        pub auto_refresh: Timing,
    }

    pub enum Timing {
        Ns(u32),
        Ms(u32),
        Cycles(u32)
    }
}

pub struct FmcDrv {
}

impl FmcDrv {
    pub fn init_sdram<Sdcke0, Sdcke1, Sdne0, Sdne1, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31, NBL2, NBL3>(
        bank: Bank,
        cfg: SdRamCfg,
        pins: FmcSdRamPins<D, Sdcke0, Sdcke1, Sdne0, Sdne1, D, D, D>,
        address_pins: FmcSdRamAddressPins<D, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12>,
        data_pins: FmcSdRamDataPins<D, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12, D13, D14, D15, D16, D17, D18, D19, D20, D21, D22, D23, D24, D25, D26, D27, D28, D29, D30, D31>,
        bank_pins: FmcSdRamBankPins<D, D>,
        mask_pins: FmcSdRamByteMaskPins<D, D, NBL2, NBL3>,
    ) {
        



    }
}