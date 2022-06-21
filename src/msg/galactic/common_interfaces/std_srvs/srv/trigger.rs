// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn std_srvs__srv__Trigger_Request__init(msg: *mut TriggerRequest) -> bool;
    fn std_srvs__srv__Trigger_Request__fini(msg: *mut TriggerRequest);
    fn std_srvs__srv__Trigger_Request__Sequence__init(msg: *mut TriggerRequestSeqRaw, size: usize) -> bool;
    fn std_srvs__srv__Trigger_Request__Sequence__fini(msg: *mut TriggerRequestSeqRaw);
    fn std_srvs__srv__Trigger_Response__init(msg: *mut TriggerResponse) -> bool;
    fn std_srvs__srv__Trigger_Response__fini(msg: *mut TriggerResponse);
    fn std_srvs__srv__Trigger_Response__Sequence__init(msg: *mut TriggerResponseSeqRaw, size: usize) -> bool;
    fn std_srvs__srv__Trigger_Response__Sequence__fini(msg: *mut TriggerResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Trigger() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct TriggerRequest {
    _unused: u8
}

#[repr(C)]
#[derive(Debug)]
pub struct TriggerResponse {
    pub success: bool,
    pub message: crate::msg::RosString<0>,
}

impl TriggerRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TriggerRequest {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Trigger_Request__fini(self) };
    }
}


struct TriggerRequestSeqRaw {
    data: *mut TriggerRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of TriggerRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct TriggerRequestSeq<const N: usize> {
    data: *mut TriggerRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> TriggerRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: TriggerRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[TriggerRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [TriggerRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for TriggerRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = TriggerRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_srvs__srv__Trigger_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for TriggerRequestSeq<N> {}
unsafe impl<const N: usize> Sync for TriggerRequestSeq<N> {}


impl TriggerResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TriggerResponse {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Trigger_Response__fini(self) };
    }
}


struct TriggerResponseSeqRaw {
    data: *mut TriggerResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of TriggerResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct TriggerResponseSeq<const N: usize> {
    data: *mut TriggerResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> TriggerResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: TriggerResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Trigger_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[TriggerResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [TriggerResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for TriggerResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = TriggerResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_srvs__srv__Trigger_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for TriggerResponseSeq<N> {}
unsafe impl<const N: usize> Sync for TriggerResponseSeq<N> {}


pub struct Trigger;

impl ServiceMsg for Trigger {
    type Request = TriggerRequest;
    type Response = TriggerResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Trigger()
        }
    }
}
