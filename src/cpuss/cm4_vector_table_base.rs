#[doc = "Register `CM4_VECTOR_TABLE_BASE` reader"]
pub type R = crate::R<CM4_VECTOR_TABLE_BASE_SPEC>;
#[doc = "Register `CM4_VECTOR_TABLE_BASE` writer"]
pub type W = crate::W<CM4_VECTOR_TABLE_BASE_SPEC>;
#[doc = "Field `ADDR22` reader - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
pub type ADDR22_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR22` writer - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
pub type ADDR22_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    pub fn addr22(&self) -> ADDR22_R {
        ADDR22_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    #[must_use]
    pub fn addr22(&mut self) -> ADDR22_W<CM4_VECTOR_TABLE_BASE_SPEC> {
        ADDR22_W::new(self, 10)
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
#[doc = "CM4 vector table base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_vector_table_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_vector_table_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_VECTOR_TABLE_BASE_SPEC;
impl crate::RegisterSpec for CM4_VECTOR_TABLE_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_vector_table_base::R`](R) reader structure"]
impl crate::Readable for CM4_VECTOR_TABLE_BASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_vector_table_base::W`](W) writer structure"]
impl crate::Writable for CM4_VECTOR_TABLE_BASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_VECTOR_TABLE_BASE to value 0"]
impl crate::Resettable for CM4_VECTOR_TABLE_BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
