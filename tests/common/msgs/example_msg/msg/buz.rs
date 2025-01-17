// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use safe_drive::msg::*;
use safe_drive::rcl;
use safe_drive::msg::common_interfaces::*;

extern "C" {
    fn example_msg__msg__Buz__init(msg: *mut Buz) -> bool;
    fn example_msg__msg__Buz__fini(msg: *mut Buz);
    fn example_msg__msg__Buz__are_equal(lhs: *const Buz, rhs: *const Buz) -> bool;
    fn example_msg__msg__Buz__Sequence__init(msg: *mut BuzSeqRaw, size: usize) -> bool;
    fn example_msg__msg__Buz__Sequence__fini(msg: *mut BuzSeqRaw);
    fn example_msg__msg__Buz__Sequence__are_equal(lhs: *const BuzSeqRaw, rhs: *const BuzSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__msg__Buz() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Buz {
    pub c: safe_drive::msg::RosString<0>,
}

impl Buz {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__msg__Buz__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Buz {
    fn drop(&mut self) {
        unsafe { example_msg__msg__Buz__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct BuzSeqRaw {
    data: *mut Buz,
    size: usize,
    capacity: usize,
}

/// Sequence of Buz.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct BuzSeq<const N: usize> {
    data: *mut Buz,
    size: usize,
    capacity: usize,
}

impl<const N: usize> BuzSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: BuzSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__msg__Buz__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: BuzSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[Buz] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Buz] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Buz> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Buz> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for BuzSeq<N> {
    fn drop(&mut self) {
        let mut msg = BuzSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { example_msg__msg__Buz__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for BuzSeq<N> {}
unsafe impl<const N: usize> Sync for BuzSeq<N> {}


impl TypeSupport for Buz {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__msg__Buz()
        }
    }
}

impl PartialEq for Buz {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            example_msg__msg__Buz__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for BuzSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = BuzSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = BuzSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            example_msg__msg__Buz__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

