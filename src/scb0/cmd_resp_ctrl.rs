#[doc = "Register `CMD_RESP_CTRL` reader"]
pub type R = crate::R<CMD_RESP_CTRL_SPEC>;
#[doc = "Register `CMD_RESP_CTRL` writer"]
pub type W = crate::W<CMD_RESP_CTRL_SPEC>;
#[doc = "Field `BASE_RD_ADDR` reader - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_RD_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `BASE_RD_ADDR` writer - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_RD_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BASE_WR_ADDR` reader - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_WR_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `BASE_WR_ADDR` writer - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_WR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&self) -> BASE_RD_ADDR_R {
        BASE_RD_ADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&self) -> BASE_WR_ADDR_R {
        BASE_WR_ADDR_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    #[must_use]
    pub fn base_rd_addr(&mut self) -> BASE_RD_ADDR_W<CMD_RESP_CTRL_SPEC> {
        BASE_RD_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    #[must_use]
    pub fn base_wr_addr(&mut self) -> BASE_WR_ADDR_W<CMD_RESP_CTRL_SPEC> {
        BASE_WR_ADDR_W::new(self, 16)
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
#[doc = "Command/response control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_resp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_resp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_RESP_CTRL_SPEC;
impl crate::RegisterSpec for CMD_RESP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_resp_ctrl::R`](R) reader structure"]
impl crate::Readable for CMD_RESP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_resp_ctrl::W`](W) writer structure"]
impl crate::Writable for CMD_RESP_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_RESP_CTRL to value 0"]
impl crate::Resettable for CMD_RESP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
