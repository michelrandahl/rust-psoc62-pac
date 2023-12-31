#[doc = "Register `ADDR_CTL` reader"]
pub type R = crate::R<ADDR_CTL_SPEC>;
#[doc = "Register `ADDR_CTL` writer"]
pub type W = crate::W<ADDR_CTL_SPEC>;
#[doc = "Field `SIZE2` reader - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type SIZE2_R = crate::FieldReader;
#[doc = "Field `SIZE2` writer - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type SIZE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIV2` reader - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type DIV2_R = crate::BitReader;
#[doc = "Field `DIV2` writer - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type DIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn size2(&self) -> SIZE2_R {
        SIZE2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn div2(&self) -> DIV2_R {
        DIV2_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    #[must_use]
    pub fn size2(&mut self) -> SIZE2_W<ADDR_CTL_SPEC> {
        SIZE2_W::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    #[must_use]
    pub fn div2(&mut self) -> DIV2_W<ADDR_CTL_SPEC> {
        DIV2_W::new(self, 8)
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
#[doc = "Address control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_CTL_SPEC;
impl crate::RegisterSpec for ADDR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_ctl::R`](R) reader structure"]
impl crate::Readable for ADDR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_ctl::W`](W) writer structure"]
impl crate::Writable for ADDR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR_CTL to value 0"]
impl crate::Resettable for ADDR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
