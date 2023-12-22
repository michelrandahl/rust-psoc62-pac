#[doc = "Register `DATA0` reader"]
pub type R = crate::R<DATA0_SPEC>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<DATA0_SPEC>;
#[doc = "Field `DATA` reader - This field holds a 32-bit data element that is associated with the IPC structure."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - This field holds a 32-bit data element that is associated with the IPC structure."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATA0_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "IPC data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0_SPEC;
impl crate::RegisterSpec for DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for DATA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for DATA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
