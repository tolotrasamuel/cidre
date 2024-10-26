use std::ffi::c_void;

use crate::{arc, at::AudioBufListN, core_audio, os};

use super::{
    AudioObjId, AudioObjPropAddr, AudioObjPropElement, AudioObjPropScope, AudioObjPropSelector,
};

impl core_audio::AudioObjId {
    #[doc(alias = "kAudioObjectSystemObject")]
    pub const SYS_OBJECT: Self = Self(1);

    pub fn hardware_devices(&self) -> os::Result<Vec<Self>> {
        self.prop_vec::<Self>(&AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEVICES,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::WILDCARD,
        })
    }

    pub fn default_input_device(&self) -> os::Result<Self> {
        self.prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEFAULT_INPUT_DEVICE,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    pub fn default_output_device(&self) -> os::Result<Self> {
        self.prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEFAULT_OUTPUT_DEVICE,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    #[doc(alias = "AudioObjectSetPropertyData")]
    pub fn set_prop<T: Sized>(&self, address: &AudioObjPropAddr, val: &T) -> os::Result {
        unsafe {
            AudioObjectSetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                std::mem::size_of_val(val) as u32,
                val as *const _ as _,
            )
            .result()
        }
    }
    pub fn stream_cfg(&self, scope: AudioObjPropScope) -> os::Result<AudioBufListN> {
        let addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::STREAM_CFG,
            scope,
            element: AudioObjPropElement::WILDCARD,
        };
        let mut size = self.prop_size(&addr)?;
        let mut res = AudioBufListN::new(size as _);
        unsafe {
            AudioObjectGetPropertyData(
                *self,
                &addr,
                0,
                std::ptr::null(),
                &mut size,
                res.as_mut_ptr() as _,
            )
        }
        .result()?;
        Ok(res)
    }

    pub fn input_stream_cfg(&self) -> os::Result<AudioBufListN> {
        self.stream_cfg(AudioObjPropScope::INPUT)
    }

    pub fn output_stream_cfg(&self) -> os::Result<AudioBufListN> {
        self.stream_cfg(AudioObjPropScope::OUTPUT)
    }

    #[doc(alias = "AudioObjectGetPropertyDataSize")]
    pub fn prop_size(&self, address: &AudioObjPropAddr) -> os::Result<u32> {
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioObjectGetPropertyDataSize(*self, address, 0, std::ptr::null(), val.as_mut_ptr())
                .result()?;
            Ok(val.assume_init())
        }
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop<T: Sized>(&self, address: &AudioObjPropAddr) -> os::Result<T> {
        let mut data_size = std::mem::size_of::<T>() as u32;
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                val.as_mut_ptr() as _,
            )
            .result()?;
            Ok(val.assume_init())
        }
    }

    pub fn cf_prop<T: arc::Release>(&self, address: &AudioObjPropAddr) -> os::Result<arc::R<T>> {
        os::result_init(|res| unsafe {
            let mut data_size = std::mem::size_of::<arc::R<T>>() as u32;
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                res as _,
            )
        })
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop_vec<T: Sized>(&self, address: &AudioObjPropAddr) -> os::Result<Vec<T>> {
        let mut data_size = self.prop_size(address)?;
        unsafe {
            let len = (data_size as usize) / std::mem::size_of::<T>();
            if len == 0 {
                return Ok(vec![]);
            }
            let mut out = Vec::<T>::with_capacity(len);
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                out.as_mut_ptr().cast(),
            )
            .result()?;
            out.set_len(len);
            Ok(out)
        }
    }

    pub fn show(&self) {
        unsafe { AudioObjectShow(*self) }
    }
}

impl core_audio::AudioObjPropSelector {
    #[doc(alias = "kAudioHardwarePropertyProcessInputMute")]
    pub const HARDWARE_PROCESS_INPUT_MUTE: Self = Self(u32::from_be_bytes(*b"pmin"));

    /// An array of the AudioObjectIds that represent all the devices currently
    /// available to the system.
    #[doc(alias = "kAudioHardwarePropertyDevices")]
    pub const HARDWARE_DEVICES: Self = Self(u32::from_be_bytes(*b"dev#"));

    /// The AudioObjectId of the default input AudioDevice.
    #[doc(alias = "kAudioHardwarePropertyDefaultInputDevice")]
    pub const HARDWARE_DEFAULT_INPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dIn "));

    /// The AudioObjectId of the default output AudioDevice.
    #[doc(alias = "kAudioHardwarePropertyDefaultOutputDevice")]
    pub const HARDWARE_DEFAULT_OUTPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dOut"));

