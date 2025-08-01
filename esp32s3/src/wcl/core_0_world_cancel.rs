#[doc = "Register `Core_0_World_Cancel` writer"]
pub type W = crate::W<CORE_0_WORLD_CANCEL_SPEC>;
#[doc = "Field `CORE_0_WORLD_CANCEL` writer - This field is used to cancel switch world configuration,if the trigger address and update configuration complete,use this register to cancel world switch, jujst need write any value,the hardware only checks the write operation of this register and does not case about its value"]
pub type CORE_0_WORLD_CANCEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_WORLD_CANCEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to cancel switch world configuration,if the trigger address and update configuration complete,use this register to cancel world switch, jujst need write any value,the hardware only checks the write operation of this register and does not case about its value"]
    #[inline(always)]
    pub fn core_0_world_cancel(&mut self) -> CORE_0_WORLD_CANCEL_W<CORE_0_WORLD_CANCEL_SPEC> {
        CORE_0_WORLD_CANCEL_W::new(self, 0)
    }
}
#[doc = "Core_0 configuration cancel register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_world_cancel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_WORLD_CANCEL_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_CANCEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_0_world_cancel::W`](W) writer structure"]
impl crate::Writable for CORE_0_WORLD_CANCEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_0_World_Cancel to value 0"]
impl crate::Resettable for CORE_0_WORLD_CANCEL_SPEC {}
