#[doc = "Register `MEM_RX_STATUS` reader"]
pub type R = crate::R<MEM_RX_STATUS_SPEC>;
#[doc = "Field `RX_SRAM_RADDR` reader - Represents the offset address to read RX FIFO."]
pub type RX_SRAM_RADDR_R = crate::FieldReader;
#[doc = "Field `RX_SRAM_WADDR` reader - Represents the offset address to write RX FIFO."]
pub type RX_SRAM_WADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 3:7 - Represents the offset address to read RX FIFO."]
    #[inline(always)]
    pub fn rx_sram_raddr(&self) -> RX_SRAM_RADDR_R {
        RX_SRAM_RADDR_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - Represents the offset address to write RX FIFO."]
    #[inline(always)]
    pub fn rx_sram_waddr(&self) -> RX_SRAM_WADDR_R {
        RX_SRAM_WADDR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_RX_STATUS")
            .field("rx_sram_raddr", &self.rx_sram_raddr())
            .field("rx_sram_waddr", &self.rx_sram_waddr())
            .finish()
    }
}
#[doc = "RX FIFO write and read offset address\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_RX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_rx_status::R`](R) reader structure"]
impl crate::Readable for MEM_RX_STATUS_SPEC {}
#[doc = "`reset()` method sets MEM_RX_STATUS to value 0x0001_0080"]
impl crate::Resettable for MEM_RX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0001_0080;
}
