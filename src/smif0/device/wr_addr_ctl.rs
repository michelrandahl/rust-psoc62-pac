#[doc = "Register `WR_ADDR_CTL` reader"]
pub type R = crate::R<WR_ADDR_CTL_SPEC>;
#[doc = "Register `WR_ADDR_CTL` writer"]
pub type W = crate::W<WR_ADDR_CTL_SPEC>;
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WIDTH_R = crate::FieldReader;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<WR_ADDR_CTL_SPEC> {
        WIDTH_W::new(self, 16)
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
#[doc = "Write address control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_addr_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_addr_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_ADDR_CTL_SPEC;
impl crate::RegisterSpec for WR_ADDR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_addr_ctl::R`](R) reader structure"]
impl crate::Readable for WR_ADDR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_addr_ctl::W`](W) writer structure"]
impl crate::Writable for WR_ADDR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_ADDR_CTL to value 0"]
impl crate::Resettable for WR_ADDR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
