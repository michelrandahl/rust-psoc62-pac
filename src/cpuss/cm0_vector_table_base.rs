#[doc = "Register `CM0_VECTOR_TABLE_BASE` reader"]
pub type R = crate::R<CM0_VECTOR_TABLE_BASE_SPEC>;
#[doc = "Register `CM0_VECTOR_TABLE_BASE` writer"]
pub type W = crate::W<CM0_VECTOR_TABLE_BASE_SPEC>;
#[doc = "Field `ADDR24` reader - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
pub type ADDR24_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR24` writer - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
pub type ADDR24_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    #[must_use]
    pub fn addr24(&mut self) -> ADDR24_W<CM0_VECTOR_TABLE_BASE_SPEC> {
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
#[doc = "CM0+ vector table base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_vector_table_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_vector_table_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM0_VECTOR_TABLE_BASE_SPEC;
impl crate::RegisterSpec for CM0_VECTOR_TABLE_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_vector_table_base::R`](R) reader structure"]
impl crate::Readable for CM0_VECTOR_TABLE_BASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm0_vector_table_base::W`](W) writer structure"]
impl crate::Writable for CM0_VECTOR_TABLE_BASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0_VECTOR_TABLE_BASE to value 0"]
impl crate::Resettable for CM0_VECTOR_TABLE_BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
