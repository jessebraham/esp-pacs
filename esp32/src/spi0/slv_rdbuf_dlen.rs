#[doc = "Register `SLV_RDBUF_DLEN` reader"]
pub type R = crate::R<SLV_RDBUF_DLEN_SPEC>;
#[doc = "Register `SLV_RDBUF_DLEN` writer"]
pub type W = crate::W<SLV_RDBUF_DLEN_SPEC>;
#[doc = "Field `SLV_RDBUF_DBITLEN` reader - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
pub type SLV_RDBUF_DBITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_RDBUF_DBITLEN` writer - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
pub type SLV_RDBUF_DBITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dbitlen(&self) -> SLV_RDBUF_DBITLEN_R {
        SLV_RDBUF_DBITLEN_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_RDBUF_DLEN")
            .field("slv_rdbuf_dbitlen", &self.slv_rdbuf_dbitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dbitlen(&mut self) -> SLV_RDBUF_DBITLEN_W<SLV_RDBUF_DLEN_SPEC> {
        SLV_RDBUF_DBITLEN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slv_rdbuf_dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_rdbuf_dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_RDBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_RDBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_rdbuf_dlen::R`](R) reader structure"]
impl crate::Readable for SLV_RDBUF_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_rdbuf_dlen::W`](W) writer structure"]
impl crate::Writable for SLV_RDBUF_DLEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLV_RDBUF_DLEN to value 0"]
impl crate::Resettable for SLV_RDBUF_DLEN_SPEC {}
