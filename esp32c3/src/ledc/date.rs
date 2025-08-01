#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `LEDC_DATE` reader - reg_ledc_date."]
pub type LEDC_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `LEDC_DATE` writer - reg_ledc_date."]
pub type LEDC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_ledc_date."]
    #[inline(always)]
    pub fn ledc_date(&self) -> LEDC_DATE_R {
        LEDC_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("ledc_date", &self.ledc_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_ledc_date."]
    #[inline(always)]
    pub fn ledc_date(&mut self) -> LEDC_DATE_W<DATE_SPEC> {
        LEDC_DATE_W::new(self, 0)
    }
}
#[doc = "LEDC_DATE.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0x1906_1700"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x1906_1700;
}
