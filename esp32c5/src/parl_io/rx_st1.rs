#[doc = "Register `RX_ST1` reader"]
pub type R = crate::R<RX_ST1_SPEC>;
#[doc = "Field `RX_FIFO_RD_BIT_CNT` reader - Indicates the current read bit number from Rx FIFO."]
pub type RX_FIFO_RD_BIT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 13:31 - Indicates the current read bit number from Rx FIFO."]
    #[inline(always)]
    pub fn rx_fifo_rd_bit_cnt(&self) -> RX_FIFO_RD_BIT_CNT_R {
        RX_FIFO_RD_BIT_CNT_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ST1")
            .field("rx_fifo_rd_bit_cnt", &self.rx_fifo_rd_bit_cnt())
            .finish()
    }
}
#[doc = "Parallel IO RX status register1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_st1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ST1_SPEC;
impl crate::RegisterSpec for RX_ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_st1::R`](R) reader structure"]
impl crate::Readable for RX_ST1_SPEC {}
#[doc = "`reset()` method sets RX_ST1 to value 0"]
impl crate::Resettable for RX_ST1_SPEC {}
