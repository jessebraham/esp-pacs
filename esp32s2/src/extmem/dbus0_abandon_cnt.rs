#[doc = "Register `DBUS0_ABANDON_CNT` reader"]
pub type R = crate::R<DBUS0_ABANDON_CNT_SPEC>;
#[doc = "Field `DBUS0_ABANDON_CNT` reader - The bits are used to count the number of the abandoned dbus0 access."]
pub type DBUS0_ABANDON_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of the abandoned dbus0 access."]
    #[inline(always)]
    pub fn dbus0_abandon_cnt(&self) -> DBUS0_ABANDON_CNT_R {
        DBUS0_ABANDON_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS0_ABANDON_CNT")
            .field("dbus0_abandon_cnt", &self.dbus0_abandon_cnt())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_abandon_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS0_ABANDON_CNT_SPEC;
impl crate::RegisterSpec for DBUS0_ABANDON_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus0_abandon_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS0_ABANDON_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS0_ABANDON_CNT to value 0"]
impl crate::Resettable for DBUS0_ABANDON_CNT_SPEC {}
