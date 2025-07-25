#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    uart: [UART; 3],
    mspi_conf: MSPI_CONF,
    mspi_clk_conf: MSPI_CLK_CONF,
    i2c_conf: I2C_CONF,
    i2c_sclk_conf: I2C_SCLK_CONF,
    ledc_conf: LEDC_CONF,
    ledc_sclk_conf: LEDC_SCLK_CONF,
    ledc_pd_ctrl: LEDC_PD_CTRL,
    timergroup0_conf: TIMERGROUP0_CONF,
    timergroup0_timer_clk_conf: TIMERGROUP0_TIMER_CLK_CONF,
    timergroup0_wdt_clk_conf: TIMERGROUP0_WDT_CLK_CONF,
    timergroup1_conf: TIMERGROUP1_CONF,
    timergroup1_timer_clk_conf: TIMERGROUP1_TIMER_CLK_CONF,
    timergroup1_wdt_clk_conf: TIMERGROUP1_WDT_CLK_CONF,
    systimer_conf: SYSTIMER_CONF,
    systimer_func_clk_conf: SYSTIMER_FUNC_CLK_CONF,
    i2s_conf: I2S_CONF,
    i2s_tx_clkm_conf: I2S_TX_CLKM_CONF,
    i2s_tx_clkm_div_conf: I2S_TX_CLKM_DIV_CONF,
    i2s_rx_clkm_conf: I2S_RX_CLKM_CONF,
    i2s_rx_clkm_div_conf: I2S_RX_CLKM_DIV_CONF,
    saradc_conf: SARADC_CONF,
    saradc_clkm_conf: SARADC_CLKM_CONF,
    tsens_clk_conf: TSENS_CLK_CONF,
    usb_device_conf: USB_DEVICE_CONF,
    intmtx_conf: INTMTX_CONF,
    pvt_monitor_conf: PVT_MONITOR_CONF,
    pvt_monitor_func_clk_conf: PVT_MONITOR_FUNC_CLK_CONF,
    gdma_conf: GDMA_CONF,
    spi2_conf: SPI2_CONF,
    spi2_clkm_conf: SPI2_CLKM_CONF,
    aes_conf: AES_CONF,
    sha_conf: SHA_CONF,
    rsa_conf: RSA_CONF,
    rsa_pd_ctrl: RSA_PD_CTRL,
    ecc_conf: ECC_CONF,
    ecc_pd_ctrl: ECC_PD_CTRL,
    ds_conf: DS_CONF,
    hmac_conf: HMAC_CONF,
    ecdsa_conf: ECDSA_CONF,
    iomux_conf: IOMUX_CONF,
    iomux_clk_conf: IOMUX_CLK_CONF,
    tcm_mem_monitor_conf: TCM_MEM_MONITOR_CONF,
    psram_mem_monitor_conf: PSRAM_MEM_MONITOR_CONF,
    trace_conf: TRACE_CONF,
    assist_conf: ASSIST_CONF,
    cache_conf: CACHE_CONF,
    cache_pd_ctrl: CACHE_PD_CTRL,
    modem_conf: MODEM_CONF,
    timeout_conf: TIMEOUT_CONF,
    sysclk_conf: SYSCLK_CONF,
    cpu_waiti_conf: CPU_WAITI_CONF,
    cpu_freq_conf: CPU_FREQ_CONF,
    ahb_freq_conf: AHB_FREQ_CONF,
    apb_freq_conf: APB_FREQ_CONF,
    sysclk_freq_query_0: SYSCLK_FREQ_QUERY_0,
    pll_div_clk_en: PLL_DIV_CLK_EN,
    ctrl_clk_out_en: CTRL_CLK_OUT_EN,
    _reserved58: [u8; 0x04],
    ctrl_32k_conf: CTRL_32K_CONF,
    sram_power_conf_0: SRAM_POWER_CONF_0,
    sram_power_conf_1: SRAM_POWER_CONF_1,
    sec_conf: SEC_CONF,
    adc_dac_inv_phase_conf: ADC_DAC_INV_PHASE_CONF,
    bus_clk_update: BUS_CLK_UPDATE,
    sar_clk_div: SAR_CLK_DIV,
    pwdet_sar_clk_conf: PWDET_SAR_CLK_CONF,
    timergroup_wdt_conf: TIMERGROUP_WDT_CONF,
    timergroup_xtal_conf: TIMERGROUP_XTAL_CONF,
    reset_event_bypass: RESET_EVENT_BYPASS,
    regdma_conf: REGDMA_CONF,
    etm_conf: ETM_CONF,
    _reserved71: [u8; 0x18],
    sdio_slave_conf: SDIO_SLAVE_CONF,
    sdio_slave_pd_ctrl: SDIO_SLAVE_PD_CTRL,
    _reserved73: [u8; 0x0e94],
    fpga_debug: FPGA_DEBUG,
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x24 - Cluster UART%s, containing UART?_CONF, UART?_SCLK_CONF, UART?_PD_CTRL"]
    #[inline(always)]
    pub const fn uart(&self, n: usize) -> &UART {
        &self.uart[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x24 - Cluster UART%s, containing UART?_CONF, UART?_SCLK_CONF, UART?_PD_CTRL"]
    #[inline(always)]
    pub fn uart_iter(&self) -> impl Iterator<Item = &UART> {
        self.uart.iter()
    }
    #[doc = "0x24 - MSPI configuration register"]
    #[inline(always)]
    pub const fn mspi_conf(&self) -> &MSPI_CONF {
        &self.mspi_conf
    }
    #[doc = "0x28 - MSPI_CLK configuration register"]
    #[inline(always)]
    pub const fn mspi_clk_conf(&self) -> &MSPI_CLK_CONF {
        &self.mspi_clk_conf
    }
    #[doc = "0x2c - I2C configuration register"]
    #[inline(always)]
    pub const fn i2c_conf(&self) -> &I2C_CONF {
        &self.i2c_conf
    }
    #[doc = "0x30 - I2C_SCLK configuration register"]
    #[inline(always)]
    pub const fn i2c_sclk_conf(&self) -> &I2C_SCLK_CONF {
        &self.i2c_sclk_conf
    }
    #[doc = "0x34 - LEDC configuration register"]
    #[inline(always)]
    pub const fn ledc_conf(&self) -> &LEDC_CONF {
        &self.ledc_conf
    }
    #[doc = "0x38 - LEDC_SCLK configuration register"]
    #[inline(always)]
    pub const fn ledc_sclk_conf(&self) -> &LEDC_SCLK_CONF {
        &self.ledc_sclk_conf
    }
    #[doc = "0x3c - LEDC power control register"]
    #[inline(always)]
    pub const fn ledc_pd_ctrl(&self) -> &LEDC_PD_CTRL {
        &self.ledc_pd_ctrl
    }
    #[doc = "0x40 - TIMERGROUP0 configuration register"]
    #[inline(always)]
    pub const fn timergroup0_conf(&self) -> &TIMERGROUP0_CONF {
        &self.timergroup0_conf
    }
    #[doc = "0x44 - TIMERGROUP0_TIMER_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup0_timer_clk_conf(&self) -> &TIMERGROUP0_TIMER_CLK_CONF {
        &self.timergroup0_timer_clk_conf
    }
    #[doc = "0x48 - TIMERGROUP0_WDT_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup0_wdt_clk_conf(&self) -> &TIMERGROUP0_WDT_CLK_CONF {
        &self.timergroup0_wdt_clk_conf
    }
    #[doc = "0x4c - TIMERGROUP1 configuration register"]
    #[inline(always)]
    pub const fn timergroup1_conf(&self) -> &TIMERGROUP1_CONF {
        &self.timergroup1_conf
    }
    #[doc = "0x50 - TIMERGROUP1_TIMER_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup1_timer_clk_conf(&self) -> &TIMERGROUP1_TIMER_CLK_CONF {
        &self.timergroup1_timer_clk_conf
    }
    #[doc = "0x54 - TIMERGROUP1_WDT_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup1_wdt_clk_conf(&self) -> &TIMERGROUP1_WDT_CLK_CONF {
        &self.timergroup1_wdt_clk_conf
    }
    #[doc = "0x58 - SYSTIMER configuration register"]
    #[inline(always)]
    pub const fn systimer_conf(&self) -> &SYSTIMER_CONF {
        &self.systimer_conf
    }
    #[doc = "0x5c - SYSTIMER_FUNC_CLK configuration register"]
    #[inline(always)]
    pub const fn systimer_func_clk_conf(&self) -> &SYSTIMER_FUNC_CLK_CONF {
        &self.systimer_func_clk_conf
    }
    #[doc = "0x60 - I2S configuration register"]
    #[inline(always)]
    pub const fn i2s_conf(&self) -> &I2S_CONF {
        &self.i2s_conf
    }
    #[doc = "0x64 - I2S_TX_CLKM configuration register"]
    #[inline(always)]
    pub const fn i2s_tx_clkm_conf(&self) -> &I2S_TX_CLKM_CONF {
        &self.i2s_tx_clkm_conf
    }
    #[doc = "0x68 - I2S_TX_CLKM_DIV configuration register"]
    #[inline(always)]
    pub const fn i2s_tx_clkm_div_conf(&self) -> &I2S_TX_CLKM_DIV_CONF {
        &self.i2s_tx_clkm_div_conf
    }
    #[doc = "0x6c - I2S_RX_CLKM configuration register"]
    #[inline(always)]
    pub const fn i2s_rx_clkm_conf(&self) -> &I2S_RX_CLKM_CONF {
        &self.i2s_rx_clkm_conf
    }
    #[doc = "0x70 - I2S_RX_CLKM_DIV configuration register"]
    #[inline(always)]
    pub const fn i2s_rx_clkm_div_conf(&self) -> &I2S_RX_CLKM_DIV_CONF {
        &self.i2s_rx_clkm_div_conf
    }
    #[doc = "0x74 - SARADC configuration register"]
    #[inline(always)]
    pub const fn saradc_conf(&self) -> &SARADC_CONF {
        &self.saradc_conf
    }
    #[doc = "0x78 - SARADC_CLKM configuration register"]
    #[inline(always)]
    pub const fn saradc_clkm_conf(&self) -> &SARADC_CLKM_CONF {
        &self.saradc_clkm_conf
    }
    #[doc = "0x7c - TSENS_CLK configuration register"]
    #[inline(always)]
    pub const fn tsens_clk_conf(&self) -> &TSENS_CLK_CONF {
        &self.tsens_clk_conf
    }
    #[doc = "0x80 - USB_DEVICE configuration register"]
    #[inline(always)]
    pub const fn usb_device_conf(&self) -> &USB_DEVICE_CONF {
        &self.usb_device_conf
    }
    #[doc = "0x84 - INTMTX configuration register"]
    #[inline(always)]
    pub const fn intmtx_conf(&self) -> &INTMTX_CONF {
        &self.intmtx_conf
    }
    #[doc = "0x88 - PVT_MONITOR configuration register"]
    #[inline(always)]
    pub const fn pvt_monitor_conf(&self) -> &PVT_MONITOR_CONF {
        &self.pvt_monitor_conf
    }
    #[doc = "0x8c - PVT_MONITOR function clock configuration register"]
    #[inline(always)]
    pub const fn pvt_monitor_func_clk_conf(&self) -> &PVT_MONITOR_FUNC_CLK_CONF {
        &self.pvt_monitor_func_clk_conf
    }
    #[doc = "0x90 - GDMA configuration register"]
    #[inline(always)]
    pub const fn gdma_conf(&self) -> &GDMA_CONF {
        &self.gdma_conf
    }
    #[doc = "0x94 - SPI2 configuration register"]
    #[inline(always)]
    pub const fn spi2_conf(&self) -> &SPI2_CONF {
        &self.spi2_conf
    }
    #[doc = "0x98 - SPI2_CLKM configuration register"]
    #[inline(always)]
    pub const fn spi2_clkm_conf(&self) -> &SPI2_CLKM_CONF {
        &self.spi2_clkm_conf
    }
    #[doc = "0x9c - AES configuration register"]
    #[inline(always)]
    pub const fn aes_conf(&self) -> &AES_CONF {
        &self.aes_conf
    }
    #[doc = "0xa0 - SHA configuration register"]
    #[inline(always)]
    pub const fn sha_conf(&self) -> &SHA_CONF {
        &self.sha_conf
    }
    #[doc = "0xa4 - RSA configuration register"]
    #[inline(always)]
    pub const fn rsa_conf(&self) -> &RSA_CONF {
        &self.rsa_conf
    }
    #[doc = "0xa8 - RSA power control register"]
    #[inline(always)]
    pub const fn rsa_pd_ctrl(&self) -> &RSA_PD_CTRL {
        &self.rsa_pd_ctrl
    }
    #[doc = "0xac - ECC configuration register"]
    #[inline(always)]
    pub const fn ecc_conf(&self) -> &ECC_CONF {
        &self.ecc_conf
    }
    #[doc = "0xb0 - ECC power control register"]
    #[inline(always)]
    pub const fn ecc_pd_ctrl(&self) -> &ECC_PD_CTRL {
        &self.ecc_pd_ctrl
    }
    #[doc = "0xb4 - DS configuration register"]
    #[inline(always)]
    pub const fn ds_conf(&self) -> &DS_CONF {
        &self.ds_conf
    }
    #[doc = "0xb8 - HMAC configuration register"]
    #[inline(always)]
    pub const fn hmac_conf(&self) -> &HMAC_CONF {
        &self.hmac_conf
    }
    #[doc = "0xbc - ECDSA configuration register"]
    #[inline(always)]
    pub const fn ecdsa_conf(&self) -> &ECDSA_CONF {
        &self.ecdsa_conf
    }
    #[doc = "0xc0 - IOMUX configuration register"]
    #[inline(always)]
    pub const fn iomux_conf(&self) -> &IOMUX_CONF {
        &self.iomux_conf
    }
    #[doc = "0xc4 - IOMUX_CLK configuration register"]
    #[inline(always)]
    pub const fn iomux_clk_conf(&self) -> &IOMUX_CLK_CONF {
        &self.iomux_clk_conf
    }
    #[doc = "0xc8 - TCM_MEM_MONITOR configuration register"]
    #[inline(always)]
    pub const fn tcm_mem_monitor_conf(&self) -> &TCM_MEM_MONITOR_CONF {
        &self.tcm_mem_monitor_conf
    }
    #[doc = "0xcc - PSRAM_MEM_MONITOR configuration register"]
    #[inline(always)]
    pub const fn psram_mem_monitor_conf(&self) -> &PSRAM_MEM_MONITOR_CONF {
        &self.psram_mem_monitor_conf
    }
    #[doc = "0xd0 - TRACE configuration register"]
    #[inline(always)]
    pub const fn trace_conf(&self) -> &TRACE_CONF {
        &self.trace_conf
    }
    #[doc = "0xd4 - ASSIST configuration register"]
    #[inline(always)]
    pub const fn assist_conf(&self) -> &ASSIST_CONF {
        &self.assist_conf
    }
    #[doc = "0xd8 - CACHE configuration register"]
    #[inline(always)]
    pub const fn cache_conf(&self) -> &CACHE_CONF {
        &self.cache_conf
    }
    #[doc = "0xdc - CACHE power control register"]
    #[inline(always)]
    pub const fn cache_pd_ctrl(&self) -> &CACHE_PD_CTRL {
        &self.cache_pd_ctrl
    }
    #[doc = "0xe0 - MODEM_APB configuration register"]
    #[inline(always)]
    pub const fn modem_conf(&self) -> &MODEM_CONF {
        &self.modem_conf
    }
    #[doc = "0xe4 - TIMEOUT configuration register"]
    #[inline(always)]
    pub const fn timeout_conf(&self) -> &TIMEOUT_CONF {
        &self.timeout_conf
    }
    #[doc = "0xe8 - SYSCLK configuration register"]
    #[inline(always)]
    pub const fn sysclk_conf(&self) -> &SYSCLK_CONF {
        &self.sysclk_conf
    }
    #[doc = "0xec - CPU_WAITI configuration register"]
    #[inline(always)]
    pub const fn cpu_waiti_conf(&self) -> &CPU_WAITI_CONF {
        &self.cpu_waiti_conf
    }
    #[doc = "0xf0 - CPU_FREQ configuration register"]
    #[inline(always)]
    pub const fn cpu_freq_conf(&self) -> &CPU_FREQ_CONF {
        &self.cpu_freq_conf
    }
    #[doc = "0xf4 - AHB_FREQ configuration register"]
    #[inline(always)]
    pub const fn ahb_freq_conf(&self) -> &AHB_FREQ_CONF {
        &self.ahb_freq_conf
    }
    #[doc = "0xf8 - APB_FREQ configuration register"]
    #[inline(always)]
    pub const fn apb_freq_conf(&self) -> &APB_FREQ_CONF {
        &self.apb_freq_conf
    }
    #[doc = "0xfc - SYSCLK frequency query 0 register"]
    #[inline(always)]
    pub const fn sysclk_freq_query_0(&self) -> &SYSCLK_FREQ_QUERY_0 {
        &self.sysclk_freq_query_0
    }
    #[doc = "0x100 - SPLL DIV clock-gating configuration register"]
    #[inline(always)]
    pub const fn pll_div_clk_en(&self) -> &PLL_DIV_CLK_EN {
        &self.pll_div_clk_en
    }
    #[doc = "0x104 - CLK_OUT_EN configuration register"]
    #[inline(always)]
    pub const fn ctrl_clk_out_en(&self) -> &CTRL_CLK_OUT_EN {
        &self.ctrl_clk_out_en
    }
    #[doc = "0x10c - 32KHz clock configuration register"]
    #[inline(always)]
    pub const fn ctrl_32k_conf(&self) -> &CTRL_32K_CONF {
        &self.ctrl_32k_conf
    }
    #[doc = "0x110 - HP SRAM/ROM configuration register"]
    #[inline(always)]
    pub const fn sram_power_conf_0(&self) -> &SRAM_POWER_CONF_0 {
        &self.sram_power_conf_0
    }
    #[doc = "0x114 - HP SRAM/ROM configuration register"]
    #[inline(always)]
    pub const fn sram_power_conf_1(&self) -> &SRAM_POWER_CONF_1 {
        &self.sram_power_conf_1
    }
    #[doc = "0x118 - Clock source configuration register for External Memory Encryption and Decryption"]
    #[inline(always)]
    pub const fn sec_conf(&self) -> &SEC_CONF {
        &self.sec_conf
    }
    #[doc = "0x11c - xxxx"]
    #[inline(always)]
    pub const fn adc_dac_inv_phase_conf(&self) -> &ADC_DAC_INV_PHASE_CONF {
        &self.adc_dac_inv_phase_conf
    }
    #[doc = "0x120 - Configuration register for applying updated high-performance system clock sources"]
    #[inline(always)]
    pub const fn bus_clk_update(&self) -> &BUS_CLK_UPDATE {
        &self.bus_clk_update
    }
    #[doc = "0x124 - SAR ADC clock divisor configuration register"]
    #[inline(always)]
    pub const fn sar_clk_div(&self) -> &SAR_CLK_DIV {
        &self.sar_clk_div
    }
    #[doc = "0x128 - xxxx"]
    #[inline(always)]
    pub const fn pwdet_sar_clk_conf(&self) -> &PWDET_SAR_CLK_CONF {
        &self.pwdet_sar_clk_conf
    }
    #[doc = "0x12c - TIMERGROUP_WDT configuration register"]
    #[inline(always)]
    pub const fn timergroup_wdt_conf(&self) -> &TIMERGROUP_WDT_CONF {
        &self.timergroup_wdt_conf
    }
    #[doc = "0x130 - TIMERGROUP1 configuration register"]
    #[inline(always)]
    pub const fn timergroup_xtal_conf(&self) -> &TIMERGROUP_XTAL_CONF {
        &self.timergroup_xtal_conf
    }
    #[doc = "0x134 - reset event bypass backdoor configuration register"]
    #[inline(always)]
    pub const fn reset_event_bypass(&self) -> &RESET_EVENT_BYPASS {
        &self.reset_event_bypass
    }
    #[doc = "0x138 - REGDMA configuration register"]
    #[inline(always)]
    pub const fn regdma_conf(&self) -> &REGDMA_CONF {
        &self.regdma_conf
    }
    #[doc = "0x13c - ETM configuration register"]
    #[inline(always)]
    pub const fn etm_conf(&self) -> &ETM_CONF {
        &self.etm_conf
    }
    #[doc = "0x158 - SDIO_SLAVE configuration register"]
    #[inline(always)]
    pub const fn sdio_slave_conf(&self) -> &SDIO_SLAVE_CONF {
        &self.sdio_slave_conf
    }
    #[doc = "0x15c - LEDC power control register"]
    #[inline(always)]
    pub const fn sdio_slave_pd_ctrl(&self) -> &SDIO_SLAVE_PD_CTRL {
        &self.sdio_slave_pd_ctrl
    }
    #[doc = "0xff4 - fpga debug register"]
    #[inline(always)]
    pub const fn fpga_debug(&self) -> &FPGA_DEBUG {
        &self.fpga_debug
    }
    #[doc = "0xff8 - PCR clock gating configure register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "Cluster UART%s, containing UART?_CONF, UART?_SCLK_CONF, UART?_PD_CTRL"]
