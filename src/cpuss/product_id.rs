#[doc = "Register `PRODUCT_ID` reader"]
pub type R = crate::R<PRODUCT_ID_SPEC>;
#[doc = "Field `FAMILY_ID` reader - Family ID a.k.a. Partnumber a.k.a. Silicon ID"]
pub type FAMILY_ID_R = crate::FieldReader<u16>;
#[doc = "Field `MAJOR_REV` reader - Major Revision, starts with 1, increments with all layer tape-out (implemented with metal ECO-able tie-off)"]
pub type MAJOR_REV_R = crate::FieldReader;
#[doc = "Field `MINOR_REV` reader - Minor Revision, starts with 1, increments with metal layer only tape-out (implemented with metal ECO-able tie-off)"]
pub type MINOR_REV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Family ID a.k.a. Partnumber a.k.a. Silicon ID"]
    #[inline(always)]
    pub fn family_id(&self) -> FAMILY_ID_R {
        FAMILY_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Major Revision, starts with 1, increments with all layer tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor Revision, starts with 1, increments with metal layer only tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Product identifier and version (same as CoreSight RomTables)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRODUCT_ID_SPEC;
impl crate::RegisterSpec for PRODUCT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`product_id::R`](R) reader structure"]
impl crate::Readable for PRODUCT_ID_SPEC {}
#[doc = "`reset()` method sets PRODUCT_ID to value 0"]
impl crate::Resettable for PRODUCT_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
