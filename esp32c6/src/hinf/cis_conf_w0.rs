#[doc = "Register `CIS_CONF_W0` reader"]
pub type R = crate::R<CIS_CONF_W0_SPEC>;
#[doc = "Register `CIS_CONF_W0` writer"]
pub type W = crate::W<CIS_CONF_W0_SPEC>;
#[doc = "Field `CIS_CONF_W0` reader - Configure cis addr 39~36"]
pub type CIS_CONF_W0_R = crate::FieldReader<u32>;
#[doc = "Field `CIS_CONF_W0` writer - Configure cis addr 39~36"]
pub type CIS_CONF_W0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure cis addr 39~36"]
    #[inline(always)]
    pub fn cis_conf_w0(&self) -> CIS_CONF_W0_R {
        CIS_CONF_W0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIS_CONF_W0")
            .field("cis_conf_w0", &self.cis_conf_w0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure cis addr 39~36"]
    #[inline(always)]
    pub fn cis_conf_w0(&mut self) -> CIS_CONF_W0_W<CIS_CONF_W0_SPEC> {
        CIS_CONF_W0_W::new(self, 0)
    }
}
#[doc = "SDIO cis configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cis_conf_w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_conf_w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIS_CONF_W0_SPEC;
impl crate::RegisterSpec for CIS_CONF_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cis_conf_w0::R`](R) reader structure"]
impl crate::Readable for CIS_CONF_W0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cis_conf_w0::W`](W) writer structure"]
impl crate::Writable for CIS_CONF_W0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIS_CONF_W0 to value 0xffff_ffff"]
impl crate::Resettable for CIS_CONF_W0_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
