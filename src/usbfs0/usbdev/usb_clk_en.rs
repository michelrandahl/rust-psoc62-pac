#[doc = "Register `USB_CLK_EN` reader"]
pub type R = crate::R<USB_CLK_EN_SPEC>;
#[doc = "Register `USB_CLK_EN` writer"]
pub type W = crate::W<USB_CLK_EN_SPEC>;
#[doc = "Field `CSR_CLK_EN` reader - Clock Enable for Core Logic clocked by AHB bus clock"]
pub type CSR_CLK_EN_R = crate::BitReader;
#[doc = "Field `CSR_CLK_EN` writer - Clock Enable for Core Logic clocked by AHB bus clock"]
pub type CSR_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    pub fn csr_clk_en(&self) -> CSR_CLK_EN_R {
        CSR_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    #[must_use]
    pub fn csr_clk_en(&mut self) -> CSR_CLK_EN_W<USB_CLK_EN_SPEC> {
        CSR_CLK_EN_W::new(self, 0)
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
#[doc = "USB Block Clock Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_CLK_EN_SPEC;
impl crate::RegisterSpec for USB_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_clk_en::R`](R) reader structure"]
impl crate::Readable for USB_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_clk_en::W`](W) writer structure"]
impl crate::Writable for USB_CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CLK_EN to value 0"]
impl crate::Resettable for USB_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
