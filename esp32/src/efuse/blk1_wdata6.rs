#[doc = "Register `BLK1_WDATA6` reader"]
pub type R = crate::R<BLK1_WDATA6_SPEC>;
#[doc = "Register `BLK1_WDATA6` writer"]
pub type W = crate::W<BLK1_WDATA6_SPEC>;
#[doc = "Field `BLK1_DIN6` reader - "]
pub type BLK1_DIN6_R = crate::FieldReader<u32>;
#[doc = "Field `BLK1_DIN6` writer - "]
pub type BLK1_DIN6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk1_din6(&self) -> BLK1_DIN6_R {
        BLK1_DIN6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_WDATA6")
            .field("blk1_din6", &self.blk1_din6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk1_din6(&mut self) -> BLK1_DIN6_W<BLK1_WDATA6_SPEC> {
        BLK1_DIN6_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`blk1_wdata6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk1_wdata6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK1_WDATA6_SPEC;
impl crate::RegisterSpec for BLK1_WDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk1_wdata6::R`](R) reader structure"]
impl crate::Readable for BLK1_WDATA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk1_wdata6::W`](W) writer structure"]
impl crate::Writable for BLK1_WDATA6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK1_WDATA6 to value 0"]
impl crate::Resettable for BLK1_WDATA6_SPEC {}
