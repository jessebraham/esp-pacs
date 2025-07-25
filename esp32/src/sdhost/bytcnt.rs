#[doc = "Register `BYTCNT` reader"]
pub type R = crate::R<BYTCNT_SPEC>;
#[doc = "Register `BYTCNT` writer"]
pub type W = crate::W<BYTCNT_SPEC>;
#[doc = "Field `BYTE_COUNT` reader - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
pub type BYTE_COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `BYTE_COUNT` writer - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
pub type BYTE_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BYTCNT")
            .field("byte_count", &self.byte_count())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
    #[inline(always)]
    pub fn byte_count(&mut self) -> BYTE_COUNT_W<BYTCNT_SPEC> {
        BYTE_COUNT_W::new(self, 0)
    }
}
#[doc = "Data transfer length configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bytcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bytcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BYTCNT_SPEC;
impl crate::RegisterSpec for BYTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bytcnt::R`](R) reader structure"]
impl crate::Readable for BYTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bytcnt::W`](W) writer structure"]
impl crate::Writable for BYTCNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BYTCNT to value 0x0200"]
impl crate::Resettable for BYTCNT_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
