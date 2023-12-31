#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `BIT_DATA` reader - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
pub type BIT_DATA_R = crate::BitReader;
#[doc = "Field `BIT_DATA` writer - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
pub type BIT_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ADDR` reader - Bit address. This field specifies a bit within a Byte."]
pub type BIT_ADDR_R = crate::FieldReader;
#[doc = "Field `BIT_ADDR` writer - Bit address. This field specifies a bit within a Byte."]
pub type BIT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BYTE_ADDR` reader - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
pub type BYTE_ADDR_R = crate::FieldReader;
#[doc = "Field `BYTE_ADDR` writer - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
pub type BYTE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MACRO_ADDR` reader - Macro address. This field specifies an eFUSE macro."]
pub type MACRO_ADDR_R = crate::FieldReader;
#[doc = "Field `MACRO_ADDR` writer - Macro address. This field specifies an eFUSE macro."]
pub type MACRO_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `START` reader - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub fn bit_data(&self) -> BIT_DATA_R {
        BIT_DATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub fn bit_addr(&self) -> BIT_ADDR_R {
        BIT_ADDR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    pub fn byte_addr(&self) -> BYTE_ADDR_R {
        BYTE_ADDR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    pub fn macro_addr(&self) -> MACRO_ADDR_R {
        MACRO_ADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    #[must_use]
    pub fn bit_data(&mut self) -> BIT_DATA_W<CMD_SPEC> {
        BIT_DATA_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    #[must_use]
    pub fn bit_addr(&mut self) -> BIT_ADDR_W<CMD_SPEC> {
        BIT_ADDR_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    #[must_use]
    pub fn byte_addr(&mut self) -> BYTE_ADDR_W<CMD_SPEC> {
        BYTE_ADDR_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    #[must_use]
    pub fn macro_addr(&mut self) -> MACRO_ADDR_W<CMD_SPEC> {
        MACRO_ADDR_W::new(self, 16)
    }
    #[doc = "Bit 31 - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CMD_SPEC> {
        START_W::new(self, 31)
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
#[doc = "Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0x01"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
