#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_6` reader"]
pub type R = crate::R<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_6` writer"]
pub type W = crate::W<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_3` reader - Region 2 end address and Region 3 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_3` writer - Region 2 end address and Region 3 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Region 2 end address and Region 3 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_3(&self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_R {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_REGION_PMS_CONSTRAIN_6")
            .field(
                "core_1_region_pms_constrain_addr_3",
                &self.core_1_region_pms_constrain_addr_3(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 2 end address and Region 3 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_3(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_W<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC> {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_W::new(self, 0)
    }
}
#[doc = "core1 region permission register 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_region_pms_constrain_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_REGION_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_region_pms_constrain_6::R`](R) reader structure"]
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_region_pms_constrain_6::W`](W) writer structure"]
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_6 to value 0"]
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {}
