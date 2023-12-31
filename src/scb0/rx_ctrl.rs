#[doc = "Register `RX_CTRL` reader"]
pub type R = crate::R<RX_CTRL_SPEC>;
#[doc = "Register `RX_CTRL` writer"]
pub type W = crate::W<RX_CTRL_SPEC>;
#[doc = "Field `DATA_WIDTH` reader - N/A"]
pub type DATA_WIDTH_R = crate::FieldReader;
#[doc = "Field `DATA_WIDTH` writer - N/A"]
pub type DATA_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
pub type MSB_FIRST_R = crate::BitReader;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
pub type MSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEDIAN` reader - N/A"]
pub type MEDIAN_R = crate::BitReader;
#[doc = "Field `MEDIAN` writer - N/A"]
pub type MEDIAN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn median(&self) -> MEDIAN_R {
        MEDIAN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<RX_CTRL_SPEC> {
        DATA_WIDTH_W::new(self, 0)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MSB_FIRST_W<RX_CTRL_SPEC> {
        MSB_FIRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn median(&mut self) -> MEDIAN_W<RX_CTRL_SPEC> {
        MEDIAN_W::new(self, 9)
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
#[doc = "Receiver control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ctrl::R`](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_ctrl::W`](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CTRL to value 0x0107"]
impl crate::Resettable for RX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0107;
}
