#[doc = "Register `SHROM_MPU_TABLE3` reader"]
pub type R = crate::R<SHROM_MPU_TABLE3_SPEC>;
#[doc = "Register `SHROM_MPU_TABLE3` writer"]
pub type W = crate::W<SHROM_MPU_TABLE3_SPEC>;
#[doc = "Field `SHROM_MPU_TABLE3` reader - "]
pub type SHROM_MPU_TABLE3_R = crate::FieldReader;
#[doc = "Field `SHROM_MPU_TABLE3` writer - "]
pub type SHROM_MPU_TABLE3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table3(&self) -> SHROM_MPU_TABLE3_R {
        SHROM_MPU_TABLE3_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHROM_MPU_TABLE3")
            .field("shrom_mpu_table3", &self.shrom_mpu_table3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table3(&mut self) -> SHROM_MPU_TABLE3_W<SHROM_MPU_TABLE3_SPEC> {
        SHROM_MPU_TABLE3_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`shrom_mpu_table3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrom_mpu_table3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHROM_MPU_TABLE3_SPEC;
impl crate::RegisterSpec for SHROM_MPU_TABLE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shrom_mpu_table3::R`](R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shrom_mpu_table3::W`](W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHROM_MPU_TABLE3 to value 0x01"]
impl crate::Resettable for SHROM_MPU_TABLE3_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
