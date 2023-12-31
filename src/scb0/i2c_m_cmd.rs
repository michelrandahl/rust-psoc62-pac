#[doc = "Register `I2C_M_CMD` reader"]
pub type R = crate::R<I2C_M_CMD_SPEC>;
#[doc = "Register `I2C_M_CMD` writer"]
pub type W = crate::W<I2C_M_CMD_SPEC>;
#[doc = "Field `M_START` reader - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_R = crate::BitReader;
#[doc = "Field `M_START` writer - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_START_ON_IDLE` reader - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_ON_IDLE_R = crate::BitReader;
#[doc = "Field `M_START_ON_IDLE` writer - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_ON_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_ACK` reader - N/A"]
pub type M_ACK_R = crate::BitReader;
#[doc = "Field `M_ACK` writer - N/A"]
pub type M_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_NACK` reader - N/A"]
pub type M_NACK_R = crate::BitReader;
#[doc = "Field `M_NACK` writer - N/A"]
pub type M_NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_STOP` reader - N/A"]
pub type M_STOP_R = crate::BitReader;
#[doc = "Field `M_STOP` writer - N/A"]
pub type M_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&self) -> M_START_R {
        M_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start_on_idle(&self) -> M_START_ON_IDLE_R {
        M_START_ON_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn m_ack(&self) -> M_ACK_R {
        M_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn m_nack(&self) -> M_NACK_R {
        M_NACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn m_stop(&self) -> M_STOP_R {
        M_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn m_start(&mut self) -> M_START_W<I2C_M_CMD_SPEC> {
        M_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn m_start_on_idle(&mut self) -> M_START_ON_IDLE_W<I2C_M_CMD_SPEC> {
        M_START_ON_IDLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn m_ack(&mut self) -> M_ACK_W<I2C_M_CMD_SPEC> {
        M_ACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn m_nack(&mut self) -> M_NACK_W<I2C_M_CMD_SPEC> {
        M_NACK_W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn m_stop(&mut self) -> M_STOP_W<I2C_M_CMD_SPEC> {
        M_STOP_W::new(self, 4)
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
#[doc = "I2C master command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_m_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_m_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_M_CMD_SPEC;
impl crate::RegisterSpec for I2C_M_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_m_cmd::R`](R) reader structure"]
impl crate::Readable for I2C_M_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_m_cmd::W`](W) writer structure"]
impl crate::Writable for I2C_M_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_M_CMD to value 0"]
impl crate::Resettable for I2C_M_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
