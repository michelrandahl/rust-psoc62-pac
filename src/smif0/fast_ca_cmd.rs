#[doc = "Register `FAST_CA_CMD` reader"]
pub type R = crate::R<FAST_CA_CMD_SPEC>;
#[doc = "Register `FAST_CA_CMD` writer"]
pub type W = crate::W<FAST_CA_CMD_SPEC>;
#[doc = "Field `INV` reader - See SLOW_CA_CMD.INV."]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - See SLOW_CA_CMD.INV."]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - See SLOW_CA_CMD.INV."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See SLOW_CA_CMD.INV."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<FAST_CA_CMD_SPEC> {
        INV_W::new(self, 0)
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
#[doc = "Fast cache command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fast_ca_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fast_ca_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FAST_CA_CMD_SPEC;
impl crate::RegisterSpec for FAST_CA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fast_ca_cmd::R`](R) reader structure"]
impl crate::Readable for FAST_CA_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fast_ca_cmd::W`](W) writer structure"]
impl crate::Writable for FAST_CA_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAST_CA_CMD to value 0"]
impl crate::Resettable for FAST_CA_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
