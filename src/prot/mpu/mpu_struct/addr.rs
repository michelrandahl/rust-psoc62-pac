#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `SUBREGION_DISABLE` reader - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by MPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
pub type SUBREGION_DISABLE_R = crate::FieldReader;
#[doc = "Field `SUBREGION_DISABLE` writer - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by MPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
pub type SUBREGION_DISABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR24` reader - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
pub type ADDR24_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR24` writer - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
pub type ADDR24_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by MPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by MPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    #[must_use]
    pub fn subregion_disable(&mut self) -> SUBREGION_DISABLE_W<ADDR_SPEC> {
        SUBREGION_DISABLE_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn addr24(&mut self) -> ADDR24_W<ADDR_SPEC> {
        ADDR24_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MPU region address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
