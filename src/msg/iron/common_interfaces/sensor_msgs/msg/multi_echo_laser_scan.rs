// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__MultiEchoLaserScan__init(msg: *mut MultiEchoLaserScan) -> bool;
    fn sensor_msgs__msg__MultiEchoLaserScan__fini(msg: *mut MultiEchoLaserScan);
    fn sensor_msgs__msg__MultiEchoLaserScan__are_equal(lhs: *const MultiEchoLaserScan, rhs: *const MultiEchoLaserScan) -> bool;
    fn sensor_msgs__msg__MultiEchoLaserScan__Sequence__init(msg: *mut MultiEchoLaserScanSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__MultiEchoLaserScan__Sequence__fini(msg: *mut MultiEchoLaserScanSeqRaw);
    fn sensor_msgs__msg__MultiEchoLaserScan__Sequence__are_equal(lhs: *const MultiEchoLaserScanSeqRaw, rhs: *const MultiEchoLaserScanSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MultiEchoLaserScan() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MultiEchoLaserScan {
    pub header: std_msgs::msg::Header,
    pub angle_min: f32,
    pub angle_max: f32,
    pub angle_increment: f32,
    pub time_increment: f32,
    pub scan_time: f32,
    pub range_min: f32,
    pub range_max: f32,
    pub ranges: LaserEchoSeq<0>,
    pub intensities: LaserEchoSeq<0>,
}

impl MultiEchoLaserScan {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MultiEchoLaserScan__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MultiEchoLaserScan {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__MultiEchoLaserScan__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MultiEchoLaserScanSeqRaw {
    data: *mut MultiEchoLaserScan,
    size: usize,
    capacity: usize,
}

/// Sequence of MultiEchoLaserScan.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MultiEchoLaserScanSeq<const N: usize> {
    data: *mut MultiEchoLaserScan,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MultiEchoLaserScanSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MultiEchoLaserScanSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MultiEchoLaserScan__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: MultiEchoLaserScanSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[MultiEchoLaserScan] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MultiEchoLaserScan] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MultiEchoLaserScan> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MultiEchoLaserScan> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MultiEchoLaserScanSeq<N> {
    fn drop(&mut self) {
        let mut msg = MultiEchoLaserScanSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__MultiEchoLaserScan__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MultiEchoLaserScanSeq<N> {}
unsafe impl<const N: usize> Sync for MultiEchoLaserScanSeq<N> {}


impl TypeSupport for MultiEchoLaserScan {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MultiEchoLaserScan()
        }
    }
}

impl PartialEq for MultiEchoLaserScan {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__MultiEchoLaserScan__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for MultiEchoLaserScanSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MultiEchoLaserScanSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = MultiEchoLaserScanSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__MultiEchoLaserScan__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

