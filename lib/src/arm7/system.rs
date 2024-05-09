use core::panic;

#[repr(C)]
#[repr(packed)]
pub struct PersonalData {
    ///	??? (0x05 0x00). (version according to gbatek)
    pub reserved_0: [u8; 2],

    ///The user's theme color (0-15).
    pub theme: u8,
    ///The user's birth month (1-12).
    pub birth_month: u8,
    ///The user's birth day (1-31).
    pub birth_day: u8,

    ///	???
    pub reserved_1: u8,

    ///The user's name in UTF-16 format.
    pub name: [i16; 10],
    ///The length of the user's name in characters.   
    pub name_length: u16,

    ///The user's message.
    pub message: [i16; 26],
    ///The length of the user's message in characters.
    pub message_length: u16,

    ///What hour the alarm clock is set to (0-23).
    pub alarm_hour: u8,
    ///What minute the alarm clock is set to (0-59).
    pub alarm_minute: u8,

    ///??? 0x02FFFCD4
    pub reserved_2: [u8; 4],

    ///	Touchscreen calibration: first X touch
    pub calibration_x1: u16,
    ///	Touchscreen calibration: first Y touch
    pub calibration_y1: u16,
    ///	Touchscreen calibration: first X touch pixel
    pub calibration_1px: u8,
    ///	Touchscreen calibration: first X touch pixel
    pub calibration_1py: u8,

    ///	Touchscreen calibration: second X touch
    pub calibration_x2: u16,
    ///	Touchscreen calibration: second Y touch
    pub calibration_y2: u16,
    ///	Touchscreen calibration: second X touch pixel
    pub calibration_2px: u8,
    ///	Touchscreen calibration: second X touch pixel
    pub calibration_2py: u8,

    pub sys_data: SystemData,

    ///???
    pub reserved_3: u16,
    ///Real Time Clock offset.
    pub rtc_offset: u32,
    ///???
    pub reserved_4: u32,
}

pub struct SystemDataRepresentation {
    pub language: usize,
    ///GBA screen selection (lower screen if set, otherwise upper screen).
    pub gba_screen: usize,
    ///Brightness level at power on, dslite.
    pub default_brightness: usize,
    ///The DS should boot from the DS cart or GBA cart automatically if one is inserted.
    pub auto_mode: usize,
    ///???
    pub reserved_5: usize,
    ///User Settings Lost (0=Normal, 1=Prompt/Settings Lost)
    pub settings_lost: usize,
    ///???
    pub reserved_6: usize,
}

#[derive(Clone, Copy)]
pub struct SystemData(u16);

impl From<u16> for SystemData {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Into<u16> for SystemData {
    fn into(self) -> u16 {
        self.0
    }
}

fn mask(data: usize, bit_count: usize, offset: usize) -> usize {
    let mask = (1 << bit_count) - 1;
    (data & (mask << offset)) >> offset
}

impl From<SystemData> for SystemDataRepresentation {
    fn from(value: SystemData) -> Self {
        let data = value.0 as usize;
        SystemDataRepresentation {
            language: mask(data, 3, 0),
            gba_screen: mask(data, 1, 3),
            default_brightness: mask(data, 2, 4),
            auto_mode: mask(data, 1, 6),
            reserved_5: mask(data, 2, 7),
            settings_lost: mask(data, 1, 9),
            reserved_6: mask(data, 6, 10),
        }
    }
}

impl From<SystemDataRepresentation> for SystemData {
    fn from(value: SystemDataRepresentation) -> Self {
        {
            let bad_lang = value.language >= (1 << 3);
            let bad_screen = value.gba_screen >= (1 << 1);
            let bad_brightness = value.default_brightness >= (1 << 2);
            let bad_amode = value.auto_mode >= (1 << 1);
            let bad_res5 = value.reserved_5 >= (1 << 2);
            let bad_lost = value.settings_lost >= (1 << 1);
            let bad_res6 = value.reserved_6 >= (1 << 6);

            if bad_lang
                || bad_screen
                || bad_brightness
                || bad_amode
                || bad_lost
                || bad_res5
                || bad_res6
            {
                panic!("Bad SystemData");
            }
        }
        let data = value.language << 0
            | value.gba_screen << 4
            | value.default_brightness << 6
            | value.reserved_5 << 7
            | value.settings_lost << 9
            | value.reserved_6 << 10;
        Self(data as u16)
    }
}
