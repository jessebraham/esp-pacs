#[doc = "Register `DATA_6` reader"]
pub type R = crate::R<DATA_6_SPEC>;
#[doc = "Register `DATA_6` writer"]
pub type W = crate::W<DATA_6_SPEC>;
#[doc = "Field `DATA_6` reader - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
pub type DATA_6_R = crate::FieldReader;
#[doc = "Field `DATA_6` writer - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
pub type DATA_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
    #[inline(always)]
    pub fn data_6(&self) -> DATA_6_R {
        DATA_6_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_6")
            .field("data_6", &self.data_6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
    #[inline(always)]
    pub fn data_6(&mut self) -> DATA_6_W<DATA_6_SPEC> {
        DATA_6_W::new(self, 0)
    }
}
#[doc = "Data register 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`data_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_6_SPEC;
impl crate::RegisterSpec for DATA_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_6::R`](R) reader structure"]
impl crate::Readable for DATA_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_6::W`](W) writer structure"]
impl crate::Writable for DATA_6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_6 to value 0"]
impl crate::Resettable for DATA_6_SPEC {}
