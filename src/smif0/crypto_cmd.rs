#[doc = "Register `CRYPTO_CMD` reader"]
pub type R = crate::R<CRYPTO_CMD_SPEC>;
#[doc = "Register `CRYPTO_CMD` writer"]
pub type W = crate::W<CRYPTO_CMD_SPEC>;
#[doc = "Field `START` reader - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRYPTO_CMD_SPEC> {
        START_W::new(self, 0)
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
#[doc = "Cryptography Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_CMD_SPEC;
impl crate::RegisterSpec for CRYPTO_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_cmd::R`](R) reader structure"]
impl crate::Readable for CRYPTO_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crypto_cmd::W`](W) writer structure"]
impl crate::Writable for CRYPTO_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_CMD to value 0"]
impl crate::Resettable for CRYPTO_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