    /// The AudioObjectId of the output AudioDevice to use for system related sound
    /// from the alert sound to digital call progress.
    #[doc(alias = "kAudioHardwarePropertyDefaultSystemOutputDevice")]
    pub const HARDWARE_DEFAULT_SYS_OUTPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"sOut"));
}

///  AudioDevice Properties
impl core_audio::AudioObjPropSelector {
    /// An os::Status that contains any error codes generated by loading the IOAudio
    /// driver plug-in for the AudioDevice or kAudioHardwareNoError if the plug-in
    /// loaded successfully. This property only exists for IOAudio-based
    /// AudioDevices whose driver has specified a plug-in to load.
    #[doc(alias = "kAudioDevicePropertyPlugIn")]
    pub const PLUG_IN: Self = Self(u32::from_be_bytes(*b"plug"));

    /// The type of this property is a UInt32, but its value has no meaning. This
    /// property exists so that clients can listen to it and be told when the
    /// configuration of the AudioDevice has changed in ways that cannot otherwise
    /// be conveyed through other notifications. In response to this notification,
    /// clients should re-evaluate everything they need to know about the device,
    /// particularly the layout and values of the controls.
    #[doc(alias = "kAudioDevicePropertyDeviceHasChanged")]
    pub const DEVICE_HAS_CHANGED: Self = Self(u32::from_be_bytes(*b"diff"));

    /// u32 where 1 means that the AudioDevice is running in at least one
    /// process on the system and 0 means that it isn't running at all.
    #[doc(alias = "kAudioDevicePropertyDeviceIsRunningSomewhere")]
    pub const DEVICE_IS_RUNNING_SOMEWHERE: Self = Self(u32::from_be_bytes(*b"gone"));

    /// A u32 where the value has no meaning. This property exists so that
    /// clients can be notified when the AudioDevice detects that an IO cycle has
    /// run past its deadline. Note that the notification for this property is
    /// usually sent from the AudioDevice's IO thread.
    #[doc(alias = "kAudioDeviceProcessorOverload")]
    pub const PROCESSOR_OVERLOAD: Self = Self(u32::from_be_bytes(*b"over"));

    /// A u32 where the value has no meaning. This property exists so that
    /// clients can be notified when IO on the device has stopped outside of the
    /// normal mechanisms. This typically comes up when IO is stopped after
    /// AudioDeviceStart has returned successfully but prior to the notification for
    /// kAudioDevicePropertyIsRunning being sent.
    #[doc(alias = "kAudioDevicePropertyIOStoppedAbnormally")]
    pub const IO_STOPPED_ABNORMALLY: Self = Self(u32::from_be_bytes(*b"stpd"));

    /// A pid_t indicating the process that currently owns exclusive access to the
    /// AudioDevice or a value of -1 indicating that the device is currently
    /// available to all processes.

    /// If the AudioDevice is in a non-mixable mode,
    /// the HAL will automatically take hog mode on behalf of the first process to
    /// start an IOProc.
    ///
    /// Note that when setting this property, the value passed in is ignored. If
    /// another process owns exclusive access, that remains unchanged. If the
    /// current process owns exclusive access, it is released and made available to
    /// all processes again. If no process has exclusive access (meaning the current
    /// value is -1), this process gains ownership of exclusive access.  On return,
    /// the pid_t pointed to by inPropertyData will contain the new value of the
    /// property.
    #[doc(alias = "kAudioDevicePropertyHogMode")]
    pub const HOG_MODE: Self = Self(u32::from_be_bytes(*b"oink"));

    ///  A u32 whose value indicates the number of frames in the IO buffers.
    #[doc(alias = "kAudioDevicePropertyBufferFrameSize")]
    pub const BUF_FRAME_SIZE: Self = Self(u32::from_be_bytes(*b"fsiz"));

    /// An AudioValueRange indicating the minimum and maximum values, inclusive, for
    /// kAudioDevicePropertyBufferFrameSize.
    #[doc(alias = "kAudioDevicePropertyBufferFrameSizeRange")]
    pub const BUF_FRAME_SIZE_RANGE: Self = Self(u32::from_be_bytes(*b"fsz#"));

    /// A u32 that, if implemented by a device, indicates that the sizes of the
    /// buffers passed to an IOProc will vary by a small amount. The value of this
    /// property will indicate the largest buffer that will be passed and
    /// kAudioDevicePropertyBufferFrameSize will indicate the smallest buffer that
    /// will get passed to the IOProc. The usage of this property is narrowed to
    /// only allow for devices whose buffer sizes vary by small amounts greater than
    /// kAudioDevicePropertyBufferFrameSize. It is not intended to be a license for
    /// devices to be able to send buffers however they please. Rather, it is
    /// intended to allow for hardware whose natural rhythms lead to this necessity.
    #[doc(alias = "kAudioDevicePropertyUsesVariableBufferFrameSizes")]
    pub const USES_VARIABLE_BUF_FRAME_SIZES: Self = Self(u32::from_be_bytes(*b"vfsz"));

