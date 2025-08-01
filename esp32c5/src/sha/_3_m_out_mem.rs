#[doc = "Register `_3_M_OUT_MEM[%s]` reader"]
pub type R = crate::R<_3_M_OUT_MEM_SPEC>;
#[doc = "Register `_3_M_OUT_MEM[%s]` writer"]
pub type W = crate::W<_3_M_OUT_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Sha3 hash reg which contains intermediate hash or finial hash.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_m_out_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_m_out_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_M_OUT_MEM_SPEC;
impl crate::RegisterSpec for _3_M_OUT_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`_3_m_out_mem::R`](R) reader structure"]
impl crate::Readable for _3_M_OUT_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_m_out_mem::W`](W) writer structure"]
impl crate::Writable for _3_M_OUT_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_M_OUT_MEM[%s] to value 0"]
impl crate::Resettable for _3_M_OUT_MEM_SPEC {}
