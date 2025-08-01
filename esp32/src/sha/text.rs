#[doc = "Register `TEXT%s` reader"]
pub type R = crate::R<TEXT_SPEC>;
#[doc = "Register `TEXT%s` writer"]
pub type W = crate::W<TEXT_SPEC>;
#[doc = "Field `TEXT` reader - SHA Message block and hash result register."]
pub type TEXT_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT` writer - SHA Message block and hash result register."]
pub type TEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA Message block and hash result register."]
    #[inline(always)]
    pub fn text(&self) -> TEXT_R {
        TEXT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT").field("text", &self.text()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA Message block and hash result register."]
    #[inline(always)]
    pub fn text(&mut self) -> TEXT_W<TEXT_SPEC> {
        TEXT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`text::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEXT_SPEC;
impl crate::RegisterSpec for TEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text::R`](R) reader structure"]
impl crate::Readable for TEXT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`text::W`](W) writer structure"]
impl crate::Writable for TEXT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEXT%s to value 0"]
impl crate::Resettable for TEXT_SPEC {}
