#[doc = "Register `STORE3` reader"]
pub type R = crate::R<STORE3_SPEC>;
#[doc = "Register `STORE3` writer"]
pub type W = crate::W<STORE3_SPEC>;
#[doc = "Field `SCRATCH3` reader - Reserved register"]
pub type SCRATCH3_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH3` writer - Reserved register"]
pub type SCRATCH3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved register"]
    #[inline(always)]
    pub fn scratch3(&self) -> SCRATCH3_R {
        SCRATCH3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE3")
            .field("scratch3", &self.scratch3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved register"]
    #[inline(always)]
    pub fn scratch3(&mut self) -> SCRATCH3_W<STORE3_SPEC> {
        SCRATCH3_W::new(self, 0)
    }
}
#[doc = "Reserved register\n\nYou can [`read`](crate::Reg::read) this register and get [`store3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE3_SPEC;
impl crate::RegisterSpec for STORE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store3::R`](R) reader structure"]
impl crate::Readable for STORE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store3::W`](W) writer structure"]
impl crate::Writable for STORE3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STORE3 to value 0"]
impl crate::Resettable for STORE3_SPEC {}
