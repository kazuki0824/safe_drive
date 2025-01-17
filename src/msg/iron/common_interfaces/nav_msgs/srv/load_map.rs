// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;
pub const RESULT_SUCCESS: u8 = 0;
pub const RESULT_MAP_DOES_NOT_EXIST: u8 = 1;
pub const RESULT_INVALID_MAP_DATA: u8 = 2;
pub const RESULT_INVALID_MAP_METADATA: u8 = 3;
pub const RESULT_UNDEFINED_FAILURE: u8 = 255;

extern "C" {
    fn nav_msgs__srv__LoadMap_Request__init(msg: *mut LoadMapRequest) -> bool;
    fn nav_msgs__srv__LoadMap_Request__fini(msg: *mut LoadMapRequest);
    fn nav_msgs__srv__LoadMap_Request__Sequence__init(msg: *mut LoadMapRequestSeqRaw, size: usize) -> bool;
    fn nav_msgs__srv__LoadMap_Request__Sequence__fini(msg: *mut LoadMapRequestSeqRaw);
    fn nav_msgs__srv__LoadMap_Response__init(msg: *mut LoadMapResponse) -> bool;
    fn nav_msgs__srv__LoadMap_Response__fini(msg: *mut LoadMapResponse);
    fn nav_msgs__srv__LoadMap_Response__Sequence__init(msg: *mut LoadMapResponseSeqRaw, size: usize) -> bool;
    fn nav_msgs__srv__LoadMap_Response__Sequence__fini(msg: *mut LoadMapResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__LoadMap() -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__LoadMap_Request() -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__LoadMap_Response() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct LoadMapRequest {
    pub map_url: crate::msg::RosString<0>,
}

#[repr(C)]
#[derive(Debug)]
pub struct LoadMapResponse {
    pub map: OccupancyGrid,
    pub result: u8,
}

impl LoadMapRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for LoadMapRequest {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__LoadMap_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct LoadMapRequestSeqRaw {
    data: *mut LoadMapRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of LoadMapRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct LoadMapRequestSeq<const N: usize> {
    data: *mut LoadMapRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> LoadMapRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: LoadMapRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: LoadMapRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[LoadMapRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [LoadMapRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, LoadMapRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, LoadMapRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for LoadMapRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = LoadMapRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { nav_msgs__srv__LoadMap_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for LoadMapRequestSeq<N> {}
unsafe impl<const N: usize> Sync for LoadMapRequestSeq<N> {}


impl LoadMapResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for LoadMapResponse {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__LoadMap_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct LoadMapResponseSeqRaw {
    data: *mut LoadMapResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of LoadMapResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct LoadMapResponseSeq<const N: usize> {
    data: *mut LoadMapResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> LoadMapResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: LoadMapResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: LoadMapResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[LoadMapResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [LoadMapResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, LoadMapResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, LoadMapResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for LoadMapResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = LoadMapResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { nav_msgs__srv__LoadMap_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for LoadMapResponseSeq<N> {}
unsafe impl<const N: usize> Sync for LoadMapResponseSeq<N> {}


pub struct LoadMap;

impl ServiceMsg for LoadMap {
    type Request = LoadMapRequest;
    type Response = LoadMapResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__LoadMap()
        }
    }
}

impl TypeSupport for LoadMapRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__LoadMap_Request()
        }
    }
}

impl TypeSupport for LoadMapResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__LoadMap_Response()
        }
    }
}

