use crate::{define_cls, define_obj_type, ns, objc};

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
    TV,
    /// CarPlay style UI
    CarPlay,
    /// Optimized for Mac UI
    Mac = 5,
}

pub mod notifications {
    use crate::cf;

    #[inline]
    pub fn orientation_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceOrientationDidChangeNotification }
    }

    #[inline]
    pub fn battery_state_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceBatteryStateDidChangeNotification }
    }

    #[inline]
    pub fn battery_level_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceBatteryLevelDidChangeNotification }
    }

    #[inline]
    pub fn proximity_state_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceProximityStateDidChangeNotification }
    }

    #[link(name = "UIKit", kind = "framework")]
    extern "C" {
        static UIDeviceOrientationDidChangeNotification: &'static cf::NotificationName;
        static UIDeviceBatteryStateDidChangeNotification: &'static cf::NotificationName;
        static UIDeviceBatteryLevelDidChangeNotification: &'static cf::NotificationName;
        static UIDeviceProximityStateDidChangeNotification: &'static cf::NotificationName;
    }
}

define_obj_type!(Device(ns::Id));

impl Device {
    define_cls!(UI_DEVICE);

    #[objc::cls_msg_send(currentDevice)]
    pub fn current() -> &'static Device;

    #[objc::msg_send(userInterfaceIdiom)]
    pub fn user_interface_idiom(&self) -> UserInterfaceIdiom;

    #[objc::msg_send(isMultitaskingSupported)]
    pub fn is_multitasking_supported(&self) -> bool;

    #[objc::msg_send(proximityState)]
    pub fn proximity_state(&self) -> bool;

    #[objc::msg_send(isProximityMonitoringEnabled)]
    pub fn is_proximity_monitoring_enabled(&self) -> bool;

    #[objc::msg_send(setProximityMonitoringEnabled:)]
    pub fn set_proximity_monitoring_enabled(&self, value: bool);

    #[objc::msg_send(batteryLevel)]
    pub fn battery_level(&self) -> f32;

    #[objc::msg_send(batteryState)]
    pub fn battery_state(&self) -> BatteryState;

    #[objc::msg_send(isBatteryMonitoringEnabled)]
    pub fn is_battery_monitoring_enabled(&self) -> bool;

    #[objc::msg_send(setBatteryMonitoringEnabled:)]
    pub fn set_battery_monitoring_enabled(&self, value: bool);

    #[objc::msg_send(identifierForVendor)]
    pub fn identifier_for_vendor(&self) -> Option<&ns::UUID>;

    #[objc::msg_send(model)]
    pub fn model(&self) -> &ns::String;

    #[objc::msg_send(systemName)]
    pub fn system_name(&self) -> &ns::String;

    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(systemVersion)]
    pub fn system_version(&self) -> &ns::String;

    /// Returns current device orientation. This will return Orientation::Unknown
    /// unless device orientation notifications are being generated.
    #[objc::msg_send(orientation)]
    pub fn orientation(&self) -> Orientation;
}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_DEVICE: &'static objc::Class<Device>;
}
