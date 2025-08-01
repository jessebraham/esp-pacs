#[doc = "Register `MEM_RX_STATUS` reader"]
pub type R = crate::R<MEM_RX_STATUS_SPEC>;
#[doc = "Field `APB_RX_RADDR` reader - This register stores the offset address in RX_FIFO when software reads data from RX FIFO via APB."]
pub type APB_RX_RADDR_R = crate::FieldReader<u16>;
#[doc = "Field `RX_WADDR` reader - This register stores the offset address in RX FIFO when Rx_FIFO_Ctrl writes RX FIFO."]
pub type RX_WADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - This register stores the offset address in RX_FIFO when software reads data from RX FIFO via APB."]
    #[inline(always)]
    pub fn apb_rx_raddr(&self) -> APB_RX_RADDR_R {
        APB_RX_RADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - This register stores the offset address in RX FIFO when Rx_FIFO_Ctrl writes RX FIFO."]
    #[inline(always)]
    pub fn rx_waddr(&self) -> RX_WADDR_R {
        RX_WADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_RX_STATUS")
            .field("apb_rx_raddr", &self.apb_rx_raddr())
            .field("rx_waddr", &self.rx_waddr())
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
#[doc = "`reset()` method sets MEM_RX_STATUS to value 0"]
impl crate::Resettable for MEM_RX_STATUS_SPEC {}
