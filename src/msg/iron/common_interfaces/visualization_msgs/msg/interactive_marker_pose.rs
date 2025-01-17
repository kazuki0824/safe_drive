// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn visualization_msgs__msg__InteractiveMarkerPose__init(msg: *mut InteractiveMarkerPose) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerPose__fini(msg: *mut InteractiveMarkerPose);
    fn visualization_msgs__msg__InteractiveMarkerPose__are_equal(lhs: *const InteractiveMarkerPose, rhs: *const InteractiveMarkerPose) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerPose__Sequence__init(msg: *mut InteractiveMarkerPoseSeqRaw, size: usize) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerPose__Sequence__fini(msg: *mut InteractiveMarkerPoseSeqRaw);
    fn visualization_msgs__msg__InteractiveMarkerPose__Sequence__are_equal(lhs: *const InteractiveMarkerPoseSeqRaw, rhs: *const InteractiveMarkerPoseSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerPose() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerPose {
    pub header: std_msgs::msg::Header,
    pub pose: geometry_msgs::msg::Pose,
    pub name: crate::msg::RosString<0>,
}

impl InteractiveMarkerPose {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerPose__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for InteractiveMarkerPose {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__InteractiveMarkerPose__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct InteractiveMarkerPoseSeqRaw {
    data: *mut InteractiveMarkerPose,
    size: usize,
    capacity: usize,
}

/// Sequence of InteractiveMarkerPose.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerPoseSeq<const N: usize> {
    data: *mut InteractiveMarkerPose,
    size: usize,
    capacity: usize,
}

impl<const N: usize> InteractiveMarkerPoseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: InteractiveMarkerPoseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerPose__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: InteractiveMarkerPoseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[InteractiveMarkerPose] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [InteractiveMarkerPose] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, InteractiveMarkerPose> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, InteractiveMarkerPose> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for InteractiveMarkerPoseSeq<N> {
    fn drop(&mut self) {
        let mut msg = InteractiveMarkerPoseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { visualization_msgs__msg__InteractiveMarkerPose__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for InteractiveMarkerPoseSeq<N> {}
unsafe impl<const N: usize> Sync for InteractiveMarkerPoseSeq<N> {}


impl TypeSupport for InteractiveMarkerPose {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerPose()
        }
    }
}

impl PartialEq for InteractiveMarkerPose {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            visualization_msgs__msg__InteractiveMarkerPose__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for InteractiveMarkerPoseSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = InteractiveMarkerPoseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = InteractiveMarkerPoseSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            visualization_msgs__msg__InteractiveMarkerPose__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

