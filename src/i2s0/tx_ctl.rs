# [doc = "Register `TX_CTL` reader"] pub type R = crate :: R < TX_CTL_SPEC > ; # [doc = "Register `TX_CTL` writer"] pub type W = crate :: W < TX_CTL_SPEC > ; # [doc = "Field `B_CLOCK_INV` reader - Serial data transmission is advanced by 0.5 SCK cycles. This bit is valid only in TX slave mode. When set to '1', the serial data will be transmitted 0.5 SCK cycles earlier than when set to '0'. 1) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK falling edge 2) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK rising edge that is 0.5 SCK cycles before the SCK falling edge in 1) 3) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK rising edge 4) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK falling edge that is 0.5 SCK cycles before the SCK rising edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual timing is generated by an internal clock that runs 8x the SCK frequency). The word sync (TX_WS) signal is not affected by this bit setting. Note: When Master mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.TX_BCLKINV)"] pub type B_CLOCK_INV_R = crate :: BitReader < B_CLOCK_INV_A > ; # [doc = "Serial data transmission is advanced by 0.5 SCK cycles. This bit is valid only in TX slave mode. When set to '1', the serial data will be transmitted 0.5 SCK cycles earlier than when set to '0'. 1) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK falling edge 2) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK rising edge that is 0.5 SCK cycles before the SCK falling edge in 1) 3) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK rising edge 4) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK falling edge that is 0.5 SCK cycles before the SCK rising edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual timing is generated by an internal clock that runs 8x the SCK frequency). The word sync (TX_WS) signal is not affected by this bit setting. Note: When Master mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.TX_BCLKINV)\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum B_CLOCK_INV_A { # [doc = "0: SDO transmitted at SCK falling edge when TX_CTL.SCKI_POL=0"] FALLING_EDGE_TX = 0 , # [doc = "1: SDO transmitted at SCK rising edge when TX_CTL.SCKI_POL=0"] RISING_EDGE_TX = 1 , } impl From < B_CLOCK_INV_A > for bool { # [inline (always)] fn from (variant : B_CLOCK_INV_A) -> Self { variant as u8 != 0 } } impl B_CLOCK_INV_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> B_CLOCK_INV_A { match self . bits { false => B_CLOCK_INV_A :: FALLING_EDGE_TX , true => B_CLOCK_INV_A :: RISING_EDGE_TX , } } # [doc = "SDO transmitted at SCK falling edge when TX_CTL.SCKI_POL=0"] # [inline (always)] pub fn is_falling_edge_tx (& self) -> bool { * self == B_CLOCK_INV_A :: FALLING_EDGE_TX } # [doc = "SDO transmitted at SCK rising edge when TX_CTL.SCKI_POL=0"] # [inline (always)] pub fn is_rising_edge_tx (& self) -> bool { * self == B_CLOCK_INV_A :: RISING_EDGE_TX } } # [doc = "Field `B_CLOCK_INV` writer - Serial data transmission is advanced by 0.5 SCK cycles. This bit is valid only in TX slave mode. When set to '1', the serial data will be transmitted 0.5 SCK cycles earlier than when set to '0'. 1) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK falling edge 2) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK rising edge that is 0.5 SCK cycles before the SCK falling edge in 1) 3) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK rising edge 4) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK falling edge that is 0.5 SCK cycles before the SCK rising edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual timing is generated by an internal clock that runs 8x the SCK frequency). The word sync (TX_WS) signal is not affected by this bit setting. Note: When Master mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.TX_BCLKINV)"] pub type B_CLOCK_INV_W < 'a , REG > = crate :: BitWriter < 'a , REG , B_CLOCK_INV_A > ; impl < 'a , REG > B_CLOCK_INV_W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "SDO transmitted at SCK falling edge when TX_CTL.SCKI_POL=0"] # [inline (always)] pub fn falling_edge_tx (self) -> & 'a mut crate :: W < REG > { self . variant (B_CLOCK_INV_A :: FALLING_EDGE_TX) } # [doc = "SDO transmitted at SCK rising edge when TX_CTL.SCKI_POL=0"] # [inline (always)] pub fn rising_edge_tx (self) -> & 'a mut crate :: W < REG > { self . variant (B_CLOCK_INV_A :: RISING_EDGE_TX) } } # [doc = "Field `CH_NR` reader - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHSET)"] pub type CH_NR_R = crate :: FieldReader < CH_NR_A > ; # [doc = "Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHSET)\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CH_NR_A { # [doc = "0: 1 channel"] CH_NUM1 = 0 , # [doc = "1: 2 channels"] CH_NUM2 = 1 , # [doc = "2: 3 channels"] CH_NUM3 = 2 , # [doc = "3: 4 channels"] CH_NUM4 = 3 , # [doc = "4: 5 channels"] CH_NUM5 = 4 , # [doc = "5: 6 channels"] CH_NUM6 = 5 , # [doc = "6: 7 channels"] CH_NUM7 = 6 , # [doc = "7: 8 channels"] CH_NUM8 = 7 , } impl From < CH_NR_A > for u8 { # [inline (always)] fn from (variant : CH_NR_A) -> Self { variant as _ } } impl crate :: FieldSpec for CH_NR_A { type Ux = u8 ; } impl CH_NR_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CH_NR_A { match self . bits { 0 => CH_NR_A :: CH_NUM1 , 1 => CH_NR_A :: CH_NUM2 , 2 => CH_NR_A :: CH_NUM3 , 3 => CH_NR_A :: CH_NUM4 , 4 => CH_NR_A :: CH_NUM5 , 5 => CH_NR_A :: CH_NUM6 , 6 => CH_NR_A :: CH_NUM7 , 7 => CH_NR_A :: CH_NUM8 , _ => unreachable ! () , } } # [doc = "1 channel"] # [inline (always)] pub fn is_ch_num1 (& self) -> bool { * self == CH_NR_A :: CH_NUM1 } # [doc = "2 channels"] # [inline (always)] pub fn is_ch_num2 (& self) -> bool { * self == CH_NR_A :: CH_NUM2 } # [doc = "3 channels"] # [inline (always)] pub fn is_ch_num3 (& self) -> bool { * self == CH_NR_A :: CH_NUM3 } # [doc = "4 channels"] # [inline (always)] pub fn is_ch_num4 (& self) -> bool { * self == CH_NR_A :: CH_NUM4 } # [doc = "5 channels"] # [inline (always)] pub fn is_ch_num5 (& self) -> bool { * self == CH_NR_A :: CH_NUM5 } # [doc = "6 channels"] # [inline (always)] pub fn is_ch_num6 (& self) -> bool { * self == CH_NR_A :: CH_NUM6 } # [doc = "7 channels"] # [inline (always)] pub fn is_ch_num7 (& self) -> bool { * self == CH_NR_A :: CH_NUM7 } # [doc = "8 channels"] # [inline (always)] pub fn is_ch_num8 (& self) -> bool { * self == CH_NR_A :: CH_NUM8 } } # [doc = "Field `CH_NR` writer - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHSET)"] pub type CH_NR_W < 'a , REG > = crate :: FieldWriterSafe < 'a , REG , 3 , CH_NR_A > ; impl < 'a , REG > CH_NR_W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > , { # [doc = "1 channel"] # [inline (always)] pub fn ch_num1 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM1) } # [doc = "2 channels"] # [inline (always)] pub fn ch_num2 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM2) } # [doc = "3 channels"] # [inline (always)] pub fn ch_num3 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM3) } # [doc = "4 channels"] # [inline (always)] pub fn ch_num4 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM4) } # [doc = "5 channels"] # [inline (always)] pub fn ch_num5 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM5) } # [doc = "6 channels"] # [inline (always)] pub fn ch_num6 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM6) } # [doc = "7 channels"] # [inline (always)] pub fn ch_num7 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM7) } # [doc = "8 channels"] # [inline (always)] pub fn ch_num8 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_NR_A :: CH_NUM8) } } # [doc = "Field `MS` reader - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_MS)"] pub type MS_R = crate :: BitReader < MS_A > ; # [doc = "Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_MS)\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum MS_A { # [doc = "0: Slave"] SLAVE = 0 , # [doc = "1: Master"] MASTER = 1 , } impl From < MS_A > for bool { # [inline (always)] fn from (variant : MS_A) -> Self { variant as u8 != 0 } } impl MS_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> MS_A { match self . bits { false => MS_A :: SLAVE , true => MS_A :: MASTER , } } # [doc = "Slave"] # [inline (always)] pub fn is_slave (& self) -> bool { * self == MS_A :: SLAVE } # [doc = "Master"] # [inline (always)] pub fn is_master (& self) -> bool { * self == MS_A :: MASTER } } # [doc = "Field `MS` writer - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_MS)"] pub type MS_W < 'a , REG > = crate :: BitWriter < 'a , REG , MS_A > ; impl < 'a , REG > MS_W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Slave"] # [inline (always)] pub fn slave (self) -> & 'a mut crate :: W < REG > { self . variant (MS_A :: SLAVE) } # [doc = "Master"] # [inline (always)] pub fn master (self) -> & 'a mut crate :: W < REG > { self . variant (MS_A :: MASTER) } } # [doc = "Field `I2S_MODE` reader - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.TX_CFG.TX_I2S_MODE)"] pub type I2S_MODE_R = crate :: FieldReader < I2S_MODE_A > ; # [doc = "Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.TX_CFG.TX_I2S_MODE)\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum I2S_MODE_A { # [doc = "0: Left Justified"] LEFT_JUSTIFIED = 0 , # [doc = "1: I2S mode"] I2S = 1 , # [doc = "2: TDM mode A, the 1st Channel align to WSO Rising Edge"] TDM_A = 2 , # [doc = "3: TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"] TDM_B = 3 , } impl From < I2S_MODE_A > for u8 { # [inline (always)] fn from (variant : I2S_MODE_A) -> Self { variant as _ } } impl crate :: FieldSpec for I2S_MODE_A { type Ux = u8 ; } impl I2S_MODE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> I2S_MODE_A { match self . bits { 0 => I2S_MODE_A :: LEFT_JUSTIFIED , 1 => I2S_MODE_A :: I2S , 2 => I2S_MODE_A :: TDM_A , 3 => I2S_MODE_A :: TDM_B , _ => unreachable ! () , } } # [doc = "Left Justified"] # [inline (always)] pub fn is_left_justified (& self) -> bool { * self == I2S_MODE_A :: LEFT_JUSTIFIED } # [doc = "I2S mode"] # [inline (always)] pub fn is_i2s (& self) -> bool { * self == I2S_MODE_A :: I2S } # [doc = "TDM mode A, the 1st Channel align to WSO Rising Edge"] # [inline (always)] pub fn is_tdm_a (& self) -> bool { * self == I2S_MODE_A :: TDM_A } # [doc = "TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"] # [inline (always)] pub fn is_tdm_b (& self) -> bool { * self == I2S_MODE_A :: TDM_B } } # [doc = "Field `I2S_MODE` writer - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.TX_CFG.TX_I2S_MODE)"] pub type I2S_MODE_W < 'a , REG > = crate :: FieldWriterSafe < 'a , REG , 2 , I2S_MODE_A > ; impl < 'a , REG > I2S_MODE_W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > , { # [doc = "Left Justified"] # [inline (always)] pub fn left_justified (self) -> & 'a mut crate :: W < REG > { self . variant (I2S_MODE_A :: LEFT_JUSTIFIED) } # [doc = "I2S mode"] # [inline (always)] pub fn i2s (self) -> & 'a mut crate :: W < REG > { self . variant (I2S_MODE_A :: I2S) } # [doc = "TDM mode A, the 1st Channel align to WSO Rising Edge"] # [inline (always)] pub fn tdm_a (self) -> & 'a mut crate :: W < REG > { self . variant (I2S_MODE_A :: TDM_A) } # [doc = "TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"] # [inline (always)] pub fn tdm_b (self) -> & 'a mut crate :: W < REG > { self . variant (I2S_MODE_A :: TDM_B) } } # [doc = "Field `WS_PULSE` reader - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_WS_PULSE) Note: When not TDM mode, must be '1'."] pub type WS_PULSE_R = crate :: BitReader < WS_PULSE_A > ; # [doc = "Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_WS_PULSE) Note: When not TDM mode, must be '1'.\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum WS_PULSE_A { # [doc = "0: Pulse width is 1 SCK period"] SCK_PERIOD = 0 , # [doc = "1: Pulse width is 1 channel length"] CH_LENGTH = 1 , } impl From < WS_PULSE_A > for bool { # [inline (always)] fn from (variant : WS_PULSE_A) -> Self { variant as u8 != 0 } } impl WS_PULSE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> WS_PULSE_A { match self . bits { false => WS_PULSE_A :: SCK_PERIOD , true => WS_PULSE_A :: CH_LENGTH , } } # [doc = "Pulse width is 1 SCK period"] # [inline (always)] pub fn is_sck_period (& self) -> bool { * self == WS_PULSE_A :: SCK_PERIOD } # [doc = "Pulse width is 1 channel length"] # [inline (always)] pub fn is_ch_length (& self) -> bool { * self == WS_PULSE_A :: CH_LENGTH } } # [doc = "Field `WS_PULSE` writer - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_WS_PULSE) Note: When not TDM mode, must be '1'."] pub type WS_PULSE_W < 'a , REG > = crate :: BitWriter < 'a , REG , WS_PULSE_A > ; impl < 'a , REG > WS_PULSE_W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Pulse width is 1 SCK period"] # [inline (always)] pub fn sck_period (self) -> & 'a mut crate :: W < REG > { self . variant (WS_PULSE_A :: SCK_PERIOD) } # [doc = "Pulse width is 1 channel length"] # [inline (always)] pub fn ch_length (self) -> & 'a mut crate :: W < REG > { self . variant (WS_PULSE_A :: CH_LENGTH) } } # [doc = "Field `OVHDATA` reader - Set overhead value: '0': Set to '0' '1': Set to '1' (Note: This bit is connected to AR38U12.TX_CFG.TX_OVHDATA)"] pub type OVHDATA_R = crate :: BitReader ; # [doc = "Field `OVHDATA` writer - Set overhead value: '0': Set to '0' '1': Set to '1' (Note: This bit is connected to AR38U12.TX_CFG.TX_OVHDATA)"] pub type OVHDATA_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `WD_EN` reader - Set watchdog for 'tx_ws_in': '0': Disabled. '1': Enabled."] pub type WD_EN_R = crate :: BitReader ; # [doc = "Field `WD_EN` writer - Set watchdog for 'tx_ws_in': '0': Disabled. '1': Enabled."] pub type WD_EN_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `CH_LEN` reader - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHLEN)"] pub type CH_LEN_R = crate :: FieldReader < CH_LEN_A > ; # [doc = "Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHLEN)\n\nValue on reset: 4"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CH_LEN_A { # [doc = "0: 8-bit"] BIT_LEN8 = 0 , # [doc = "1: 16-bit"] BIT_LEN16 = 1 , # [doc = "2: 18-bit"] BIT_LEN18 = 2 , # [doc = "3: 20-bit"] BIT_LEN20 = 3 , # [doc = "4: 24-bit"] BIT_LEN24 = 4 , # [doc = "5: 32-bit"] BIT_LEN32 = 5 , } impl From < CH_LEN_A > for u8 { # [inline (always)] fn from (variant : CH_LEN_A) -> Self { variant as _ } } impl crate :: FieldSpec for CH_LEN_A { type Ux = u8 ; } impl CH_LEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < CH_LEN_A > { match self . bits { 0 => Some (CH_LEN_A :: BIT_LEN8) , 1 => Some (CH_LEN_A :: BIT_LEN16) , 2 => Some (CH_LEN_A :: BIT_LEN18) , 3 => Some (CH_LEN_A :: BIT_LEN20) , 4 => Some (CH_LEN_A :: BIT_LEN24) , 5 => Some (CH_LEN_A :: BIT_LEN32) , _ => None , } } # [doc = "8-bit"] # [inline (always)] pub fn is_bit_len8 (& self) -> bool { * self == CH_LEN_A :: BIT_LEN8 } # [doc = "16-bit"] # [inline (always)] pub fn is_bit_len16 (& self) -> bool { * self == CH_LEN_A :: BIT_LEN16 } # [doc = "18-bit"] # [inline (always)] pub fn is_bit_len18 (& self) -> bool { * self == CH_LEN_A :: BIT_LEN18 } # [doc = "20-bit"] # [inline (always)] pub fn is_bit_len20 (& self) -> bool { * self == CH_LEN_A :: BIT_LEN20 } # [doc = "24-bit"] # [inline (always)] pub fn is_bit_len24 (& self) -> bool { * self == CH_LEN_A :: BIT_LEN24 } # [doc = "32-bit"] # [inline (always)] pub fn is_bit_len32 (& self) -> bool { * self == CH_LEN_A :: BIT_LEN32 } } # [doc = "Field `CH_LEN` writer - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHLEN)"] pub type CH_LEN_W < 'a , REG > = crate :: FieldWriter < 'a , REG , 3 , CH_LEN_A > ; impl < 'a , REG > CH_LEN_W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > , { # [doc = "8-bit"] # [inline (always)] pub fn bit_len8 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_LEN_A :: BIT_LEN8) } # [doc = "16-bit"] # [inline (always)] pub fn bit_len16 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_LEN_A :: BIT_LEN16) } # [doc = "18-bit"] # [inline (always)] pub fn bit_len18 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_LEN_A :: BIT_LEN18) } # [doc = "20-bit"] # [inline (always)] pub fn bit_len20 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_LEN_A :: BIT_LEN20) } # [doc = "24-bit"] # [inline (always)] pub fn bit_len24 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_LEN_A :: BIT_LEN24) } # [doc = "32-bit"] # [inline (always)] pub fn bit_len32 (self) -> & 'a mut crate :: W < REG > { self . variant (CH_LEN_A :: BIT_LEN32) } } # [doc = "Field `WORD_LEN` reader - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.TX_CFG.TX_IWL)"] pub type WORD_LEN_R = crate :: FieldReader < WORD_LEN_A > ; # [doc = "Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.TX_CFG.TX_IWL)\n\nValue on reset: 4"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum WORD_LEN_A { # [doc = "0: 8-bit"] BIT_LEN8 = 0 , # [doc = "1: 16-bit"] BIT_LEN16 = 1 , # [doc = "2: 18-bit"] BIT_LEN18 = 2 , # [doc = "3: 20-bit"] BIT_LEN20 = 3 , # [doc = "4: 24-bit"] BIT_LEN24 = 4 , # [doc = "5: 32-bit"] BIT_LEN32 = 5 , } impl From < WORD_LEN_A > for u8 { # [inline (always)] fn from (variant : WORD_LEN_A) -> Self { variant as _ } } impl crate :: FieldSpec for WORD_LEN_A { type Ux = u8 ; } impl WORD_LEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < WORD_LEN_A > { match self . bits { 0 => Some (WORD_LEN_A :: BIT_LEN8) , 1 => Some (WORD_LEN_A :: BIT_LEN16) , 2 => Some (WORD_LEN_A :: BIT_LEN18) , 3 => Some (WORD_LEN_A :: BIT_LEN20) , 4 => Some (WORD_LEN_A :: BIT_LEN24) , 5 => Some (WORD_LEN_A :: BIT_LEN32) , _ => None , } } # [doc = "8-bit"] # [inline (always)] pub fn is_bit_len8 (& self) -> bool { * self == WORD_LEN_A :: BIT_LEN8 } # [doc = "16-bit"] # [inline (always)] pub fn is_bit_len16 (& self) -> bool { * self == WORD_LEN_A :: BIT_LEN16 } # [doc = "18-bit"] # [inline (always)] pub fn is_bit_len18 (& self) -> bool { * self == WORD_LEN_A :: BIT_LEN18 } # [doc = "20-bit"] # [inline (always)] pub fn is_bit_len20 (& self) -> bool { * self == WORD_LEN_A :: BIT_LEN20 } # [doc = "24-bit"] # [inline (always)] pub fn is_bit_len24 (& self) -> bool { * self == WORD_LEN_A :: BIT_LEN24 } # [doc = "32-bit"] # [inline (always)] pub fn is_bit_len32 (& self) -> bool { * self == WORD_LEN_A :: BIT_LEN32 } } # [doc = "Field `WORD_LEN` writer - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.TX_CFG.TX_IWL)"] pub type WORD_LEN_W < 'a , REG > = crate :: FieldWriter < 'a , REG , 3 , WORD_LEN_A > ; impl < 'a , REG > WORD_LEN_W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > , { # [doc = "8-bit"] # [inline (always)] pub fn bit_len8 (self) -> & 'a mut crate :: W < REG > { self . variant (WORD_LEN_A :: BIT_LEN8) } # [doc = "16-bit"] # [inline (always)] pub fn bit_len16 (self) -> & 'a mut crate :: W < REG > { self . variant (WORD_LEN_A :: BIT_LEN16) } # [doc = "18-bit"] # [inline (always)] pub fn bit_len18 (self) -> & 'a mut crate :: W < REG > { self . variant (WORD_LEN_A :: BIT_LEN18) } # [doc = "20-bit"] # [inline (always)] pub fn bit_len20 (self) -> & 'a mut crate :: W < REG > { self . variant (WORD_LEN_A :: BIT_LEN20) } # [doc = "24-bit"] # [inline (always)] pub fn bit_len24 (self) -> & 'a mut crate :: W < REG > { self . variant (WORD_LEN_A :: BIT_LEN24) } # [doc = "32-bit"] # [inline (always)] pub fn bit_len32 (self) -> & 'a mut crate :: W < REG > { self . variant (WORD_LEN_A :: BIT_LEN32) } } # [doc = "Field `SCKO_POL` reader - TX master bit clock polarity. When this bit is 1, the outgoing tx_sck signal is inverted after it has been transmitted from the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. '0': When transmitter is in master mode, serial data is transmitted from the falling bit clock edge '1': When transmitter is in master mode, serial data is transmitted from the rising bit clock edge"] pub type SCKO_POL_R = crate :: BitReader ; # [doc = "Field `SCKO_POL` writer - TX master bit clock polarity. When this bit is 1, the outgoing tx_sck signal is inverted after it has been transmitted from the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. '0': When transmitter is in master mode, serial data is transmitted from the falling bit clock edge '1': When transmitter is in master mode, serial data is transmitted from the rising bit clock edge"] pub type SCKO_POL_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `SCKI_POL` reader - TX slave bit clock polarity. When this bit is 1, the incoming tx_sck signal is inverted before it is received by the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. See TX_CTL.B_CLOCK_INV for more details."] pub type SCKI_POL_R = crate :: BitReader ; # [doc = "Field `SCKI_POL` writer - TX slave bit clock polarity. When this bit is 1, the incoming tx_sck signal is inverted before it is received by the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. See TX_CTL.B_CLOCK_INV for more details."] pub type SCKI_POL_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Bit 3 - Serial data transmission is advanced by 0.5 SCK cycles. This bit is valid only in TX slave mode. When set to '1', the serial data will be transmitted 0.5 SCK cycles earlier than when set to '0'. 1) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK falling edge 2) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK rising edge that is 0.5 SCK cycles before the SCK falling edge in 1) 3) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK rising edge 4) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK falling edge that is 0.5 SCK cycles before the SCK rising edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual timing is generated by an internal clock that runs 8x the SCK frequency). The word sync (TX_WS) signal is not affected by this bit setting. Note: When Master mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.TX_BCLKINV)"] # [inline (always)] pub fn b_clock_inv (& self) -> B_CLOCK_INV_R { B_CLOCK_INV_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bits 4:6 - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHSET)"] # [inline (always)] pub fn ch_nr (& self) -> CH_NR_R { CH_NR_R :: new (((self . bits >> 4) & 7) as u8) } # [doc = "Bit 7 - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_MS)"] # [inline (always)] pub fn ms (& self) -> MS_R { MS_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bits 8:9 - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.TX_CFG.TX_I2S_MODE)"] # [inline (always)] pub fn i2s_mode (& self) -> I2S_MODE_R { I2S_MODE_R :: new (((self . bits >> 8) & 3) as u8) } # [doc = "Bit 10 - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_WS_PULSE) Note: When not TDM mode, must be '1'."] # [inline (always)] pub fn ws_pulse (& self) -> WS_PULSE_R { WS_PULSE_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 12 - Set overhead value: '0': Set to '0' '1': Set to '1' (Note: This bit is connected to AR38U12.TX_CFG.TX_OVHDATA)"] # [inline (always)] pub fn ovhdata (& self) -> OVHDATA_R { OVHDATA_R :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - Set watchdog for 'tx_ws_in': '0': Disabled. '1': Enabled."] # [inline (always)] pub fn wd_en (& self) -> WD_EN_R { WD_EN_R :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bits 16:18 - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHLEN)"] # [inline (always)] pub fn ch_len (& self) -> CH_LEN_R { CH_LEN_R :: new (((self . bits >> 16) & 7) as u8) } # [doc = "Bits 20:22 - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.TX_CFG.TX_IWL)"] # [inline (always)] pub fn word_len (& self) -> WORD_LEN_R { WORD_LEN_R :: new (((self . bits >> 20) & 7) as u8) } # [doc = "Bit 24 - TX master bit clock polarity. When this bit is 1, the outgoing tx_sck signal is inverted after it has been transmitted from the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. '0': When transmitter is in master mode, serial data is transmitted from the falling bit clock edge '1': When transmitter is in master mode, serial data is transmitted from the rising bit clock edge"] # [inline (always)] pub fn scko_pol (& self) -> SCKO_POL_R { SCKO_POL_R :: new (((self . bits >> 24) & 1) != 0) } # [doc = "Bit 25 - TX slave bit clock polarity. When this bit is 1, the incoming tx_sck signal is inverted before it is received by the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. See TX_CTL.B_CLOCK_INV for more details."] # [inline (always)] pub fn scki_pol (& self) -> SCKI_POL_R { SCKI_POL_R :: new (((self . bits >> 25) & 1) != 0) } } impl W { # [doc = "Bit 3 - Serial data transmission is advanced by 0.5 SCK cycles. This bit is valid only in TX slave mode. When set to '1', the serial data will be transmitted 0.5 SCK cycles earlier than when set to '0'. 1) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK falling edge 2) TX_CTL.SCKI_POL=0 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK rising edge that is 0.5 SCK cycles before the SCK falling edge in 1) 3) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=0: Serial data will be transmitted off the SCK rising edge 4) TX_CTL.SCKI_POL=1 and TX_CTL.B_CLOCK_INV=1: Serial data will be transmitted off the SCK falling edge that is 0.5 SCK cycles before the SCK rising edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual timing is generated by an internal clock that runs 8x the SCK frequency). The word sync (TX_WS) signal is not affected by this bit setting. Note: When Master mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.TX_BCLKINV)"] # [inline (always)] # [must_use] pub fn b_clock_inv (& mut self) -> B_CLOCK_INV_W < TX_CTL_SPEC > { B_CLOCK_INV_W :: new (self , 3) } # [doc = "Bits 4:6 - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHSET)"] # [inline (always)] # [must_use] pub fn ch_nr (& mut self) -> CH_NR_W < TX_CTL_SPEC > { CH_NR_W :: new (self , 4) } # [doc = "Bit 7 - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_MS)"] # [inline (always)] # [must_use] pub fn ms (& mut self) -> MS_W < TX_CTL_SPEC > { MS_W :: new (self , 7) } # [doc = "Bits 8:9 - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.TX_CFG.TX_I2S_MODE)"] # [inline (always)] # [must_use] pub fn i2s_mode (& mut self) -> I2S_MODE_W < TX_CTL_SPEC > { I2S_MODE_W :: new (self , 8) } # [doc = "Bit 10 - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.TX_CFG.TX_WS_PULSE) Note: When not TDM mode, must be '1'."] # [inline (always)] # [must_use] pub fn ws_pulse (& mut self) -> WS_PULSE_W < TX_CTL_SPEC > { WS_PULSE_W :: new (self , 10) } # [doc = "Bit 12 - Set overhead value: '0': Set to '0' '1': Set to '1' (Note: This bit is connected to AR38U12.TX_CFG.TX_OVHDATA)"] # [inline (always)] # [must_use] pub fn ovhdata (& mut self) -> OVHDATA_W < TX_CTL_SPEC > { OVHDATA_W :: new (self , 12) } # [doc = "Bit 13 - Set watchdog for 'tx_ws_in': '0': Disabled. '1': Enabled."] # [inline (always)] # [must_use] pub fn wd_en (& mut self) -> WD_EN_W < TX_CTL_SPEC > { WD_EN_W :: new (self , 13) } # [doc = "Bits 16:18 - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.TX_CFG.TX_CHLEN)"] # [inline (always)] # [must_use] pub fn ch_len (& mut self) -> CH_LEN_W < TX_CTL_SPEC > { CH_LEN_W :: new (self , 16) } # [doc = "Bits 20:22 - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.TX_CFG.TX_IWL)"] # [inline (always)] # [must_use] pub fn word_len (& mut self) -> WORD_LEN_W < TX_CTL_SPEC > { WORD_LEN_W :: new (self , 20) } # [doc = "Bit 24 - TX master bit clock polarity. When this bit is 1, the outgoing tx_sck signal is inverted after it has been transmitted from the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. '0': When transmitter is in master mode, serial data is transmitted from the falling bit clock edge '1': When transmitter is in master mode, serial data is transmitted from the rising bit clock edge"] # [inline (always)] # [must_use] pub fn scko_pol (& mut self) -> SCKO_POL_W < TX_CTL_SPEC > { SCKO_POL_W :: new (self , 24) } # [doc = "Bit 25 - TX slave bit clock polarity. When this bit is 1, the incoming tx_sck signal is inverted before it is received by the I2S transceiver core. This bit does not affect the internal serial data transmission timing. The word sync (TX_WS) signal is not affected by this bit setting. See TX_CTL.B_CLOCK_INV for more details."] # [inline (always)] # [must_use] pub fn scki_pol (& mut self) -> SCKI_POL_W < TX_CTL_SPEC > { SCKI_POL_W :: new (self , 25) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Transmitter control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct TX_CTL_SPEC ; impl crate :: RegisterSpec for TX_CTL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`tx_ctl::R`](R) reader structure"] impl crate :: Readable for TX_CTL_SPEC { } # [doc = "`write(|w| ..)` method takes [`tx_ctl::W`](W) writer structure"] impl crate :: Writable for TX_CTL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets TX_CTL to value 0x0044_0510"] impl crate :: Resettable for TX_CTL_SPEC { const RESET_VALUE : Self :: Ux = 0x0044_0510 ; }