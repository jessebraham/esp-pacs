#[doc = "Register `QAY_MEM[%s]` reader"]
pub type R = crate::R<QAY_MEM_SPEC>;
#[doc = "Register `QAY_MEM[%s]` writer"]
pub type W = crate::W<QAY_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores y coordinates of QA.\n\nYou can [`read`](crate::Reg::read) this register and get [`qay_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qay_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QAY_MEM_SPEC;
impl crate::RegisterSpec for QAY_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qay_mem::R`](R) reader structure"]
impl crate::Readable for QAY_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qay_mem::W`](W) writer structure"]
impl crate::Writable for QAY_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QAY_MEM[%s] to value 0"]
impl crate::Resettable for QAY_MEM_SPEC {}
