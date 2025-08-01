#[doc = "Register `TIME_UPDATE` reader"]
pub type R = crate::R<TIME_UPDATE_SPEC>;
#[doc = "Register `TIME_UPDATE` writer"]
pub type W = crate::W<TIME_UPDATE_SPEC>;
#[doc = "Field `TIME_VALID` reader - To indicate the register is updated"]
pub type TIME_VALID_R = crate::BitReader;
#[doc = "Field `TIME_UPDATE` writer - Set 1: to update register with RTC timer"]
pub type TIME_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - To indicate the register is updated"]
    #[inline(always)]
    pub fn time_valid(&self) -> TIME_VALID_R {
        TIME_VALID_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_UPDATE")
            .field("time_valid", &self.time_valid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Set 1: to update register with RTC timer"]
    #[inline(always)]
    pub fn time_update(&mut self) -> TIME_UPDATE_W<TIME_UPDATE_SPEC> {
        TIME_UPDATE_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`time_update::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_update::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_UPDATE_SPEC;
impl crate::RegisterSpec for TIME_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_update::R`](R) reader structure"]
impl crate::Readable for TIME_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_update::W`](W) writer structure"]
impl crate::Writable for TIME_UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIME_UPDATE to value 0"]
impl crate::Resettable for TIME_UPDATE_SPEC {}
