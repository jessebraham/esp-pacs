#[doc = "Register `STORE5` reader"]
pub type R = crate::R<STORE5_SPEC>;
#[doc = "Register `STORE5` writer"]
pub type W = crate::W<STORE5_SPEC>;
#[doc = "Field `SCRATCH5` reader - reserved register"]
pub type SCRATCH5_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH5` writer - reserved register"]
pub type SCRATCH5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch5(&self) -> SCRATCH5_R {
        SCRATCH5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE5")
            .field("scratch5", &self.scratch5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved register"]
    #[inline(always)]
    pub fn scratch5(&mut self) -> SCRATCH5_W<STORE5_SPEC> {
        SCRATCH5_W::new(self, 0)
    }
}
#[doc = "reserved register\n\nYou can [`read`](crate::Reg::read) this register and get [`store5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE5_SPEC;
impl crate::RegisterSpec for STORE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store5::R`](R) reader structure"]
impl crate::Readable for STORE5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store5::W`](W) writer structure"]
impl crate::Writable for STORE5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STORE5 to value 0"]
impl crate::Resettable for STORE5_SPEC {}
