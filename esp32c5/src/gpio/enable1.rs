#[doc = "Register `ENABLE1` reader"]
pub type R = crate::R<ENABLE1_SPEC>;
#[doc = "Register `ENABLE1` writer"]
pub type W = crate::W<ENABLE1_SPEC>;
#[doc = "Field `DATA` reader - Configures whether or not to enable the output of GPIO32 ~ GPIO32.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type DATA_R = crate::BitReader;
#[doc = "Field `DATA` writer - Configures whether or not to enable the output of GPIO32 ~ GPIO32.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable the output of GPIO32 ~ GPIO32.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable the output of GPIO32 ~ GPIO32.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<ENABLE1_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "GPIO output enable register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`enable1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE1_SPEC;
impl crate::RegisterSpec for ENABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable1::R`](R) reader structure"]
impl crate::Readable for ENABLE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable1::W`](W) writer structure"]
impl crate::Writable for ENABLE1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE1 to value 0"]
impl crate::Resettable for ENABLE1_SPEC {}
