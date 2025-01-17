// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;
pub const DEPTH_RECURSIVE: u64 = 0;

extern "C" {
    fn rcl_interfaces__srv__ListParameters_Request__init(msg: *mut ListParametersRequest) -> bool;
    fn rcl_interfaces__srv__ListParameters_Request__fini(msg: *mut ListParametersRequest);
    fn rcl_interfaces__srv__ListParameters_Request__Sequence__init(msg: *mut ListParametersRequestSeqRaw, size: usize) -> bool;
    fn rcl_interfaces__srv__ListParameters_Request__Sequence__fini(msg: *mut ListParametersRequestSeqRaw);
    fn rcl_interfaces__srv__ListParameters_Response__init(msg: *mut ListParametersResponse) -> bool;
    fn rcl_interfaces__srv__ListParameters_Response__fini(msg: *mut ListParametersResponse);
    fn rcl_interfaces__srv__ListParameters_Response__Sequence__init(msg: *mut ListParametersResponseSeqRaw, size: usize) -> bool;
    fn rcl_interfaces__srv__ListParameters_Response__Sequence__fini(msg: *mut ListParametersResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__ListParameters() -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__ListParameters_Request() -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__ListParameters_Response() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct ListParametersRequest {
    pub prefixes: crate::msg::RosStringSeq<0, 0>,
    pub depth: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct ListParametersResponse {
    pub result: ListParametersResult,
}

impl ListParametersRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__ListParameters_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ListParametersRequest {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__srv__ListParameters_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ListParametersRequestSeqRaw {
    data: *mut ListParametersRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of ListParametersRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ListParametersRequestSeq<const N: usize> {
    data: *mut ListParametersRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ListParametersRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ListParametersRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__ListParameters_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: ListParametersRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[ListParametersRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ListParametersRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ListParametersRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ListParametersRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for ListParametersRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = ListParametersRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { rcl_interfaces__srv__ListParameters_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ListParametersRequestSeq<N> {}
unsafe impl<const N: usize> Sync for ListParametersRequestSeq<N> {}


impl ListParametersResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__ListParameters_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ListParametersResponse {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__srv__ListParameters_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ListParametersResponseSeqRaw {
    data: *mut ListParametersResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of ListParametersResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ListParametersResponseSeq<const N: usize> {
    data: *mut ListParametersResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ListParametersResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ListParametersResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__ListParameters_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: ListParametersResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[ListParametersResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ListParametersResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ListParametersResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ListParametersResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for ListParametersResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = ListParametersResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { rcl_interfaces__srv__ListParameters_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ListParametersResponseSeq<N> {}
unsafe impl<const N: usize> Sync for ListParametersResponseSeq<N> {}


pub struct ListParameters;

impl ServiceMsg for ListParameters {
    type Request = ListParametersRequest;
    type Response = ListParametersResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__ListParameters()
        }
    }
}

impl TypeSupport for ListParametersRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__ListParameters_Request()
        }
    }
}

impl TypeSupport for ListParametersResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__ListParameters_Response()
        }
    }
}

