#[doc = "Register `CHAN_CONFIG[%s]` reader"]
pub type R = crate::R<CHAN_CONFIG_SPEC>;
#[doc = "Register `CHAN_CONFIG[%s]` writer"]
pub type W = crate::W<CHAN_CONFIG_SPEC>;
#[doc = "Field `POS_PIN_ADDR` reader - Address of the pin to be sampled by this channel (connected to Vplus)"]
pub type POS_PIN_ADDR_R = crate::FieldReader;
#[doc = "Field `POS_PIN_ADDR` writer - Address of the pin to be sampled by this channel (connected to Vplus)"]
pub type POS_PIN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POS_PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
pub type POS_PORT_ADDR_R = crate::FieldReader<POS_PORT_ADDR_A>;
#[doc = "Address of the port that contains the pin to be sampled by this channel (connected to Vplus)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POS_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "1: CTB0"]
    CTB0 = 1,
    #[doc = "2: CTB1"]
    CTB1 = 2,
    #[doc = "3: CTB2"]
    CTB2 = 3,
    #[doc = "4: CTB3"]
    CTB3 = 4,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 5,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 6,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 7,
}
impl From<POS_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: POS_PORT_ADDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POS_PORT_ADDR_A {
    type Ux = u8;
}
impl POS_PORT_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POS_PORT_ADDR_A {
        match self.bits {
            0 => POS_PORT_ADDR_A::SARMUX,
            1 => POS_PORT_ADDR_A::CTB0,
            2 => POS_PORT_ADDR_A::CTB1,
            3 => POS_PORT_ADDR_A::CTB2,
            4 => POS_PORT_ADDR_A::CTB3,
            5 => POS_PORT_ADDR_A::AROUTE_VIRT2,
            6 => POS_PORT_ADDR_A::AROUTE_VIRT1,
            7 => POS_PORT_ADDR_A::SARMUX_VIRT,
            _ => unreachable!(),
        }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == POS_PORT_ADDR_A::SARMUX
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB0
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB1
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB2
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB3
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        *self == POS_PORT_ADDR_A::AROUTE_VIRT2
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        *self == POS_PORT_ADDR_A::AROUTE_VIRT1
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == POS_PORT_ADDR_A::SARMUX_VIRT
    }
}
#[doc = "Field `POS_PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
pub type POS_PORT_ADDR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, POS_PORT_ADDR_A>;
impl<'a, REG> POS_PORT_ADDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::SARMUX)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::CTB0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::CTB1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::CTB2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::CTB3)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::AROUTE_VIRT2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::AROUTE_VIRT1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut crate::W<REG> {
        self.variant(POS_PORT_ADDR_A::SARMUX_VIRT)
    }
}
#[doc = "Field `DIFFERENTIAL_EN` reader - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
pub type DIFFERENTIAL_EN_R = crate::BitReader;
#[doc = "Field `DIFFERENTIAL_EN` writer - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
pub type DIFFERENTIAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type AVG_EN_R = crate::BitReader;
#[doc = "Field `AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type AVG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_TIME_SEL` reader - Sample time select: select which of the 4 global sample times to use for this channel"]
pub type SAMPLE_TIME_SEL_R = crate::FieldReader;
#[doc = "Field `SAMPLE_TIME_SEL` writer - Sample time select: select which of the 4 global sample times to use for this channel"]
pub type SAMPLE_TIME_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NEG_PIN_ADDR` reader - Address of the neg pin to be sampled by this channel."]
pub type NEG_PIN_ADDR_R = crate::FieldReader;
#[doc = "Field `NEG_PIN_ADDR` writer - Address of the neg pin to be sampled by this channel."]
pub type NEG_PIN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NEG_PORT_ADDR` reader - Address of the neg port that contains the pin to be sampled by this channel."]
pub type NEG_PORT_ADDR_R = crate::FieldReader<NEG_PORT_ADDR_A>;
#[doc = "Address of the neg port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NEG_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 5,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 6,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 7,
}
impl From<NEG_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: NEG_PORT_ADDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NEG_PORT_ADDR_A {
    type Ux = u8;
}
impl NEG_PORT_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NEG_PORT_ADDR_A> {
        match self.bits {
            0 => Some(NEG_PORT_ADDR_A::SARMUX),
            5 => Some(NEG_PORT_ADDR_A::AROUTE_VIRT2),
            6 => Some(NEG_PORT_ADDR_A::AROUTE_VIRT1),
            7 => Some(NEG_PORT_ADDR_A::SARMUX_VIRT),
            _ => None,
        }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == NEG_PORT_ADDR_A::SARMUX
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        *self == NEG_PORT_ADDR_A::AROUTE_VIRT2
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        *self == NEG_PORT_ADDR_A::AROUTE_VIRT1
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == NEG_PORT_ADDR_A::SARMUX_VIRT
    }
}
#[doc = "Field `NEG_PORT_ADDR` writer - Address of the neg port that contains the pin to be sampled by this channel."]
pub type NEG_PORT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, NEG_PORT_ADDR_A>;
impl<'a, REG> NEG_PORT_ADDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut crate::W<REG> {
        self.variant(NEG_PORT_ADDR_A::SARMUX)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut crate::W<REG> {
        self.variant(NEG_PORT_ADDR_A::AROUTE_VIRT2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut crate::W<REG> {
        self.variant(NEG_PORT_ADDR_A::AROUTE_VIRT1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut crate::W<REG> {
        self.variant(NEG_PORT_ADDR_A::SARMUX_VIRT)
    }
}
#[doc = "Field `NEG_ADDR_EN` reader - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
pub type NEG_ADDR_EN_R = crate::BitReader;
#[doc = "Field `NEG_ADDR_EN` writer - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
pub type NEG_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_OUT_EN` reader - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub type DSI_OUT_EN_R = crate::BitReader;
#[doc = "Field `DSI_OUT_EN` writer - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub type DSI_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_pin_addr(&self) -> POS_PIN_ADDR_R {
        POS_PIN_ADDR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_port_addr(&self) -> POS_PORT_ADDR_R {
        POS_PORT_ADDR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn differential_en(&self) -> DIFFERENTIAL_EN_R {
        DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&self) -> AVG_EN_R {
        AVG_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&self) -> SAMPLE_TIME_SEL_R {
        SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_pin_addr(&self) -> NEG_PIN_ADDR_R {
        NEG_PIN_ADDR_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_port_addr(&self) -> NEG_PORT_ADDR_R {
        NEG_PORT_ADDR_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub fn neg_addr_en(&self) -> NEG_ADDR_EN_R {
        NEG_ADDR_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&self) -> DSI_OUT_EN_R {
        DSI_OUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    #[must_use]
    pub fn pos_pin_addr(&mut self) -> POS_PIN_ADDR_W<CHAN_CONFIG_SPEC> {
        POS_PIN_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    #[must_use]
    pub fn pos_port_addr(&mut self) -> POS_PORT_ADDR_W<CHAN_CONFIG_SPEC> {
        POS_PORT_ADDR_W::new(self, 4)
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    #[must_use]
    pub fn differential_en(&mut self) -> DIFFERENTIAL_EN_W<CHAN_CONFIG_SPEC> {
        DIFFERENTIAL_EN_W::new(self, 8)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    #[must_use]
    pub fn avg_en(&mut self) -> AVG_EN_W<CHAN_CONFIG_SPEC> {
        AVG_EN_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time_sel(&mut self) -> SAMPLE_TIME_SEL_W<CHAN_CONFIG_SPEC> {
        SAMPLE_TIME_SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    #[must_use]
    pub fn neg_pin_addr(&mut self) -> NEG_PIN_ADDR_W<CHAN_CONFIG_SPEC> {
        NEG_PIN_ADDR_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    #[must_use]
    pub fn neg_port_addr(&mut self) -> NEG_PORT_ADDR_W<CHAN_CONFIG_SPEC> {
        NEG_PORT_ADDR_W::new(self, 20)
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    #[must_use]
    pub fn neg_addr_en(&mut self) -> NEG_ADDR_EN_W<CHAN_CONFIG_SPEC> {
        NEG_ADDR_EN_W::new(self, 24)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_out_en(&mut self) -> DSI_OUT_EN_W<CHAN_CONFIG_SPEC> {
        DSI_OUT_EN_W::new(self, 31)
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
#[doc = "Channel configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chan_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chan_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHAN_CONFIG_SPEC;
impl crate::RegisterSpec for CHAN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_config::R`](R) reader structure"]
impl crate::Readable for CHAN_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chan_config::W`](W) writer structure"]
impl crate::Writable for CHAN_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHAN_CONFIG[%s]
to value 0"]
impl crate::Resettable for CHAN_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