pub use self::uart::UART;
#[doc = r"Cluster"]
#[doc = "Cluster UART%s, containing UART?_CONF, UART?_SCLK_CONF, UART?_PD_CTRL"]
pub mod uart;
#[doc = "MSPI_CONF (rw) register accessor: MSPI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_conf`] module"]
pub type MSPI_CONF = crate::Reg<mspi_conf::MSPI_CONF_SPEC>;
#[doc = "MSPI configuration register"]
pub mod mspi_conf;
#[doc = "MSPI_CLK_CONF (rw) register accessor: MSPI_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_clk_conf`] module"]
pub type MSPI_CLK_CONF = crate::Reg<mspi_clk_conf::MSPI_CLK_CONF_SPEC>;
#[doc = "MSPI_CLK configuration register"]
pub mod mspi_clk_conf;
#[doc = "I2C_CONF (rw) register accessor: I2C configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_conf`] module"]
pub type I2C_CONF = crate::Reg<i2c_conf::I2C_CONF_SPEC>;
#[doc = "I2C configuration register"]
pub mod i2c_conf;
#[doc = "I2C_SCLK_CONF (rw) register accessor: I2C_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sclk_conf`] module"]
pub type I2C_SCLK_CONF = crate::Reg<i2c_sclk_conf::I2C_SCLK_CONF_SPEC>;
#[doc = "I2C_SCLK configuration register"]
pub mod i2c_sclk_conf;
#[doc = "LEDC_CONF (rw) register accessor: LEDC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_conf`] module"]
pub type LEDC_CONF = crate::Reg<ledc_conf::LEDC_CONF_SPEC>;
#[doc = "LEDC configuration register"]
pub mod ledc_conf;
#[doc = "LEDC_SCLK_CONF (rw) register accessor: LEDC_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_sclk_conf`] module"]
pub type LEDC_SCLK_CONF = crate::Reg<ledc_sclk_conf::LEDC_SCLK_CONF_SPEC>;
#[doc = "LEDC_SCLK configuration register"]
pub mod ledc_sclk_conf;
#[doc = "LEDC_PD_CTRL (rw) register accessor: LEDC power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_pd_ctrl`] module"]
pub type LEDC_PD_CTRL = crate::Reg<ledc_pd_ctrl::LEDC_PD_CTRL_SPEC>;
#[doc = "LEDC power control register"]
pub mod ledc_pd_ctrl;
#[doc = "TIMERGROUP0_CONF (rw) register accessor: TIMERGROUP0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup0_conf`] module"]
pub type TIMERGROUP0_CONF = crate::Reg<timergroup0_conf::TIMERGROUP0_CONF_SPEC>;
#[doc = "TIMERGROUP0 configuration register"]
pub mod timergroup0_conf;
#[doc = "TIMERGROUP0_TIMER_CLK_CONF (rw) register accessor: TIMERGROUP0_TIMER_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_timer_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_timer_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup0_timer_clk_conf`] module"]
pub type TIMERGROUP0_TIMER_CLK_CONF =
    crate::Reg<timergroup0_timer_clk_conf::TIMERGROUP0_TIMER_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP0_TIMER_CLK configuration register"]
