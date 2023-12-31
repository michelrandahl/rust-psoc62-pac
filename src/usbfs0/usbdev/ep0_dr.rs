#[doc = "Register `EP0_DR[%s]` reader"]
pub type R = crate::R<EP0_DR_SPEC>;
#[doc = "Register `EP0_DR[%s]` writer"]
pub type W = crate::W<EP0_DR_SPEC>;
#[doc = "Field `DATA_BYTE` reader - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
pub type DATA_BYTE_R = crate::FieldReader;
#[doc = "Field `DATA_BYTE` writer - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
pub type DATA_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub fn data_byte(&self) -> DATA_BYTE_R {
        DATA_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte(&mut self) -> DATA_BYTE_W<EP0_DR_SPEC> {
        DATA_BYTE_W::new(self, 0)
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
#[doc = "Control End point EP0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP0_DR_SPEC;
impl crate::RegisterSpec for EP0_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0_dr::R`](R) reader structure"]
impl crate::Readable for EP0_DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep0_dr::W`](W) writer structure"]
impl crate::Writable for EP0_DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP0_DR[%s]
to value 0"]
impl crate::Resettable for EP0_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
