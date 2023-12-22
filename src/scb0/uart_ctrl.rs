#[doc = "Register `UART_CTRL` reader"]
pub type R = crate::R<UART_CTRL_SPEC>;
#[doc = "Register `UART_CTRL` writer"]
pub type W = crate::W<UART_CTRL_SPEC>;
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - N/A"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: N/A"]
    UART_STD = 0,
    #[doc = "1: N/A"]
    UART_SMARTCARD = 1,
    #[doc = "2: N/A"]
    UART_IRDA = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::UART_STD),
            1 => Some(MODE_A::UART_SMARTCARD),
            2 => Some(MODE_A::UART_IRDA),
            _ => None,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_uart_std(&self) -> bool {
        *self == MODE_A::UART_STD
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_uart_smartcard(&self) -> bool {
        *self == MODE_A::UART_SMARTCARD
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_uart_irda(&self) -> bool {
        *self == MODE_A::UART_IRDA
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart_std(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::UART_STD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart_smartcard(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::UART_SMARTCARD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart_irda(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::UART_IRDA)
    }
}
impl R {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<UART_CTRL_SPEC> {
        LOOPBACK_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<UART_CTRL_SPEC> {
        MODE_W::new(self, 24)
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
#[doc = "UART control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_CTRL_SPEC;
impl crate::RegisterSpec for UART_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ctrl::R`](R) reader structure"]
impl crate::Readable for UART_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_ctrl::W`](W) writer structure"]
impl crate::Writable for UART_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_CTRL to value 0x0300_0000"]
impl crate::Resettable for UART_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0000;
}
