// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const REQUEST_SENT: u8 = 0;
pub const REQUEST_RECEIVED: u8 = 1;
pub const RESPONSE_SENT: u8 = 2;
pub const RESPONSE_RECEIVED: u8 = 3;

extern "C" {
    fn service_msgs__msg__ServiceEventInfo__init(msg: *mut ServiceEventInfo) -> bool;
    fn service_msgs__msg__ServiceEventInfo__fini(msg: *mut ServiceEventInfo);
    fn service_msgs__msg__ServiceEventInfo__are_equal(lhs: *const ServiceEventInfo, rhs: *const ServiceEventInfo) -> bool;
    fn service_msgs__msg__ServiceEventInfo__Sequence__init(msg: *mut ServiceEventInfoSeqRaw, size: usize) -> bool;
    fn service_msgs__msg__ServiceEventInfo__Sequence__fini(msg: *mut ServiceEventInfoSeqRaw);
    fn service_msgs__msg__ServiceEventInfo__Sequence__are_equal(lhs: *const ServiceEventInfoSeqRaw, rhs: *const ServiceEventInfoSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__service_msgs__msg__ServiceEventInfo() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct ServiceEventInfo {
    pub event_type: u8,
    pub stamp: builtin_interfaces::UnsafeTime,
    pub client_gid: [i8; 16],
    pub sequence_number: i64,
}

impl ServiceEventInfo {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { service_msgs__msg__ServiceEventInfo__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ServiceEventInfo {
    fn drop(&mut self) {
        unsafe { service_msgs__msg__ServiceEventInfo__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ServiceEventInfoSeqRaw {
    data: *mut ServiceEventInfo,
    size: usize,
    capacity: usize,
}

/// Sequence of ServiceEventInfo.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ServiceEventInfoSeq<const N: usize> {
    data: *mut ServiceEventInfo,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ServiceEventInfoSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ServiceEventInfoSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { service_msgs__msg__ServiceEventInfo__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: ServiceEventInfoSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[ServiceEventInfo] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ServiceEventInfo] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ServiceEventInfo> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ServiceEventInfo> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for ServiceEventInfoSeq<N> {
    fn drop(&mut self) {
        let mut msg = ServiceEventInfoSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { service_msgs__msg__ServiceEventInfo__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ServiceEventInfoSeq<N> {}
unsafe impl<const N: usize> Sync for ServiceEventInfoSeq<N> {}


impl TypeSupport for ServiceEventInfo {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__service_msgs__msg__ServiceEventInfo()
        }
    }
}

impl PartialEq for ServiceEventInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            service_msgs__msg__ServiceEventInfo__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for ServiceEventInfoSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = ServiceEventInfoSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = ServiceEventInfoSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            service_msgs__msg__ServiceEventInfo__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

