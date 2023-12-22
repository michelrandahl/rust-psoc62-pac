#[doc = "Register `SL_ADDR` reader"]
pub type R = crate::R<SL_ADDR_SPEC>;
#[doc = "Field `ADDR30` reader - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
pub type ADDR30_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    pub fn addr30(&self) -> ADDR30_R {
        ADDR30_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Slave region, base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sl_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SL_ADDR_SPEC;
impl crate::RegisterSpec for SL_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl_addr::R`](R) reader structure"]
impl crate::Readable for SL_ADDR_SPEC {}
#[doc = "`reset()` method sets SL_ADDR to value 0"]
impl crate::Resettable for SL_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
