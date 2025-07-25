#[doc = "Register `REG_DATE` reader"]
pub type R = crate::R<REG_DATE_SPEC>;
#[doc = "Register `REG_DATE` writer"]
pub type W = crate::W<REG_DATE_SPEC>;
#[doc = "Field `REG_DATE` reader - version register"]
pub type REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `REG_DATE` writer - version register"]
pub type REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - version register"]
    #[inline(always)]
    pub fn reg_date(&self) -> REG_DATE_R {
        REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_DATE")
            .field("reg_date", &self.reg_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - version register"]
    #[inline(always)]
    pub fn reg_date(&mut self) -> REG_DATE_W<REG_DATE_SPEC> {
        REG_DATE_W::new(self, 0)
    }
}
#[doc = "GPIO version register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_DATE_SPEC;
impl crate::RegisterSpec for REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_date::R`](R) reader structure"]
impl crate::Readable for REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_date::W`](W) writer structure"]
impl crate::Writable for REG_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DATE to value 0x0200_6130"]
impl crate::Resettable for REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0200_6130;
}
