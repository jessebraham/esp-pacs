#[doc = "Register `DMMU_TABLE11` reader"]
pub type R = crate::R<DMMU_TABLE11_SPEC>;
#[doc = "Register `DMMU_TABLE11` writer"]
pub type W = crate::W<DMMU_TABLE11_SPEC>;
#[doc = "Field `DMMU_TABLE11` reader - "]
pub type DMMU_TABLE11_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE11` writer - "]
pub type DMMU_TABLE11_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table11(&self) -> DMMU_TABLE11_R {
        DMMU_TABLE11_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE11")
            .field("dmmu_table11", &self.dmmu_table11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table11(&mut self) -> DMMU_TABLE11_W<DMMU_TABLE11_SPEC> {
        DMMU_TABLE11_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmmu_table11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmmu_table11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE11_SPEC;
impl crate::RegisterSpec for DMMU_TABLE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table11::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table11::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE11_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMMU_TABLE11 to value 0x0b"]
impl crate::Resettable for DMMU_TABLE11_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}