pub mod timergroup0_timer_clk_conf;
#[doc = "TIMERGROUP0_WDT_CLK_CONF (rw) register accessor: TIMERGROUP0_WDT_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_wdt_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_wdt_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup0_wdt_clk_conf`] module"]
pub type TIMERGROUP0_WDT_CLK_CONF =
    crate::Reg<timergroup0_wdt_clk_conf::TIMERGROUP0_WDT_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP0_WDT_CLK configuration register"]
pub mod timergroup0_wdt_clk_conf;
#[doc = "TIMERGROUP1_CONF (rw) register accessor: TIMERGROUP1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup1_conf`] module"]
pub type TIMERGROUP1_CONF = crate::Reg<timergroup1_conf::TIMERGROUP1_CONF_SPEC>;
#[doc = "TIMERGROUP1 configuration register"]
pub mod timergroup1_conf;
#[doc = "TIMERGROUP1_TIMER_CLK_CONF (rw) register accessor: TIMERGROUP1_TIMER_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_timer_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_timer_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup1_timer_clk_conf`] module"]
pub type TIMERGROUP1_TIMER_CLK_CONF =
    crate::Reg<timergroup1_timer_clk_conf::TIMERGROUP1_TIMER_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP1_TIMER_CLK configuration register"]
pub mod timergroup1_timer_clk_conf;
#[doc = "TIMERGROUP1_WDT_CLK_CONF (rw) register accessor: TIMERGROUP1_WDT_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_wdt_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_wdt_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup1_wdt_clk_conf`] module"]
pub type TIMERGROUP1_WDT_CLK_CONF =
    crate::Reg<timergroup1_wdt_clk_conf::TIMERGROUP1_WDT_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP1_WDT_CLK configuration register"]
