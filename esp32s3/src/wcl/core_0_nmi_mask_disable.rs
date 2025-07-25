#[doc = "Register `Core_0_NMI_MASK_DISABLE` writer"]
pub type W = crate::W<CORE_0_NMI_MASK_DISABLE_SPEC>;
#[doc = "Field `CORE_0_NMI_MASK_DISABLE` writer - this field is used to disable NMI mask,it will not take effect immediately,only when the CPU executes to the trigger address will it start to cancel NMI mask"]
pub type CORE_0_NMI_MASK_DISABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_NMI_MASK_DISABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - this field is used to disable NMI mask,it will not take effect immediately,only when the CPU executes to the trigger address will it start to cancel NMI mask"]
    #[inline(always)]
    pub fn core_0_nmi_mask_disable(
        &mut self,
    ) -> CORE_0_NMI_MASK_DISABLE_W<CORE_0_NMI_MASK_DISABLE_SPEC> {
        CORE_0_NMI_MASK_DISABLE_W::new(self, 0)
    }
}
#[doc = "Core_0 NMI mask disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_nmi_mask_disable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_NMI_MASK_DISABLE_SPEC;
impl crate::RegisterSpec for CORE_0_NMI_MASK_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_0_nmi_mask_disable::W`](W) writer structure"]
impl crate::Writable for CORE_0_NMI_MASK_DISABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_0_NMI_MASK_DISABLE to value 0"]
impl crate::Resettable for CORE_0_NMI_MASK_DISABLE_SPEC {}
