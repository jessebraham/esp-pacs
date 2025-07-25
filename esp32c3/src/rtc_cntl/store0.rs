#[doc = "Register `STORE0` reader"]
pub type R = crate::R<STORE0_SPEC>;
#[doc = "Register `STORE0` writer"]
pub type W = crate::W<STORE0_SPEC>;
#[doc = "Field `SCRATCH0` reader - reserved register"]
pub type SCRATCH0_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH0` writer - reserved register"]
pub type SCRATCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch0(&self) -> SCRATCH0_R {
        SCRATCH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE0")
            .field("scratch0", &self.scratch0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch0(&mut self) -> SCRATCH0_W<STORE0_SPEC> {
        SCRATCH0_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`store0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE0_SPEC;
impl crate::RegisterSpec for STORE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store0::R`](R) reader structure"]
impl crate::Readable for STORE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store0::W`](W) writer structure"]
impl crate::Writable for STORE0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STORE0 to value 0"]
impl crate::Resettable for STORE0_SPEC {}
