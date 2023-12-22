#[doc = "Register `TX_CTRL` reader"]
pub type R = crate::R<TX_CTRL_SPEC>;
#[doc = "Register `TX_CTRL` writer"]
pub type W = crate::W<TX_CTRL_SPEC>;
#[doc = "Field `DATA_WIDTH` reader - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
pub type DATA_WIDTH_R = crate::FieldReader;
#[doc = "Field `DATA_WIDTH` writer - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
pub type DATA_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
pub type MSB_FIRST_R = crate::BitReader;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
pub type MSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPEN_DRAIN` reader - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
pub type OPEN_DRAIN_R = crate::BitReader;
#[doc = "Field `OPEN_DRAIN` writer - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
pub type OPEN_DRAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
    #[inline(always)]
    pub fn open_drain(&self) -> OPEN_DRAIN_R {
        OPEN_DRAIN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<TX_CTRL_SPEC> {
        DATA_WIDTH_W::new(self, 0)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MSB_FIRST_W<TX_CTRL_SPEC> {
        MSB_FIRST_W::new(self, 8)
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
    #[inline(always)]
    #[must_use]
    pub fn open_drain(&mut self) -> OPEN_DRAIN_W<TX_CTRL_SPEC> {
        OPEN_DRAIN_W::new(self, 16)
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
#[doc = "Transmitter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ctrl::R`](R) reader structure"]
impl crate::Readable for TX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_ctrl::W`](W) writer structure"]
impl crate::Writable for TX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x0107"]
impl crate::Resettable for TX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0107;
}
