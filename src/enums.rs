use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OMTFrameType {
    None = 0,
    Metadata = 1,
    Video = 2,
    Audio = 4,
}

impl Default for OMTFrameType {
    fn default() -> Self {
        OMTFrameType::None
    }
}

impl From<u8> for OMTFrameType {
    fn from(value: u8) -> Self {
        match value {
            1 => OMTFrameType::Metadata,
            2 => OMTFrameType::Video,
            4 => OMTFrameType::Audio,
            _ => OMTFrameType::None,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OMTVideoFlags {
    None = 0,
    Interlaced = 1,
    Alpha = 2,
    PreMultiplied = 4,
    Preview = 8,
    HighBitDepth = 16,
}

// Helper to handle bitwise operations for flags if needed, or use bitflags! crate
// For now treating as u32 in structs often works, but let's provide safe helpers if needed.

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OMTColorSpace {
    Undefined = 0,
    BT601 = 601,
    BT709 = 709,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OMTPlatformType {
    Unknown = 0,
    Win32 = 1,
    MacOS = 2,
    Linux = 3,
    #[allow(non_camel_case_types)]
    iOS = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OMTCodec {
    VMX1 = 0x31584D56,
    FPA1 = 0x31415046, // Planar audio
    UYVY = 0x59565955,
    YUY2 = 0x32595559,
    BGRA = 0x41524742,
    NV12 = 0x3231564E,
    YV12 = 0x32315659,
    UYVA = 0x41565955,
    P216 = 0x36313250,
    PA16 = 0x36314150,
    H264 = 0x34363248, // "H264"
    H265 = 0x35363248, // "H265"
}

impl Into<i32> for OMTCodec {
    fn into(self) -> i32 {
        self as i32
    }
}

impl OMTCodec {
    pub fn from_wire_i32(value: i32) -> Self {
        match value as u32 {
            0x31584D56 => Self::VMX1,
            0x31415046 => Self::FPA1,
            0x59565955 => Self::UYVY,
            0x32595559 => Self::YUY2,
            0x41524742 => Self::BGRA,
            0x3231564E => Self::NV12,
            0x32315659 => Self::YV12,
            0x41565955 => Self::UYVA,
            0x36313250 => Self::P216,
            0x36314150 => Self::PA16,
            0x34363248 => Self::H264,
            0x35363248 => Self::H265,
            _ => Self::VMX1, // fallback
        }
    }

    pub fn as_wire_i32(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for OMTCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VMX1 => write!(f, "vmx1"),
            Self::FPA1 => write!(f, "fpa1"),
            Self::H264 => write!(f, "h264"),
            Self::H265 => write!(f, "h265"),
            Self::BGRA => write!(f, "bgra"),
            Self::UYVY => write!(f, "uyvy"),
            Self::YUY2 => write!(f, "yuy2"),
            Self::NV12 => write!(f, "nv12"),
            Self::YV12 => write!(f, "yv12"),
            Self::UYVA => write!(f, "uyva"),
            Self::P216 => write!(f, "p216"),
            Self::PA16 => write!(f, "pa16"),
        }
    }
}
