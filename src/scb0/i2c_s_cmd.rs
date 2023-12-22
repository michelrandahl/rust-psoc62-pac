#[doc = "Register `I2C_S_CMD` reader"]
pub type R = crate::R<I2C_S_CMD_SPEC>;
#[doc = "Register `I2C_S_CMD` writer"]
pub type W = crate::W<I2C_S_CMD_SPEC>;
#[doc = "Field `S_ACK` reader - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
pub type S_ACK_R = crate::BitReader;
#[doc = "Field `S_ACK` writer - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
pub type S_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_NACK` reader - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
pub type S_NACK_R = crate::BitReader;
#[doc = "Field `S_NACK` writer - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
pub type S_NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
    #[inline(always)]
    pub fn s_ack(&self) -> S_ACK_R {
        S_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub fn s_nack(&self) -> S_NACK_R {
        S_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
    #[inline(always)]
    #[must_use]
    pub fn s_ack(&mut self) -> S_ACK_W<I2C_S_CMD_SPEC> {
        S_ACK_W::new(self, 0)
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    #[must_use]
    pub fn s_nack(&mut self) -> S_NACK_W<I2C_S_CMD_SPEC> {
        S_NACK_W::new(self, 1)
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
#[doc = "I2C slave command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_s_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_s_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_S_CMD_SPEC;
impl crate::RegisterSpec for I2C_S_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_s_cmd::R`](R) reader structure"]
impl crate::Readable for I2C_S_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_s_cmd::W`](W) writer structure"]
impl crate::Writable for I2C_S_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_S_CMD to value 0"]
impl crate::Resettable for I2C_S_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
