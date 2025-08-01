#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `PARITY` reader - This register is used to configure the parity check mode. 0: even. 1: odd."]
pub type PARITY_R = crate::BitReader;
#[doc = "Field `PARITY` writer - This register is used to configure the parity check mode. 0: even. 1: odd."]
pub type PARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_EN` reader - Set this bit to enable UART parity check."]
pub type PARITY_EN_R = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - Set this bit to enable UART parity check."]
pub type PARITY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NUM` reader - This register is used to set the length of data. 0: 5 bits. 1: 6 bits. 2: 7 bits. 3: 8 bits."]
pub type BIT_NUM_R = crate::FieldReader;
#[doc = "Field `BIT_NUM` writer - This register is used to set the length of data. 0: 5 bits. 1: 6 bits. 2: 7 bits. 3: 8 bits."]
pub type BIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP_BIT_NUM` reader - This register is used to set the length of stop bit. 1: 1 bit. 2: 1.5 bits. 3: 2 bits."]
pub type STOP_BIT_NUM_R = crate::FieldReader;
#[doc = "Field `STOP_BIT_NUM` writer - This register is used to set the length of stop bit. 1: 1 bit. 2: 1.5 bits. 3: 2 bits."]
pub type STOP_BIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_RTS` reader - This register is used to configure the software RTS signal which is used in software flow control."]
pub type SW_RTS_R = crate::BitReader;
#[doc = "Field `SW_RTS` writer - This register is used to configure the software RTS signal which is used in software flow control."]
pub type SW_RTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DTR` reader - This register is used to configure the software DTR signal which is used in software flow control."]
pub type SW_DTR_R = crate::BitReader;
#[doc = "Field `SW_DTR` writer - This register is used to configure the software DTR signal which is used in software flow control."]
pub type SW_DTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXD_BRK` reader - Set this bit to enable the transmitter to send NULL characters when the process of sending data is done."]
pub type TXD_BRK_R = crate::BitReader;
#[doc = "Field `TXD_BRK` writer - Set this bit to enable the transmitter to send NULL characters when the process of sending data is done."]
pub type TXD_BRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_DPLX` reader - Set this bit to enable IrDA loopback mode."]
pub type IRDA_DPLX_R = crate::BitReader;
#[doc = "Field `IRDA_DPLX` writer - Set this bit to enable IrDA loopback mode."]
pub type IRDA_DPLX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_TX_EN` reader - This is the start enable bit for IrDA transmitter."]
pub type IRDA_TX_EN_R = crate::BitReader;
#[doc = "Field `IRDA_TX_EN` writer - This is the start enable bit for IrDA transmitter."]
pub type IRDA_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_WCTL` reader - 1: The IrDA transmitter's 11th bit is the same as 10th bit. 0: Set IrDA transmitter's 11th bit to 0."]
pub type IRDA_WCTL_R = crate::BitReader;
#[doc = "Field `IRDA_WCTL` writer - 1: The IrDA transmitter's 11th bit is the same as 10th bit. 0: Set IrDA transmitter's 11th bit to 0."]
pub type IRDA_WCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_TX_INV` reader - Set this bit to invert the level of IrDA transmitter."]
pub type IRDA_TX_INV_R = crate::BitReader;
#[doc = "Field `IRDA_TX_INV` writer - Set this bit to invert the level of IrDA transmitter."]
pub type IRDA_TX_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_RX_INV` reader - Set this bit to invert the level of IrDA receiver."]
pub type IRDA_RX_INV_R = crate::BitReader;
#[doc = "Field `IRDA_RX_INV` writer - Set this bit to invert the level of IrDA receiver."]
pub type IRDA_RX_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Set this bit to enable UART loopback test mode."]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Set this bit to enable UART loopback test mode."]
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FLOW_EN` reader - Set this bit to enable flow control function for the transmitter."]
pub type TX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `TX_FLOW_EN` writer - Set this bit to enable flow control function for the transmitter."]
pub type TX_FLOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDA_EN` reader - Set this bit to enable IrDA protocol."]
pub type IRDA_EN_R = crate::BitReader;
#[doc = "Field `IRDA_EN` writer - Set this bit to enable IrDA protocol."]
pub type IRDA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_RST` reader - Set this bit to reset the UART RX FIFO."]
pub type RXFIFO_RST_R = crate::BitReader;
#[doc = "Field `RXFIFO_RST` writer - Set this bit to reset the UART RX FIFO."]
pub type RXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_RST` reader - Set this bit to reset the UART TX FIFO."]
pub type TXFIFO_RST_R = crate::BitReader;
#[doc = "Field `TXFIFO_RST` writer - Set this bit to reset the UART TX FIFO."]
pub type TXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXD_INV` reader - Set this bit to invert the level of UART RXD signal."]
pub type RXD_INV_R = crate::BitReader;
#[doc = "Field `RXD_INV` writer - Set this bit to invert the level of UART RXD signal."]
pub type RXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_INV` reader - Set this bit to invert the level of UART CTS signal."]
pub type CTS_INV_R = crate::BitReader;
#[doc = "Field `CTS_INV` writer - Set this bit to invert the level of UART CTS signal."]
pub type CTS_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_INV` reader - Set this bit to invert the level of UART DSR signal."]
pub type DSR_INV_R = crate::BitReader;
#[doc = "Field `DSR_INV` writer - Set this bit to invert the level of UART DSR signal."]
pub type DSR_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXD_INV` reader - Set this bit to invert the level of UART TXD signal."]
pub type TXD_INV_R = crate::BitReader;
#[doc = "Field `TXD_INV` writer - Set this bit to invert the level of UART TXD signal."]
pub type TXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_INV` reader - Set this bit to invert the level of UART RTS signal."]
pub type RTS_INV_R = crate::BitReader;
#[doc = "Field `RTS_INV` writer - Set this bit to invert the level of UART RTS signal."]
pub type RTS_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR_INV` reader - Set this bit to invert the level of UART DTR signal."]
pub type DTR_INV_R = crate::BitReader;
#[doc = "Field `DTR_INV` writer - Set this bit to invert the level of UART DTR signal."]
pub type DTR_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_WR_MASK` reader - 1: The receiver stops storing data into FIFO when data is wrong. 0: The receiver stores the data even if the received data is wrong."]
pub type ERR_WR_MASK_R = crate::BitReader;
#[doc = "Field `ERR_WR_MASK` writer - 1: The receiver stops storing data into FIFO when data is wrong. 0: The receiver stores the data even if the received data is wrong."]
pub type ERR_WR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICK_REF_ALWAYS_ON` reader - This register is used to select the clock. 1: APB_CLK. 0: REF_TICK."]
pub type TICK_REF_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `TICK_REF_ALWAYS_ON` writer - This register is used to select the clock. 1: APB_CLK. 0: REF_TICK."]
pub type TICK_REF_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_EN` reader - The signal to enable UART RAM clock gating. 1: UART RAM powers on, the data of which can be read and written. 0: UART RAM powers down."]
pub type MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MEM_CLK_EN` writer - The signal to enable UART RAM clock gating. 1: UART RAM powers on, the data of which can be read and written. 0: UART RAM powers down."]
pub type MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to configure the parity check mode. 0: even. 1: odd."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable UART parity check."]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This register is used to set the length of data. 0: 5 bits. 1: 6 bits. 2: 7 bits. 3: 8 bits."]
    #[inline(always)]
    pub fn bit_num(&self) -> BIT_NUM_R {
        BIT_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - This register is used to set the length of stop bit. 1: 1 bit. 2: 1.5 bits. 3: 2 bits."]
    #[inline(always)]
    pub fn stop_bit_num(&self) -> STOP_BIT_NUM_R {
        STOP_BIT_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - This register is used to configure the software RTS signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This register is used to configure the software DTR signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SW_DTR_R {
        SW_DTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable the transmitter to send NULL characters when the process of sending data is done."]
    #[inline(always)]
    pub fn txd_brk(&self) -> TXD_BRK_R {
        TXD_BRK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable IrDA loopback mode."]
    #[inline(always)]
    pub fn irda_dplx(&self) -> IRDA_DPLX_R {
        IRDA_DPLX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the start enable bit for IrDA transmitter."]
    #[inline(always)]
    pub fn irda_tx_en(&self) -> IRDA_TX_EN_R {
        IRDA_TX_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: The IrDA transmitter's 11th bit is the same as 10th bit. 0: Set IrDA transmitter's 11th bit to 0."]
    #[inline(always)]
    pub fn irda_wctl(&self) -> IRDA_WCTL_R {
        IRDA_WCTL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to invert the level of IrDA transmitter."]
    #[inline(always)]
    pub fn irda_tx_inv(&self) -> IRDA_TX_INV_R {
        IRDA_TX_INV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to invert the level of IrDA receiver."]
    #[inline(always)]
    pub fn irda_rx_inv(&self) -> IRDA_RX_INV_R {
        IRDA_RX_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable UART loopback test mode."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable flow control function for the transmitter."]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable IrDA protocol."]
    #[inline(always)]
    pub fn irda_en(&self) -> IRDA_EN_R {
        IRDA_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to reset the UART RX FIFO."]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to reset the UART TX FIFO."]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to invert the level of UART RXD signal."]
    #[inline(always)]
    pub fn rxd_inv(&self) -> RXD_INV_R {
        RXD_INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to invert the level of UART CTS signal."]
    #[inline(always)]
    pub fn cts_inv(&self) -> CTS_INV_R {
        CTS_INV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to invert the level of UART DSR signal."]
    #[inline(always)]
    pub fn dsr_inv(&self) -> DSR_INV_R {
        DSR_INV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to invert the level of UART TXD signal."]
    #[inline(always)]
    pub fn txd_inv(&self) -> TXD_INV_R {
        TXD_INV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to invert the level of UART RTS signal."]
    #[inline(always)]
    pub fn rts_inv(&self) -> RTS_INV_R {
        RTS_INV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to invert the level of UART DTR signal."]
    #[inline(always)]
    pub fn dtr_inv(&self) -> DTR_INV_R {
        DTR_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: The receiver stops storing data into FIFO when data is wrong. 0: The receiver stores the data even if the received data is wrong."]
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ERR_WR_MASK_R {
        ERR_WR_MASK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This register is used to select the clock. 1: APB_CLK. 0: REF_TICK."]
    #[inline(always)]
    pub fn tick_ref_always_on(&self) -> TICK_REF_ALWAYS_ON_R {
        TICK_REF_ALWAYS_ON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The signal to enable UART RAM clock gating. 1: UART RAM powers on, the data of which can be read and written. 0: UART RAM powers down."]
    #[inline(always)]
    pub fn mem_clk_en(&self) -> MEM_CLK_EN_R {
        MEM_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("parity", &self.parity())
            .field("parity_en", &self.parity_en())
            .field("bit_num", &self.bit_num())
            .field("stop_bit_num", &self.stop_bit_num())
            .field("sw_rts", &self.sw_rts())
            .field("sw_dtr", &self.sw_dtr())
            .field("txd_brk", &self.txd_brk())
            .field("irda_dplx", &self.irda_dplx())
            .field("irda_tx_en", &self.irda_tx_en())
            .field("irda_wctl", &self.irda_wctl())
            .field("irda_tx_inv", &self.irda_tx_inv())
            .field("irda_rx_inv", &self.irda_rx_inv())
            .field("loopback", &self.loopback())
            .field("tx_flow_en", &self.tx_flow_en())
            .field("irda_en", &self.irda_en())
            .field("rxfifo_rst", &self.rxfifo_rst())
            .field("txfifo_rst", &self.txfifo_rst())
            .field("rxd_inv", &self.rxd_inv())
            .field("cts_inv", &self.cts_inv())
            .field("dsr_inv", &self.dsr_inv())
            .field("txd_inv", &self.txd_inv())
            .field("rts_inv", &self.rts_inv())
            .field("dtr_inv", &self.dtr_inv())
            .field("clk_en", &self.clk_en())
            .field("err_wr_mask", &self.err_wr_mask())
            .field("tick_ref_always_on", &self.tick_ref_always_on())
            .field("mem_clk_en", &self.mem_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to configure the parity check mode. 0: even. 1: odd."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W<CONF0_SPEC> {
        PARITY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable UART parity check."]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W<CONF0_SPEC> {
        PARITY_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - This register is used to set the length of data. 0: 5 bits. 1: 6 bits. 2: 7 bits. 3: 8 bits."]
    #[inline(always)]
    pub fn bit_num(&mut self) -> BIT_NUM_W<CONF0_SPEC> {
        BIT_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - This register is used to set the length of stop bit. 1: 1 bit. 2: 1.5 bits. 3: 2 bits."]
    #[inline(always)]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W<CONF0_SPEC> {
        STOP_BIT_NUM_W::new(self, 4)
    }
    #[doc = "Bit 6 - This register is used to configure the software RTS signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_rts(&mut self) -> SW_RTS_W<CONF0_SPEC> {
        SW_RTS_W::new(self, 6)
    }
    #[doc = "Bit 7 - This register is used to configure the software DTR signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_dtr(&mut self) -> SW_DTR_W<CONF0_SPEC> {
        SW_DTR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable the transmitter to send NULL characters when the process of sending data is done."]
    #[inline(always)]
    pub fn txd_brk(&mut self) -> TXD_BRK_W<CONF0_SPEC> {
        TXD_BRK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable IrDA loopback mode."]
    #[inline(always)]
    pub fn irda_dplx(&mut self) -> IRDA_DPLX_W<CONF0_SPEC> {
        IRDA_DPLX_W::new(self, 9)
    }
    #[doc = "Bit 10 - This is the start enable bit for IrDA transmitter."]
    #[inline(always)]
    pub fn irda_tx_en(&mut self) -> IRDA_TX_EN_W<CONF0_SPEC> {
        IRDA_TX_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: The IrDA transmitter's 11th bit is the same as 10th bit. 0: Set IrDA transmitter's 11th bit to 0."]
    #[inline(always)]
    pub fn irda_wctl(&mut self) -> IRDA_WCTL_W<CONF0_SPEC> {
        IRDA_WCTL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to invert the level of IrDA transmitter."]
    #[inline(always)]
    pub fn irda_tx_inv(&mut self) -> IRDA_TX_INV_W<CONF0_SPEC> {
        IRDA_TX_INV_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to invert the level of IrDA receiver."]
    #[inline(always)]
    pub fn irda_rx_inv(&mut self) -> IRDA_RX_INV_W<CONF0_SPEC> {
        IRDA_RX_INV_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to enable UART loopback test mode."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W<CONF0_SPEC> {
        LOOPBACK_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to enable flow control function for the transmitter."]
    #[inline(always)]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W<CONF0_SPEC> {
        TX_FLOW_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to enable IrDA protocol."]
    #[inline(always)]
    pub fn irda_en(&mut self) -> IRDA_EN_W<CONF0_SPEC> {
        IRDA_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to reset the UART RX FIFO."]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W<CONF0_SPEC> {
        RXFIFO_RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to reset the UART TX FIFO."]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W<CONF0_SPEC> {
        TXFIFO_RST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to invert the level of UART RXD signal."]
    #[inline(always)]
    pub fn rxd_inv(&mut self) -> RXD_INV_W<CONF0_SPEC> {
        RXD_INV_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set this bit to invert the level of UART CTS signal."]
    #[inline(always)]
    pub fn cts_inv(&mut self) -> CTS_INV_W<CONF0_SPEC> {
        CTS_INV_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to invert the level of UART DSR signal."]
    #[inline(always)]
    pub fn dsr_inv(&mut self) -> DSR_INV_W<CONF0_SPEC> {
        DSR_INV_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to invert the level of UART TXD signal."]
    #[inline(always)]
    pub fn txd_inv(&mut self) -> TXD_INV_W<CONF0_SPEC> {
        TXD_INV_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to invert the level of UART RTS signal."]
    #[inline(always)]
    pub fn rts_inv(&mut self) -> RTS_INV_W<CONF0_SPEC> {
        RTS_INV_W::new(self, 23)
    }
    #[doc = "Bit 24 - Set this bit to invert the level of UART DTR signal."]
    #[inline(always)]
    pub fn dtr_inv(&mut self) -> DTR_INV_W<CONF0_SPEC> {
        DTR_INV_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Force clock on for registers. 0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF0_SPEC> {
        CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: The receiver stops storing data into FIFO when data is wrong. 0: The receiver stores the data even if the received data is wrong."]
    #[inline(always)]
    pub fn err_wr_mask(&mut self) -> ERR_WR_MASK_W<CONF0_SPEC> {
        ERR_WR_MASK_W::new(self, 26)
    }
    #[doc = "Bit 27 - This register is used to select the clock. 1: APB_CLK. 0: REF_TICK."]
    #[inline(always)]
    pub fn tick_ref_always_on(&mut self) -> TICK_REF_ALWAYS_ON_W<CONF0_SPEC> {
        TICK_REF_ALWAYS_ON_W::new(self, 27)
    }
    #[doc = "Bit 28 - The signal to enable UART RAM clock gating. 1: UART RAM powers on, the data of which can be read and written. 0: UART RAM powers down."]
    #[inline(always)]
    pub fn mem_clk_en(&mut self) -> MEM_CLK_EN_W<CONF0_SPEC> {
        MEM_CLK_EN_W::new(self, 28)
    }
}
#[doc = "Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF0 to value 0x1800_001c"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x1800_001c;
}