    /// A f32 whose range is from 0 to 1. This value indicates how much of the
    /// client portion of the IO cycle the process will use. The client portion of
    /// the IO cycle is the portion of the cycle in which the device calls the
    /// IOProcs so this property does not the apply to the duration of the entire
    /// cycle.
    #[doc(alias = "kAudioDevicePropertyIOCycleUsage")]
    pub const IO_CYCLE_USAGE: Self = Self(u32::from_be_bytes(*b"ncyc"));

    /// This property returns the stream configuration of the device in an
    /// AudioBufListN (with the buffer pointers set to NULL) which describes the
    /// list of streams and the number of channels in each stream. This corresponds
    /// to what will be passed into the IOProc.
    #[doc(alias = "kAudioDevicePropertyStreamConfiguration")]
    pub const STREAM_CFG: Self = Self(u32::from_be_bytes(*b"slay"));

    /// An AudioHardwareIOProcStreamUsage structure which details the stream usage
    /// of a given IO proc. If a stream is marked as not being used, the given
    /// IOProc will see a corresponding NULL buffer pointer in the AudioBufferList
    /// passed to its IO proc. Note that the number of streams detailed in the
    /// AudioHardwareIOProcStreamUsage must include all the streams of that
    /// direction on the device. Also, when getting the value of the property, one
    /// must fill out the mIOProc field of the AudioHardwareIOProcStreamUsage with
    /// the address of the of the IOProc whose stream usage is to be retrieved.
    #[doc(alias = "kAudioDevicePropertyIOProcStreamUsage")]
    pub const IO_PROC_STREAM_USAGE: Self = Self(u32::from_be_bytes(*b"suse"));

    /// A f64 that indicates the current actual sample rate of the AudioDevice
    /// as measured by its time stamps.
    #[doc(alias = "kAudioDevicePropertyActualSampleRate")]
    pub const ACTUAL_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"asrt"));

    /// A cf::String that contains the UID for the AudioClockDevice that is currently
    /// serving as the main time base of the device. The caller is responsible
    /// for releasing the returned cf::String.
    #[doc(alias = "kAudioDevicePropertyClockDevice")]
    pub const CLOCK_DEVICE: Self = Self(u32::from_be_bytes(*b"apcd"));

    /// An os_workgroup_t that represents the thread workgroup the AudioDevice's
    /// IO thread belongs to. The caller is responsible for releasing the returned
    /// object.
    #[doc(alias = "kAudioDevicePropertyIOThreadOSWorkgroup")]
    pub const IO_THREAD_OS_WORKGROUP: Self = Self(u32::from_be_bytes(*b"oswg"));

    /// A u32 where a non-zero value indicates that the current process's audio
    /// will be zeroed out by the system. Note that this property does not apply to
    /// aggregate devices, just real, physical devices.
    #[doc(alias = "kAudioDevicePropertyProcessMute")]
    pub const PROCESS_MUTE: Self = Self(u32::from_be_bytes(*b"appm"));
}

#[link(name = "CoreAudio", kind = "framework")]
extern "C-unwind" {

    fn AudioObjectShow(objectId: AudioObjId);

    fn AudioObjectGetPropertyData(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
        data: *mut c_void,
    ) -> os::Status;

    fn AudioObjectSetPropertyData(
        objectId: AudioObjId,
        address: &AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

    fn AudioObjectGetPropertyDataSize(
        objectId: AudioObjId,
        address: &AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
    ) -> os::Status;

}

#[cfg(test)]
mod tests {
    use crate::{
        arc, cf,
        core_audio::{
            AudioObjId, AudioObjPropAddr, AudioObjPropElement, AudioObjPropScope,
            AudioObjPropSelector,
        },
    };

    #[test]
    fn list_devices() {
        let addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEFAULT_INPUT_DEVICE,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        };
        let _device_id: AudioObjId = AudioObjId::SYS_OBJECT.prop(&addr).unwrap();

        let addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEVICES,
            scope: AudioObjPropScope::INPUT,
            element: AudioObjPropElement::MAIN,
        };
        let devices: Vec<AudioObjId> = AudioObjId::SYS_OBJECT.prop_vec(&addr).unwrap();

        assert!(!devices.is_empty());

        let name_addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::NAME,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        };
        let man_addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::MANUFACTURER,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        };
        for d in devices {
            let _val: arc::R<cf::String> = d.cf_prop(&name_addr).unwrap();
            let _val: arc::R<cf::String> = d.cf_prop(&man_addr).unwrap();
        }
    }
}
