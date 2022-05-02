use crate::{define_obj_type, objc::Id};

#[repr(isize)]
pub enum CaptureVideoOrienation {
    Portrait = 1,
    PortraitUpsideDown = 2,
    LandscapeRight = 3,
    LandscapeLeft = 4,
}

#[repr(isize)]
pub enum InterruptionReason {
    VideoDeviceNotAvailableInBackground = 1,
    AudioDeviceInUseByAnotherClient = 2,
    VideoDeviceInUseByAnotherClient = 3,
    VideoDeviceNotAvailableWithMultipleForegroundApps = 4,
    VideoDeviceNotAvailableDueToSystemPressure = 5,
}

define_obj_type!(CaptureSession(Id));
define_obj_type!(CaptureConnection(Id));
