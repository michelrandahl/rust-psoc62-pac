#[doc = "Register `BUF_DATA_R` reader"]
pub type R = crate::R<BUF_DATA_R_SPEC>;
#[doc = "Register `BUF_DATA_R` writer"]
pub type W = crate::W<BUF_DATA_R_SPEC>;
#[doc = "Field `BUF_DATA` reader - Buffer Data These bits enable access to the Host Controller packet buffer."]
pub type BUF_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `BUF_DATA` writer - Buffer Data These bits enable access to the Host Controller packet buffer."]
pub type BUF_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    pub fn buf_data(&self) -> BUF_DATA_R {
        BUF_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    #[must_use]
    pub fn buf_data(&mut self) -> BUF_DATA_W<BUF_DATA_R_SPEC> {
        BUF_DATA_W::new(self, 0)
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
#[doc = "Buffer Data Port Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_data_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_DATA_R_SPEC;
impl crate::RegisterSpec for BUF_DATA_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_data_r::R`](R) reader structure"]
impl crate::Readable for BUF_DATA_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_data_r::W`](W) writer structure"]
impl crate::Writable for BUF_DATA_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUF_DATA_R to value 0"]
impl crate::Resettable for BUF_DATA_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
