#[doc = "Register `STORE7` reader"]
pub type R = crate::R<STORE7_SPEC>;
#[doc = "Register `STORE7` writer"]
pub type W = crate::W<STORE7_SPEC>;
#[doc = "Field `SCRATCH7` reader - 32-bit general purpose retention register"]
pub type SCRATCH7_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH7` writer - 32-bit general purpose retention register"]
pub type SCRATCH7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch7(&self) -> SCRATCH7_R {
        SCRATCH7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE7")
            .field("scratch7", &self.scratch7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch7(&mut self) -> SCRATCH7_W<STORE7_SPEC> {
        SCRATCH7_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`store7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE7_SPEC;
impl crate::RegisterSpec for STORE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store7::R`](R) reader structure"]
impl crate::Readable for STORE7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store7::W`](W) writer structure"]
impl crate::Writable for STORE7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STORE7 to value 0"]
impl crate::Resettable for STORE7_SPEC {}
