// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn type_description_interfaces__msg__TypeDescription__init(msg: *mut TypeDescription) -> bool;
    fn type_description_interfaces__msg__TypeDescription__fini(msg: *mut TypeDescription);
    fn type_description_interfaces__msg__TypeDescription__are_equal(lhs: *const TypeDescription, rhs: *const TypeDescription) -> bool;
    fn type_description_interfaces__msg__TypeDescription__Sequence__init(msg: *mut TypeDescriptionSeqRaw, size: usize) -> bool;
    fn type_description_interfaces__msg__TypeDescription__Sequence__fini(msg: *mut TypeDescriptionSeqRaw);
    fn type_description_interfaces__msg__TypeDescription__Sequence__are_equal(lhs: *const TypeDescriptionSeqRaw, rhs: *const TypeDescriptionSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__msg__TypeDescription() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct TypeDescription {
    pub type_description: IndividualTypeDescription,
    pub referenced_type_descriptions: IndividualTypeDescriptionSeq<0>,
}

impl TypeDescription {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { type_description_interfaces__msg__TypeDescription__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TypeDescription {
    fn drop(&mut self) {
        unsafe { type_description_interfaces__msg__TypeDescription__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct TypeDescriptionSeqRaw {
    data: *mut TypeDescription,
    size: usize,
    capacity: usize,
}

/// Sequence of TypeDescription.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptionSeq<const N: usize> {
    data: *mut TypeDescription,
    size: usize,
    capacity: usize,
}

impl<const N: usize> TypeDescriptionSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: TypeDescriptionSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { type_description_interfaces__msg__TypeDescription__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: TypeDescriptionSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[TypeDescription] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [TypeDescription] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, TypeDescription> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, TypeDescription> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for TypeDescriptionSeq<N> {
    fn drop(&mut self) {
        let mut msg = TypeDescriptionSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { type_description_interfaces__msg__TypeDescription__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for TypeDescriptionSeq<N> {}
unsafe impl<const N: usize> Sync for TypeDescriptionSeq<N> {}


impl TypeSupport for TypeDescription {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__msg__TypeDescription()
        }
    }
}

impl PartialEq for TypeDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            type_description_interfaces__msg__TypeDescription__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for TypeDescriptionSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = TypeDescriptionSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = TypeDescriptionSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            type_description_interfaces__msg__TypeDescription__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

