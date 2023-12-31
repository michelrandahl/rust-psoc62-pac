#[doc = "Register `BUS_RST_CNT` reader"]
pub type R = crate::R<BUS_RST_CNT_SPEC>;
#[doc = "Register `BUS_RST_CNT` writer"]
pub type W = crate::W<BUS_RST_CNT_SPEC>;
#[doc = "Field `BUS_RST_CNT` reader - Bus Reset Count Length"]
pub type BUS_RST_CNT_R = crate::FieldReader;
#[doc = "Field `BUS_RST_CNT` writer - Bus Reset Count Length"]
pub type BUS_RST_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    pub fn bus_rst_cnt(&self) -> BUS_RST_CNT_R {
        BUS_RST_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    #[must_use]
    pub fn bus_rst_cnt(&mut self) -> BUS_RST_CNT_W<BUS_RST_CNT_SPEC> {
        BUS_RST_CNT_W::new(self, 0)
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
#[doc = "Bus Reset Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_rst_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_rst_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_RST_CNT_SPEC;
impl crate::RegisterSpec for BUS_RST_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_rst_cnt::R`](R) reader structure"]
impl crate::Readable for BUS_RST_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_rst_cnt::W`](W) writer structure"]
impl crate::Writable for BUS_RST_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_RST_CNT to value 0x0a"]
impl crate::Resettable for BUS_RST_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
