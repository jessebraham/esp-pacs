#[doc = "Register `IN_CONF0` reader"]
pub type R = crate::R<IN_CONF0_SPEC>;
#[doc = "Register `IN_CONF0` writer"]
pub type W = crate::W<IN_CONF0_SPEC>;
#[doc = "Field `IN_RST` reader - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST` reader - Reserved."]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Reserved."]
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN` reader - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DATA_BURST_EN` reader - Configures whether or not to enable INCR burst transfer for RX channel %s"]
pub type IN_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `IN_DATA_BURST_EN` writer - Configures whether or not to enable INCR burst transfer for RX channel %s"]
pub type IN_DATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN` reader - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type MEM_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_EN` reader - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type IN_ETM_EN_R = crate::BitReader;
#[doc = "Field `IN_ETM_EN` writer - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
pub type IN_ETM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DATA_BURST_MODE_SEL` reader - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type IN_DATA_BURST_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `IN_DATA_BURST_MODE_SEL` writer - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
pub type IN_DATA_BURST_MODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable INCR burst transfer for RX channel %s"]
    #[inline(always)]
    pub fn in_data_burst_en(&self) -> IN_DATA_BURST_EN_R {
        IN_DATA_BURST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn in_etm_en(&self) -> IN_ETM_EN_R {
        IN_ETM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn in_data_burst_mode_sel(&self) -> IN_DATA_BURST_MODE_SEL_R {
        IN_DATA_BURST_MODE_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0")
            .field("in_rst", &self.in_rst())
            .field("in_loop_test", &self.in_loop_test())
            .field("indscr_burst_en", &self.indscr_burst_en())
            .field("mem_trans_en", &self.mem_trans_en())
            .field("in_etm_en", &self.in_etm_en())
            .field("in_data_burst_mode_sel", &self.in_data_burst_mode_sel())
            .field("in_data_burst_en", &self.in_data_burst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 and then 0 to reset AHB_DMA channel 0 RX FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W<IN_CONF0_SPEC> {
        IN_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<IN_CONF0_SPEC> {
        IN_LOOP_TEST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable INCR burst transfer for RX channel %s to read descriptors.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<IN_CONF0_SPEC> {
        INDSCR_BURST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable INCR burst transfer for RX channel %s"]
    #[inline(always)]
    pub fn in_data_burst_en(&mut self) -> IN_DATA_BURST_EN_W<IN_CONF0_SPEC> {
        IN_DATA_BURST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable memory-to-memory data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<IN_CONF0_SPEC> {
        MEM_TRANS_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ETM control for RX channel%s.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn in_etm_en(&mut self) -> IN_ETM_EN_W<IN_CONF0_SPEC> {
        IN_ETM_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Configures max burst size for Rx channel%s.\\\\2'b00: single\\\\ 2'b01: incr4\\\\ 2'b10: incr8\\\\ 2'b11: incr16\\\\"]
    #[inline(always)]
    pub fn in_data_burst_mode_sel(&mut self) -> IN_DATA_BURST_MODE_SEL_W<IN_CONF0_SPEC> {
        IN_DATA_BURST_MODE_SEL_W::new(self, 6)
    }
}
#[doc = "Configuration register 0 of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_SPEC;
impl crate::RegisterSpec for IN_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CONF0 to value 0"]
impl crate::Resettable for IN_CONF0_SPEC {}
