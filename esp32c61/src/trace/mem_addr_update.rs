#[doc = "Register `MEM_ADDR_UPDATE` writer"]
pub type W = crate::W<MEM_ADDR_UPDATE_SPEC>;
#[doc = "Field `MEM_CURRENT_ADDR_UPDATE` writer - Configures whether to update the value of \\hyperref\\[fielddesc:TRACEMEMCURRENTADDR\\]{TRACE_MEM_CURRENT_ADDR} to \\hyperref\\[fielddesc:TRACEMEMSTARTADDR\\]{TRACE_MEM_START_ADDR}.\\\\ 0: Not update\\\\ 1: Update\\\\"]
pub type MEM_CURRENT_ADDR_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_ADDR_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to update the value of \\hyperref\\[fielddesc:TRACEMEMCURRENTADDR\\]{TRACE_MEM_CURRENT_ADDR} to \\hyperref\\[fielddesc:TRACEMEMSTARTADDR\\]{TRACE_MEM_START_ADDR}.\\\\ 0: Not update\\\\ 1: Update\\\\"]
    #[inline(always)]
    pub fn mem_current_addr_update(&mut self) -> MEM_CURRENT_ADDR_UPDATE_W<MEM_ADDR_UPDATE_SPEC> {
        MEM_CURRENT_ADDR_UPDATE_W::new(self, 0)
    }
}
#[doc = "Memory address update\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_addr_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_ADDR_UPDATE_SPEC;
impl crate::RegisterSpec for MEM_ADDR_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mem_addr_update::W`](W) writer structure"]
impl crate::Writable for MEM_ADDR_UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_ADDR_UPDATE to value 0"]
impl crate::Resettable for MEM_ADDR_UPDATE_SPEC {}
