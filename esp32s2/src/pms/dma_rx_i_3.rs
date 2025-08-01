#[doc = "Register `DMA_RX_I_3` reader"]
pub type R = crate::R<DMA_RX_I_3_SPEC>;
#[doc = "Field `DMA_RX_I_ILG_ST` reader - Record the illegitimate information of RX Copy DMA. \\[22:6\\]: store the bits \\[18:2\\] of address. \\[5\\]: if bits \\[31:19\\] of address are 0x7ff, then the bit value is 1, otherwise it is 0. \\[4\\]: 1 means write operation, 0 means read operation. \\[3:0\\]: RX Copy DMA bus byte enables."]
pub type DMA_RX_I_ILG_ST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:22 - Record the illegitimate information of RX Copy DMA. \\[22:6\\]: store the bits \\[18:2\\] of address. \\[5\\]: if bits \\[31:19\\] of address are 0x7ff, then the bit value is 1, otherwise it is 0. \\[4\\]: 1 means write operation, 0 means read operation. \\[3:0\\]: RX Copy DMA bus byte enables."]
    #[inline(always)]
    pub fn dma_rx_i_ilg_st(&self) -> DMA_RX_I_ILG_ST_R {
        DMA_RX_I_ILG_ST_R::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RX_I_3")
            .field("dma_rx_i_ilg_st", &self.dma_rx_i_ilg_st())
            .finish()
    }
}
#[doc = "RX Copy DMA status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rx_i_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_RX_I_3_SPEC;
impl crate::RegisterSpec for DMA_RX_I_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rx_i_3::R`](R) reader structure"]
impl crate::Readable for DMA_RX_I_3_SPEC {}
#[doc = "`reset()` method sets DMA_RX_I_3 to value 0"]
impl crate::Resettable for DMA_RX_I_3_SPEC {}
