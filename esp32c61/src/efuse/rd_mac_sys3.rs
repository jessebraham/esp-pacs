#[doc = "Register `RD_MAC_SYS3` reader"]
pub type R = crate::R<RD_MAC_SYS3_SPEC>;
#[doc = "Field `MAC_RESERVED_2` reader - Reserved."]
pub type MAC_RESERVED_2_R = crate::FieldReader<u32>;
#[doc = "Field `SYS_DATA_PART0_0` reader - Represents the first 14-bit of zeroth part of system data."]
pub type SYS_DATA_PART0_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:17 - Reserved."]
    #[inline(always)]
    pub fn mac_reserved_2(&self) -> MAC_RESERVED_2_R {
        MAC_RESERVED_2_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:31 - Represents the first 14-bit of zeroth part of system data."]
    #[inline(always)]
    pub fn sys_data_part0_0(&self) -> SYS_DATA_PART0_0_R {
        SYS_DATA_PART0_0_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS3")
            .field("mac_reserved_2", &self.mac_reserved_2())
            .field("sys_data_part0_0", &self.sys_data_part0_0())
            .finish()
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS3_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys3::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS3_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS3 to value 0"]
impl crate::Resettable for RD_MAC_SYS3_SPEC {}