pub mod timergroup1_wdt_clk_conf;
#[doc = "SYSTIMER_CONF (rw) register accessor: SYSTIMER configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_conf`] module"]
pub type SYSTIMER_CONF = crate::Reg<systimer_conf::SYSTIMER_CONF_SPEC>;
#[doc = "SYSTIMER configuration register"]
pub mod systimer_conf;
#[doc = "SYSTIMER_FUNC_CLK_CONF (rw) register accessor: SYSTIMER_FUNC_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_func_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_func_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_func_clk_conf`] module"]
pub type SYSTIMER_FUNC_CLK_CONF = crate::Reg<systimer_func_clk_conf::SYSTIMER_FUNC_CLK_CONF_SPEC>;
#[doc = "SYSTIMER_FUNC_CLK configuration register"]
pub mod systimer_func_clk_conf;
#[doc = "I2S_CONF (rw) register accessor: I2S configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_conf`] module"]
pub type I2S_CONF = crate::Reg<i2s_conf::I2S_CONF_SPEC>;
#[doc = "I2S configuration register"]
pub mod i2s_conf;
#[doc = "I2S_TX_CLKM_CONF (rw) register accessor: I2S_TX_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_tx_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_tx_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_tx_clkm_conf`] module"]
pub type I2S_TX_CLKM_CONF = crate::Reg<i2s_tx_clkm_conf::I2S_TX_CLKM_CONF_SPEC>;
#[doc = "I2S_TX_CLKM configuration register"]
pub mod i2s_tx_clkm_conf;
#[doc = "I2S_TX_CLKM_DIV_CONF (rw) register accessor: I2S_TX_CLKM_DIV configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_tx_clkm_div_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_tx_clkm_div_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_tx_clkm_div_conf`] module"]
pub type I2S_TX_CLKM_DIV_CONF = crate::Reg<i2s_tx_clkm_div_conf::I2S_TX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S_TX_CLKM_DIV configuration register"]
pub mod i2s_tx_clkm_div_conf;
#[doc = "I2S_RX_CLKM_CONF (rw) register accessor: I2S_RX_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_rx_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_rx_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_rx_clkm_conf`] module"]
pub type I2S_RX_CLKM_CONF = crate::Reg<i2s_rx_clkm_conf::I2S_RX_CLKM_CONF_SPEC>;
#[doc = "I2S_RX_CLKM configuration register"]
pub mod i2s_rx_clkm_conf;
#[doc = "I2S_RX_CLKM_DIV_CONF (rw) register accessor: I2S_RX_CLKM_DIV configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_rx_clkm_div_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_rx_clkm_div_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_rx_clkm_div_conf`] module"]
pub type I2S_RX_CLKM_DIV_CONF = crate::Reg<i2s_rx_clkm_div_conf::I2S_RX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S_RX_CLKM_DIV configuration register"]
pub mod i2s_rx_clkm_div_conf;
#[doc = "SARADC_CONF (rw) register accessor: SARADC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`saradc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saradc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_conf`] module"]
pub type SARADC_CONF = crate::Reg<saradc_conf::SARADC_CONF_SPEC>;
#[doc = "SARADC configuration register"]
pub mod saradc_conf;
#[doc = "SARADC_CLKM_CONF (rw) register accessor: SARADC_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`saradc_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saradc_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_clkm_conf`] module"]
pub type SARADC_CLKM_CONF = crate::Reg<saradc_clkm_conf::SARADC_CLKM_CONF_SPEC>;
#[doc = "SARADC_CLKM configuration register"]
pub mod saradc_clkm_conf;
#[doc = "TSENS_CLK_CONF (rw) register accessor: TSENS_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_clk_conf`] module"]
pub type TSENS_CLK_CONF = crate::Reg<tsens_clk_conf::TSENS_CLK_CONF_SPEC>;
#[doc = "TSENS_CLK configuration register"]
pub mod tsens_clk_conf;
#[doc = "USB_DEVICE_CONF (rw) register accessor: USB_DEVICE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_device_conf`] module"]
pub type USB_DEVICE_CONF = crate::Reg<usb_device_conf::USB_DEVICE_CONF_SPEC>;
#[doc = "USB_DEVICE configuration register"]
pub mod usb_device_conf;
#[doc = "INTMTX_CONF (rw) register accessor: INTMTX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmtx_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmtx_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmtx_conf`] module"]
pub type INTMTX_CONF = crate::Reg<intmtx_conf::INTMTX_CONF_SPEC>;
#[doc = "INTMTX configuration register"]
pub mod intmtx_conf;
#[doc = "PVT_MONITOR_CONF (rw) register accessor: PVT_MONITOR configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_conf`] module"]
pub type PVT_MONITOR_CONF = crate::Reg<pvt_monitor_conf::PVT_MONITOR_CONF_SPEC>;
#[doc = "PVT_MONITOR configuration register"]
pub mod pvt_monitor_conf;
#[doc = "PVT_MONITOR_FUNC_CLK_CONF (rw) register accessor: PVT_MONITOR function clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_func_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_func_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_func_clk_conf`] module"]
pub type PVT_MONITOR_FUNC_CLK_CONF =
    crate::Reg<pvt_monitor_func_clk_conf::PVT_MONITOR_FUNC_CLK_CONF_SPEC>;
