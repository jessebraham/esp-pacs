#[doc = "Register `REMAINDER_BIT_NUM` reader"]
pub type R = crate::R<REMAINDER_BIT_NUM_SPEC>;
#[doc = "Register `REMAINDER_BIT_NUM` writer"]
pub type W = crate::W<REMAINDER_BIT_NUM_SPEC>;
#[doc = "Field `REMAINDER_BIT_NUM` reader - Stores the Remainder Bit Number for the GCM operation."]
pub type REMAINDER_BIT_NUM_R = crate::FieldReader;
#[doc = "Field `REMAINDER_BIT_NUM` writer - Stores the Remainder Bit Number for the GCM operation."]
pub type REMAINDER_BIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Stores the Remainder Bit Number for the GCM operation."]
    #[inline(always)]
    pub fn remainder_bit_num(&self) -> REMAINDER_BIT_NUM_R {
        REMAINDER_BIT_NUM_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAINDER_BIT_NUM")
            .field("remainder_bit_num", &self.remainder_bit_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Stores the Remainder Bit Number for the GCM operation."]
    #[inline(always)]
    pub fn remainder_bit_num(&mut self) -> REMAINDER_BIT_NUM_W<REMAINDER_BIT_NUM_SPEC> {
        REMAINDER_BIT_NUM_W::new(self, 0)
    }
}
#[doc = "Remainder bit number of plaintext/ciphertext\n\nYou can [`read`](crate::Reg::read) this register and get [`remainder_bit_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remainder_bit_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAINDER_BIT_NUM_SPEC;
impl crate::RegisterSpec for REMAINDER_BIT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remainder_bit_num::R`](R) reader structure"]
impl crate::Readable for REMAINDER_BIT_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remainder_bit_num::W`](W) writer structure"]
impl crate::Writable for REMAINDER_BIT_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAINDER_BIT_NUM to value 0"]
impl crate::Resettable for REMAINDER_BIT_NUM_SPEC {}
