#[doc = "Register `STORE2` reader"]
pub type R = crate::R<STORE2_SPEC>;
#[doc = "Register `STORE2` writer"]
pub type W = crate::W<STORE2_SPEC>;
#[doc = "Field `SCRATCH2` reader - 32-bit general purpose retention register"]
pub type SCRATCH2_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH2` writer - 32-bit general purpose retention register"]
pub type SCRATCH2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch2(&self) -> SCRATCH2_R {
        SCRATCH2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE2")
            .field("scratch2", &self.scratch2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch2(&mut self) -> SCRATCH2_W<STORE2_SPEC> {
        SCRATCH2_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`store2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE2_SPEC;
impl crate::RegisterSpec for STORE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store2::R`](R) reader structure"]
impl crate::Readable for STORE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store2::W`](W) writer structure"]
impl crate::Writable for STORE2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STORE2 to value 0"]
impl crate::Resettable for STORE2_SPEC {}
