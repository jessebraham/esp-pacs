#[doc = "Register `LPBUS` reader"]
pub type R = crate::R<LPBUS_SPEC>;
#[doc = "Register `LPBUS` writer"]
pub type W = crate::W<LPBUS_SPEC>;
#[doc = "Field `FAST_MEM_WPULSE` reader - This field controls fast memory WPULSE parameter."]
pub type FAST_MEM_WPULSE_R = crate::FieldReader;
#[doc = "Field `FAST_MEM_WPULSE` writer - This field controls fast memory WPULSE parameter."]
pub type FAST_MEM_WPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST_MEM_WA` reader - This field controls fast memory WA parameter."]
pub type FAST_MEM_WA_R = crate::FieldReader;
#[doc = "Field `FAST_MEM_WA` writer - This field controls fast memory WA parameter."]
pub type FAST_MEM_WA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST_MEM_RA` reader - This field controls fast memory RA parameter."]
pub type FAST_MEM_RA_R = crate::FieldReader;
#[doc = "Field `FAST_MEM_RA` writer - This field controls fast memory RA parameter."]
pub type FAST_MEM_RA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 16:18 - This field controls fast memory WPULSE parameter."]
    #[inline(always)]
    pub fn fast_mem_wpulse(&self) -> FAST_MEM_WPULSE_R {
        FAST_MEM_WPULSE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - This field controls fast memory WA parameter."]
    #[inline(always)]
    pub fn fast_mem_wa(&self) -> FAST_MEM_WA_R {
        FAST_MEM_WA_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - This field controls fast memory RA parameter."]
    #[inline(always)]
    pub fn fast_mem_ra(&self) -> FAST_MEM_RA_R {
        FAST_MEM_RA_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPBUS")
            .field("fast_mem_wpulse", &self.fast_mem_wpulse())
            .field("fast_mem_wa", &self.fast_mem_wa())
            .field("fast_mem_ra", &self.fast_mem_ra())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:18 - This field controls fast memory WPULSE parameter."]
    #[inline(always)]
    pub fn fast_mem_wpulse(&mut self) -> FAST_MEM_WPULSE_W<LPBUS_SPEC> {
        FAST_MEM_WPULSE_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - This field controls fast memory WA parameter."]
    #[inline(always)]
    pub fn fast_mem_wa(&mut self) -> FAST_MEM_WA_W<LPBUS_SPEC> {
        FAST_MEM_WA_W::new(self, 19)
    }
    #[doc = "Bits 22:23 - This field controls fast memory RA parameter."]
    #[inline(always)]
    pub fn fast_mem_ra(&mut self) -> FAST_MEM_RA_W<LPBUS_SPEC> {
        FAST_MEM_RA_W::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPBUS_SPEC;
impl crate::RegisterSpec for LPBUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbus::R`](R) reader structure"]
impl crate::Readable for LPBUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpbus::W`](W) writer structure"]
impl crate::Writable for LPBUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPBUS to value 0x0020_0000"]
impl crate::Resettable for LPBUS_SPEC {
    const RESET_VALUE: u32 = 0x0020_0000;
}
