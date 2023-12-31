#[doc = "Register `BUF_SIZE` reader"]
pub type R = crate::R<BUF_SIZE_SPEC>;
#[doc = "Register `BUF_SIZE` writer"]
pub type W = crate::W<BUF_SIZE_SPEC>;
#[doc = "Field `IN_BUF` reader - Buffer size for IN Endpoints."]
pub type IN_BUF_R = crate::FieldReader;
#[doc = "Field `IN_BUF` writer - Buffer size for IN Endpoints."]
pub type IN_BUF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUT_BUF` reader - Buffer size for OUT Endpoints."]
pub type OUT_BUF_R = crate::FieldReader;
#[doc = "Field `OUT_BUF` writer - Buffer size for OUT Endpoints."]
pub type OUT_BUF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    pub fn in_buf(&self) -> IN_BUF_R {
        IN_BUF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub fn out_buf(&self) -> OUT_BUF_R {
        OUT_BUF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn in_buf(&mut self) -> IN_BUF_W<BUF_SIZE_SPEC> {
        IN_BUF_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn out_buf(&mut self) -> OUT_BUF_W<BUF_SIZE_SPEC> {
        OUT_BUF_W::new(self, 4)
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
#[doc = "Dedicated Endpoint Buffer Size Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_SIZE_SPEC;
impl crate::RegisterSpec for BUF_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_size::R`](R) reader structure"]
impl crate::Readable for BUF_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_size::W`](W) writer structure"]
impl crate::Writable for BUF_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUF_SIZE to value 0"]
impl crate::Resettable for BUF_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
