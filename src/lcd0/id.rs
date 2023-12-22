#[doc = "Register `ID` reader"]
pub type R = crate::R<ID_SPEC>;
#[doc = "Field `ID` reader - the ID of LCD controller peripheral is 0xF0F0"]
pub type ID_R = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - the version number is 0x0001"]
pub type REVISION_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - the ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the version number is 0x0001"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ID &amp; Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for ID_SPEC {}
#[doc = "`reset()` method sets ID to value 0x0001_f0f0"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_f0f0;
}
