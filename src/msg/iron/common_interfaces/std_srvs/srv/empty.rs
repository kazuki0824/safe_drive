// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn std_srvs__srv__Empty_Request__init(msg: *mut EmptyRequest) -> bool;
    fn std_srvs__srv__Empty_Request__fini(msg: *mut EmptyRequest);
    fn std_srvs__srv__Empty_Request__Sequence__init(msg: *mut EmptyRequestSeqRaw, size: usize) -> bool;
    fn std_srvs__srv__Empty_Request__Sequence__fini(msg: *mut EmptyRequestSeqRaw);
    fn std_srvs__srv__Empty_Response__init(msg: *mut EmptyResponse) -> bool;
    fn std_srvs__srv__Empty_Response__fini(msg: *mut EmptyResponse);
    fn std_srvs__srv__Empty_Response__Sequence__init(msg: *mut EmptyResponseSeqRaw, size: usize) -> bool;
    fn std_srvs__srv__Empty_Response__Sequence__fini(msg: *mut EmptyResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Empty() -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Empty_Request() -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Empty_Response() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct EmptyRequest {
    _unused: u8
}

#[repr(C)]
#[derive(Debug)]
pub struct EmptyResponse {
    _unused: u8
}

impl EmptyRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Empty_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for EmptyRequest {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Empty_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct EmptyRequestSeqRaw {
    data: *mut EmptyRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of EmptyRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct EmptyRequestSeq<const N: usize> {
    data: *mut EmptyRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> EmptyRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: EmptyRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Empty_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: EmptyRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[EmptyRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [EmptyRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, EmptyRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, EmptyRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for EmptyRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = EmptyRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_srvs__srv__Empty_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for EmptyRequestSeq<N> {}
unsafe impl<const N: usize> Sync for EmptyRequestSeq<N> {}


impl EmptyResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Empty_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for EmptyResponse {
    fn drop(&mut self) {
        unsafe { std_srvs__srv__Empty_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct EmptyResponseSeqRaw {
    data: *mut EmptyResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of EmptyResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct EmptyResponseSeq<const N: usize> {
    data: *mut EmptyResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> EmptyResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: EmptyResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_srvs__srv__Empty_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: EmptyResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[EmptyResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [EmptyResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, EmptyResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, EmptyResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for EmptyResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = EmptyResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_srvs__srv__Empty_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for EmptyResponseSeq<N> {}
unsafe impl<const N: usize> Sync for EmptyResponseSeq<N> {}


pub struct Empty;

impl ServiceMsg for Empty {
    type Request = EmptyRequest;
    type Response = EmptyResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__std_srvs__srv__Empty()
        }
    }
}

impl TypeSupport for EmptyRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Empty_Request()
        }
    }
}

impl TypeSupport for EmptyResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_srvs__srv__Empty_Response()
        }
    }
}

