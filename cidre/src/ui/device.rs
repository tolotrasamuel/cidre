use crate::{define_cls, define_obj_type, ns, objc};

#[doc(alias = "UIDeviceOrientation")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum Orientation {
    Unknown,
    /// Device oriented vertically, home button on the bottom
    Portrait,
    /// Device oriented vertically, home button on the top
    PortraitUpsideDown,
    /// Device oriented horizontally, home button on the right
    LandscapeLeft,
    /// Device oriented horizontally, home button on the left
    LandscapeRight,
    /// Device oriented flat, face up
    FaceUp,
    /// Device oriented flat, face down
    FaceDown,
}

impl Orientation {
    #[inline]
    pub fn is_portrait(&self) -> bool {
        *self == Self::Portrait || *self == Self::PortraitUpsideDown
    }

    #[inline]
    pub fn is_landscape(&self) -> bool {
        *self == Self::LandscapeLeft || *self == Self::LandscapeRight
    }

    #[inline]
    pub fn is_flat(&self) -> bool {
        *self == Self::FaceUp || *self == Self::FaceDown
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum BatteryState {
    Unknown,
    /// on battery, discharging
    Unplugged,
    /// plugged in, less than 100%
    Charging,
    /// plugged in, at 100%
    Full,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum UserInterfaceIdiom {
    Unspecified = -1,
    /// iPhone and iPod touch style UI
    Phone,
    /// iPad style UI
    Pad,
    /// Apple TV style UI
    Tv,
    /// CarPlay style UI
    CarPlay,
    /// Optimized for Mac UI
    Mac = 5,
}

pub mod notifications {
    use crate::ns;

    #[inline]
    pub fn orientation_did_change() -> &'static ns::NotificationName {
        unsafe { UIDeviceOrientationDidChangeNotification }
    }

    #[inline]
    pub fn battery_state_did_change() -> &'static ns::NotificationName {
        unsafe { UIDeviceBatteryStateDidChangeNotification }
    }

    #[inline]
    pub fn battery_level_did_change() -> &'static ns::NotificationName {
        unsafe { UIDeviceBatteryLevelDidChangeNotification }
    }

    #[inline]
    pub fn proximity_state_did_change() -> &'static ns::NotificationName {
        unsafe { UIDeviceProximityStateDidChangeNotification }
    }

    #[link(name = "UIKit", kind = "framework")]
    extern "C" {
        static UIDeviceOrientationDidChangeNotification: &'static ns::NotificationName;
        static UIDeviceBatteryStateDidChangeNotification: &'static ns::NotificationName;
        static UIDeviceBatteryLevelDidChangeNotification: &'static ns::NotificationName;
        static UIDeviceProximityStateDidChangeNotification: &'static ns::NotificationName;
    }
}

define_obj_type!(pub Device(ns::Id));

impl Device {
    define_cls!(UI_DEVICE);

    #[objc::msg_send2(currentDevice)]
    pub fn current() -> &'static Device;

    #[objc::msg_send2(userInterfaceIdiom)]
    pub fn user_interface_idiom(&self) -> UserInterfaceIdiom;

    #[objc::msg_send2(isMultitaskingSupported)]
    pub fn is_multitasking_supported(&self) -> bool;

    #[objc::msg_send2(proximityState)]
    pub fn proximity_state(&self) -> bool;

    #[objc::msg_send2(isProximityMonitoringEnabled)]
    pub fn is_proximity_monitoring_enabled(&self) -> bool;

    #[objc::msg_send2(setProximityMonitoringEnabled:)]
    pub fn set_proximity_monitoring_enabled(&self, val: bool);

    #[objc::msg_send2(batteryLevel)]
    pub fn battery_level(&self) -> f32;

    #[objc::msg_send2(batteryState)]
    pub fn battery_state(&self) -> BatteryState;

    #[objc::msg_send2(isBatteryMonitoringEnabled)]
    pub fn is_battery_monitoring_enabled(&self) -> bool;

    #[objc::msg_send2(setBatteryMonitoringEnabled:)]
    pub fn set_battery_monitoring_enabled(&self, val: bool);

    #[objc::msg_send2(identifierForVendor)]
    pub fn id_for_vendor(&self) -> Option<&ns::Uuid>;

    #[objc::msg_send2(model)]
    pub fn model(&self) -> &ns::String;

    #[objc::msg_send2(systemName)]
    pub fn system_name(&self) -> &ns::String;

    #[objc::msg_send2(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send2(systemVersion)]
    pub fn system_version(&self) -> &ns::String;

    /// Returns current device orientation. This will return Orientation::Unknown
    /// unless device orientation notifications are being generated.
    #[objc::msg_send2(orientation)]
    pub fn orientation(&self) -> Orientation;
}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_DEVICE: &'static objc::Class<Device>;
}
