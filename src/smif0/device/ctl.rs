#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `WR_EN` reader - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WR_EN_R = crate::BitReader;
#[doc = "Field `WR_EN` writer - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_EN` reader - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CRYPTO_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_EN` writer - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CRYPTO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_SEL` reader - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DATA_SEL_R = crate::FieldReader;
#[doc = "Field `DATA_SEL` writer - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DATA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENABLED` reader - Device enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Device enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn crypto_en(&self) -> CRYPTO_EN_R {
        CRYPTO_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn data_sel(&self) -> DATA_SEL_R {
        DATA_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WR_EN_W<CTL_SPEC> {
        WR_EN_W::new(self, 0)
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_en(&mut self) -> CRYPTO_EN_W<CTL_SPEC> {
        CRYPTO_EN_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    #[must_use]
    pub fn data_sel(&mut self) -> DATA_SEL_W<CTL_SPEC> {
        DATA_SEL_W::new(self, 16)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTL_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
