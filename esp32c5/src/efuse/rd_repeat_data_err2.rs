#[doc = "Register `RD_REPEAT_DATA_ERR2` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR2_SPEC>;
#[doc = "Field `KEY_PURPOSE_2_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_2"]
pub type KEY_PURPOSE_2_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_3"]
pub type KEY_PURPOSE_3_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_4"]
pub type KEY_PURPOSE_4_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_5"]
pub type KEY_PURPOSE_5_ERR_R = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL_ERR` reader - Represents the programming error of EFUSE_SEC_DPA_LEVEL"]
pub type SEC_DPA_LEVEL_ERR_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_EN"]
pub type SECURE_BOOT_EN_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE"]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R = crate::BitReader;
#[doc = "Field `KM_XTS_KEY_LENGTH_256_ERR` reader - Represents the programming error of EFUSE_KM_XTS_KEY_LENGTH_256"]
pub type KM_XTS_KEY_LENGTH_256_ERR_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW_ERR` reader - Represents the programming error of EFUSE_FLASH_TPUW"]
pub type FLASH_TPUW_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents the programming error of EFUSE_KEY_PURPOSE_2"]
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> KEY_PURPOSE_2_ERR_R {
        KEY_PURPOSE_2_ERR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Represents the programming error of EFUSE_KEY_PURPOSE_3"]
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> KEY_PURPOSE_3_ERR_R {
        KEY_PURPOSE_3_ERR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Represents the programming error of EFUSE_KEY_PURPOSE_4"]
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> KEY_PURPOSE_4_ERR_R {
        KEY_PURPOSE_4_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Represents the programming error of EFUSE_KEY_PURPOSE_5"]
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> KEY_PURPOSE_5_ERR_R {
        KEY_PURPOSE_5_ERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents the programming error of EFUSE_SEC_DPA_LEVEL"]
    #[inline(always)]
    pub fn sec_dpa_level_err(&self) -> SEC_DPA_LEVEL_ERR_R {
        SEC_DPA_LEVEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Represents the programming error of EFUSE_SECURE_BOOT_EN"]
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SECURE_BOOT_EN_ERR_R {
        SECURE_BOOT_EN_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents the programming error of EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents the programming error of EFUSE_KM_XTS_KEY_LENGTH_256"]
    #[inline(always)]
    pub fn km_xts_key_length_256_err(&self) -> KM_XTS_KEY_LENGTH_256_ERR_R {
        KM_XTS_KEY_LENGTH_256_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Represents the programming error of EFUSE_FLASH_TPUW"]
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FLASH_TPUW_ERR_R {
        FLASH_TPUW_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR2")
            .field("key_purpose_2_err", &self.key_purpose_2_err())
            .field("key_purpose_3_err", &self.key_purpose_3_err())
            .field("key_purpose_4_err", &self.key_purpose_4_err())
            .field("key_purpose_5_err", &self.key_purpose_5_err())
            .field("sec_dpa_level_err", &self.sec_dpa_level_err())
            .field("secure_boot_en_err", &self.secure_boot_en_err())
            .field(
                "secure_boot_aggressive_revoke_err",
                &self.secure_boot_aggressive_revoke_err(),
            )
            .field(
                "km_xts_key_length_256_err",
                &self.km_xts_key_length_256_err(),
            )
            .field("flash_tpuw_err", &self.flash_tpuw_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR2_SPEC {}
