#[doc = "Register `SAR_TOUCH_THRES1` reader"]
pub type R = crate::R<SAR_TOUCH_THRES1_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES1` writer"]
pub type W = crate::W<SAR_TOUCH_THRES1_SPEC>;
#[doc = "Field `TOUCH_OUT_TH1` reader - Finger threshold for touch pad 1"]
pub type TOUCH_OUT_TH1_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_OUT_TH1` writer - Finger threshold for touch pad 1"]
pub type TOUCH_OUT_TH1_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 1"]
    #[inline(always)]
    pub fn touch_out_th1(&self) -> TOUCH_OUT_TH1_R {
        TOUCH_OUT_TH1_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES1")
            .field("touch_out_th1", &self.touch_out_th1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 1"]
    #[inline(always)]
    pub fn touch_out_th1(&mut self) -> TOUCH_OUT_TH1_W<SAR_TOUCH_THRES1_SPEC> {
        TOUCH_OUT_TH1_W::new(self, 0)
    }
}
#[doc = "Finger threshold for touch pad 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres1::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres1::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES1 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES1_SPEC {}