#[doc = "PVT_MONITOR function clock configuration register"]
pub mod pvt_monitor_func_clk_conf;
#[doc = "GDMA_CONF (rw) register accessor: GDMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_conf`] module"]
pub type GDMA_CONF = crate::Reg<gdma_conf::GDMA_CONF_SPEC>;
#[doc = "GDMA configuration register"]
pub mod gdma_conf;
#[doc = "SPI2_CONF (rw) register accessor: SPI2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2_conf`] module"]
pub type SPI2_CONF = crate::Reg<spi2_conf::SPI2_CONF_SPEC>;
#[doc = "SPI2 configuration register"]
pub mod spi2_conf;
#[doc = "SPI2_CLKM_CONF (rw) register accessor: SPI2_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2_clkm_conf`] module"]
pub type SPI2_CLKM_CONF = crate::Reg<spi2_clkm_conf::SPI2_CLKM_CONF_SPEC>;
#[doc = "SPI2_CLKM configuration register"]
pub mod spi2_clkm_conf;
#[doc = "AES_CONF (rw) register accessor: AES configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_conf`] module"]
pub type AES_CONF = crate::Reg<aes_conf::AES_CONF_SPEC>;
#[doc = "AES configuration register"]
pub mod aes_conf;
#[doc = "SHA_CONF (rw) register accessor: SHA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_conf`] module"]
pub type SHA_CONF = crate::Reg<sha_conf::SHA_CONF_SPEC>;
#[doc = "SHA configuration register"]
pub mod sha_conf;
#[doc = "RSA_CONF (rw) register accessor: RSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_conf`] module"]
pub type RSA_CONF = crate::Reg<rsa_conf::RSA_CONF_SPEC>;
#[doc = "RSA configuration register"]
pub mod rsa_conf;
#[doc = "RSA_PD_CTRL (rw) register accessor: RSA power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_pd_ctrl`] module"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = "RSA power control register"]
pub mod rsa_pd_ctrl;
#[doc = "ECC_CONF (rw) register accessor: ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_conf`] module"]
pub type ECC_CONF = crate::Reg<ecc_conf::ECC_CONF_SPEC>;
#[doc = "ECC configuration register"]
pub mod ecc_conf;
#[doc = "ECC_PD_CTRL (rw) register accessor: ECC power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pd_ctrl`] module"]
pub type ECC_PD_CTRL = crate::Reg<ecc_pd_ctrl::ECC_PD_CTRL_SPEC>;
#[doc = "ECC power control register"]
pub mod ecc_pd_ctrl;
#[doc = "DS_CONF (rw) register accessor: DS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds_conf`] module"]
pub type DS_CONF = crate::Reg<ds_conf::DS_CONF_SPEC>;
#[doc = "DS configuration register"]
pub mod ds_conf;
#[doc = "HMAC_CONF (rw) register accessor: HMAC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hmac_conf`] module"]
pub type HMAC_CONF = crate::Reg<hmac_conf::HMAC_CONF_SPEC>;
#[doc = "HMAC configuration register"]
pub mod hmac_conf;
#[doc = "ECDSA_CONF (rw) register accessor: ECDSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecdsa_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecdsa_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecdsa_conf`] module"]
pub type ECDSA_CONF = crate::Reg<ecdsa_conf::ECDSA_CONF_SPEC>;
#[doc = "ECDSA configuration register"]
pub mod ecdsa_conf;
#[doc = "IOMUX_CONF (rw) register accessor: IOMUX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_conf`] module"]
pub type IOMUX_CONF = crate::Reg<iomux_conf::IOMUX_CONF_SPEC>;
#[doc = "IOMUX configuration register"]
pub mod iomux_conf;
#[doc = "IOMUX_CLK_CONF (rw) register accessor: IOMUX_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_clk_conf`] module"]
pub type IOMUX_CLK_CONF = crate::Reg<iomux_clk_conf::IOMUX_CLK_CONF_SPEC>;
#[doc = "IOMUX_CLK configuration register"]
pub mod iomux_clk_conf;
#[doc = "TCM_MEM_MONITOR_CONF (rw) register accessor: TCM_MEM_MONITOR configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_mem_monitor_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_mem_monitor_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_mem_monitor_conf`] module"]
pub type TCM_MEM_MONITOR_CONF = crate::Reg<tcm_mem_monitor_conf::TCM_MEM_MONITOR_CONF_SPEC>;
#[doc = "TCM_MEM_MONITOR configuration register"]
pub mod tcm_mem_monitor_conf;
#[doc = "PSRAM_MEM_MONITOR_CONF (rw) register accessor: PSRAM_MEM_MONITOR configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_mem_monitor_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_mem_monitor_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_mem_monitor_conf`] module"]
pub type PSRAM_MEM_MONITOR_CONF = crate::Reg<psram_mem_monitor_conf::PSRAM_MEM_MONITOR_CONF_SPEC>;
#[doc = "PSRAM_MEM_MONITOR configuration register"]
pub mod psram_mem_monitor_conf;
#[doc = "TRACE_CONF (rw) register accessor: TRACE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace_conf`] module"]
pub type TRACE_CONF = crate::Reg<trace_conf::TRACE_CONF_SPEC>;
#[doc = "TRACE configuration register"]
pub mod trace_conf;
#[doc = "ASSIST_CONF (rw) register accessor: ASSIST configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`assist_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assist_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assist_conf`] module"]
pub type ASSIST_CONF = crate::Reg<assist_conf::ASSIST_CONF_SPEC>;
#[doc = "ASSIST configuration register"]
pub mod assist_conf;
#[doc = "CACHE_CONF (rw) register accessor: CACHE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_conf`] module"]
pub type CACHE_CONF = crate::Reg<cache_conf::CACHE_CONF_SPEC>;
#[doc = "CACHE configuration register"]
pub mod cache_conf;
#[doc = "CACHE_PD_CTRL (rw) register accessor: CACHE power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_pd_ctrl`] module"]
pub type CACHE_PD_CTRL = crate::Reg<cache_pd_ctrl::CACHE_PD_CTRL_SPEC>;
#[doc = "CACHE power control register"]
pub mod cache_pd_ctrl;
#[doc = "MODEM_CONF (rw) register accessor: MODEM_APB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_conf`] module"]
pub type MODEM_CONF = crate::Reg<modem_conf::MODEM_CONF_SPEC>;
#[doc = "MODEM_APB configuration register"]
pub mod modem_conf;
#[doc = "TIMEOUT_CONF (rw) register accessor: TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_conf`] module"]
pub type TIMEOUT_CONF = crate::Reg<timeout_conf::TIMEOUT_CONF_SPEC>;
#[doc = "TIMEOUT configuration register"]
pub mod timeout_conf;
#[doc = "SYSCLK_CONF (rw) register accessor: SYSCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclk_conf`] module"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "SYSCLK configuration register"]
pub mod sysclk_conf;
#[doc = "CPU_WAITI_CONF (rw) register accessor: CPU_WAITI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_waiti_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_waiti_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_waiti_conf`] module"]
pub type CPU_WAITI_CONF = crate::Reg<cpu_waiti_conf::CPU_WAITI_CONF_SPEC>;
#[doc = "CPU_WAITI configuration register"]
pub mod cpu_waiti_conf;
#[doc = "CPU_FREQ_CONF (rw) register accessor: CPU_FREQ configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_freq_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_freq_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_freq_conf`] module"]
pub type CPU_FREQ_CONF = crate::Reg<cpu_freq_conf::CPU_FREQ_CONF_SPEC>;
#[doc = "CPU_FREQ configuration register"]
pub mod cpu_freq_conf;
#[doc = "AHB_FREQ_CONF (rw) register accessor: AHB_FREQ configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_freq_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_freq_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_freq_conf`] module"]
pub type AHB_FREQ_CONF = crate::Reg<ahb_freq_conf::AHB_FREQ_CONF_SPEC>;
#[doc = "AHB_FREQ configuration register"]
pub mod ahb_freq_conf;
#[doc = "APB_FREQ_CONF (rw) register accessor: APB_FREQ configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_freq_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_freq_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_freq_conf`] module"]
pub type APB_FREQ_CONF = crate::Reg<apb_freq_conf::APB_FREQ_CONF_SPEC>;
#[doc = "APB_FREQ configuration register"]
pub mod apb_freq_conf;
#[doc = "SYSCLK_FREQ_QUERY_0 (r) register accessor: SYSCLK frequency query 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_freq_query_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclk_freq_query_0`] module"]
pub type SYSCLK_FREQ_QUERY_0 = crate::Reg<sysclk_freq_query_0::SYSCLK_FREQ_QUERY_0_SPEC>;
#[doc = "SYSCLK frequency query 0 register"]
pub mod sysclk_freq_query_0;
#[doc = "PLL_DIV_CLK_EN (rw) register accessor: SPLL DIV clock-gating configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_div_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_div_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_div_clk_en`] module"]
pub type PLL_DIV_CLK_EN = crate::Reg<pll_div_clk_en::PLL_DIV_CLK_EN_SPEC>;
#[doc = "SPLL DIV clock-gating configuration register"]
pub mod pll_div_clk_en;
#[doc = "CTRL_CLK_OUT_EN (rw) register accessor: CLK_OUT_EN configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_clk_out_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_clk_out_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_clk_out_en`] module"]
pub type CTRL_CLK_OUT_EN = crate::Reg<ctrl_clk_out_en::CTRL_CLK_OUT_EN_SPEC>;
#[doc = "CLK_OUT_EN configuration register"]
pub mod ctrl_clk_out_en;
#[doc = "CTRL_32K_CONF (rw) register accessor: 32KHz clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_32k_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_32k_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_32k_conf`] module"]
pub type CTRL_32K_CONF = crate::Reg<ctrl_32k_conf::CTRL_32K_CONF_SPEC>;
#[doc = "32KHz clock configuration register"]
pub mod ctrl_32k_conf;
#[doc = "SRAM_POWER_CONF_0 (rw) register accessor: HP SRAM/ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_power_conf_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_power_conf_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_power_conf_0`] module"]
pub type SRAM_POWER_CONF_0 = crate::Reg<sram_power_conf_0::SRAM_POWER_CONF_0_SPEC>;
#[doc = "HP SRAM/ROM configuration register"]
pub mod sram_power_conf_0;
#[doc = "SRAM_POWER_CONF_1 (rw) register accessor: HP SRAM/ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_power_conf_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_power_conf_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_power_conf_1`] module"]
pub type SRAM_POWER_CONF_1 = crate::Reg<sram_power_conf_1::SRAM_POWER_CONF_1_SPEC>;
#[doc = "HP SRAM/ROM configuration register"]
pub mod sram_power_conf_1;
#[doc = "SEC_CONF (rw) register accessor: Clock source configuration register for External Memory Encryption and Decryption\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_conf`] module"]
pub type SEC_CONF = crate::Reg<sec_conf::SEC_CONF_SPEC>;
#[doc = "Clock source configuration register for External Memory Encryption and Decryption"]
pub mod sec_conf;
#[doc = "ADC_DAC_INV_PHASE_CONF (rw) register accessor: xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dac_inv_phase_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dac_inv_phase_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dac_inv_phase_conf`] module"]
pub type ADC_DAC_INV_PHASE_CONF = crate::Reg<adc_dac_inv_phase_conf::ADC_DAC_INV_PHASE_CONF_SPEC>;
#[doc = "xxxx"]
pub mod adc_dac_inv_phase_conf;
#[doc = "BUS_CLK_UPDATE (rw) register accessor: Configuration register for applying updated high-performance system clock sources\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clk_update`] module"]
pub type BUS_CLK_UPDATE = crate::Reg<bus_clk_update::BUS_CLK_UPDATE_SPEC>;
#[doc = "Configuration register for applying updated high-performance system clock sources"]
pub mod bus_clk_update;
#[doc = "SAR_CLK_DIV (rw) register accessor: SAR ADC clock divisor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_clk_div`] module"]
pub type SAR_CLK_DIV = crate::Reg<sar_clk_div::SAR_CLK_DIV_SPEC>;
#[doc = "SAR ADC clock divisor configuration register"]
pub mod sar_clk_div;
#[doc = "PWDET_SAR_CLK_CONF (rw) register accessor: xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdet_sar_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdet_sar_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdet_sar_clk_conf`] module"]
pub type PWDET_SAR_CLK_CONF = crate::Reg<pwdet_sar_clk_conf::PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "xxxx"]
pub mod pwdet_sar_clk_conf;
#[doc = "TIMERGROUP_WDT_CONF (rw) register accessor: TIMERGROUP_WDT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup_wdt_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup_wdt_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup_wdt_conf`] module"]
pub type TIMERGROUP_WDT_CONF = crate::Reg<timergroup_wdt_conf::TIMERGROUP_WDT_CONF_SPEC>;
#[doc = "TIMERGROUP_WDT configuration register"]
pub mod timergroup_wdt_conf;
#[doc = "TIMERGROUP_XTAL_CONF (rw) register accessor: TIMERGROUP1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup_xtal_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup_xtal_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup_xtal_conf`] module"]
pub type TIMERGROUP_XTAL_CONF = crate::Reg<timergroup_xtal_conf::TIMERGROUP_XTAL_CONF_SPEC>;
#[doc = "TIMERGROUP1 configuration register"]
pub mod timergroup_xtal_conf;
#[doc = "RESET_EVENT_BYPASS (rw) register accessor: reset event bypass backdoor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_event_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_event_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_event_bypass`] module"]
pub type RESET_EVENT_BYPASS = crate::Reg<reset_event_bypass::RESET_EVENT_BYPASS_SPEC>;
#[doc = "reset event bypass backdoor configuration register"]
pub mod reset_event_bypass;
#[doc = "REGDMA_CONF (rw) register accessor: REGDMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_conf`] module"]
pub type REGDMA_CONF = crate::Reg<regdma_conf::REGDMA_CONF_SPEC>;
#[doc = "REGDMA configuration register"]
pub mod regdma_conf;
#[doc = "ETM_CONF (rw) register accessor: ETM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_conf`] module"]
pub type ETM_CONF = crate::Reg<etm_conf::ETM_CONF_SPEC>;
#[doc = "ETM configuration register"]
pub mod etm_conf;
#[doc = "SDIO_SLAVE_CONF (rw) register accessor: SDIO_SLAVE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_slave_conf`] module"]
pub type SDIO_SLAVE_CONF = crate::Reg<sdio_slave_conf::SDIO_SLAVE_CONF_SPEC>;
#[doc = "SDIO_SLAVE configuration register"]
pub mod sdio_slave_conf;
#[doc = "SDIO_SLAVE_PD_CTRL (rw) register accessor: LEDC power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_slave_pd_ctrl`] module"]
pub type SDIO_SLAVE_PD_CTRL = crate::Reg<sdio_slave_pd_ctrl::SDIO_SLAVE_PD_CTRL_SPEC>;
#[doc = "LEDC power control register"]
pub mod sdio_slave_pd_ctrl;
#[doc = "FPGA_DEBUG (rw) register accessor: fpga debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpga_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpga_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpga_debug`] module"]
pub type FPGA_DEBUG = crate::Reg<fpga_debug::FPGA_DEBUG_SPEC>;
#[doc = "fpga debug register"]
pub mod fpga_debug;
#[doc = "CLOCK_GATE (rw) register accessor: PCR clock gating configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "PCR clock gating configure register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Date register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
