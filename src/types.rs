#![allow(unused_unsafe)]

macro_rules! access {
    ($this:ident.$method:ident -> $t:ty = $field:expr) => {
        #[inline(always)] pub fn $method(&$this) -> &$t { unsafe { &$field } }
    };
    ($this:ident.$method:ident/$mut:ident -> $t:ty = $field:expr) => {
        access!($this.$method -> $t = $field);
        #[inline(always)] pub fn $mut(&mut $this) -> &mut $t { unsafe { &mut $field } }
    };
    (move $this:ident.$method:ident -> $t:ty = $field:expr) => {
        #[inline(always)] pub fn $method(&$this) -> $t { unsafe { $field } }
    };
}

#[repr(C)]
pub struct io_uring {
    sq: io_uring_sq,
    cq: io_uring_cq,
    flags: libc::c_uint,
    ring_fd: libc::c_int,
}

impl io_uring {
    access!(self.sq/sq_mut -> io_uring_sq = self.sq);
    access!(self.cq/cq_mut -> io_uring_cq = self.cq);
    access!(self.flags/flags_mut -> libc::c_uint = self.flags);
    access!(move self.ring_fd -> libc::c_int = self.ring_fd);
}

#[repr(C)]
pub struct io_uring_sq {
    khead: *mut libc::c_uint,
    ktail: *mut libc::c_uint,
    kring_mask: *mut libc::c_uint,
    kring_entries: *mut libc::c_uint,
    kflags: *mut libc::c_uint,
    kdropped: *mut libc::c_uint,
    array: *mut libc::c_uint,
    sqes: *mut io_uring_sqe,

    sqe_head: libc::c_uint,
    sqe_tail: libc::c_uint,

    ring_sz: libc::size_t,
    ring_ptr: *mut libc::c_void,
}

impl io_uring_sq {
    access!(move self.khead -> *mut libc::c_uint = self.khead);
    access!(move self.ktail -> *mut libc::c_uint = self.ktail);
    access!(move self.kring_mask -> *mut libc::c_uint = self.kring_mask);
    access!(move self.kring_entries -> *mut libc::c_uint = self.kring_entries);
    access!(move self.kflags -> *mut libc::c_uint = self.kflags);
    access!(move self.kdropped -> *mut libc::c_uint = self.kdropped);
    access!(move self.array -> *mut libc::c_uint = self.array);
    access!(move self.sqes -> *mut io_uring_sqe = self.sqes);
    access!(self.sqe_head/sqe_head_mut -> libc::c_uint = self.sqe_head);
    access!(self.sqe_tail/sqe_tail_mut -> libc::c_uint = self.sqe_tail);
    access!(move self.ring_sz -> libc::size_t = self.ring_sz);
    access!(move self.ring_ptr -> *mut libc::c_void = self.ring_ptr);
}

#[repr(C)]
pub struct io_uring_sqe {
    opcode: libc::__u8,     /* type of operation for this sqe */
    flags: libc::__u8,      /* IOSQE_ flags */
    ioprio: libc::__u16,    /* ioprio for the request */
    fd: libc::__s32,        /* file descriptor to do IO on */
    off_addr2: off_addr2,
    addr: libc::__u64,      /* pointer to buffer or iovecs */
    len: libc::__u32,       /* buffer size or number of iovecs */
    cmd_flags: cmd_flags,
    user_data: libc::__u64, /* data to be passed back at completion time */
    buf_index: buf_index,   /* index into fixed buffers, if used */
}

impl io_uring_sqe {
    access!(self.opcode/opcode_mut -> libc::__u8 = self.opcode);
    access!(self.flags/flags_mut -> libc::__u8 = self.flags);
    access!(self.ioprio/ioprio_mut -> libc::__u16 = self.ioprio);
    access!(self.fd/fd_mut -> libc::__s32 = self.fd);
    access!(self.off/off_mut -> libc::__u64 = self.off_addr2.off);
    access!(self.addr/addr_mut -> libc::__u64 = self.addr);
    access!(self.addr2/addr2_mut -> libc::__u64 = self.off_addr2.addr2);
    access!(self.len/len_mut -> libc::__u32 = self.len);
    access!(self.user_data/user_data_mut -> libc::__u64 = self.user_data);
    access!(self.buf_index/buf_index_mut -> libc::__u16 = self.buf_index.buf_index);
    access!(self.cmd_flags/cmd_flags_mut -> cmd_flags = self.cmd_flags);
}

#[repr(C)]
union off_addr2 {
    off: libc::__u64,
    addr2: libc::__u64,
}

#[repr(C)]
pub union cmd_flags {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: libc::__u32,
    pub poll_events: libc::__u16,
    pub sync_range_flags: libc::__u32,
    pub msg_flags: libc::__u32,
    pub timeout_flags: libc::__u32,
    pub accept_flags: libc::__u32,
    pub cancel_flags: libc::__u32,
    pub open_flags: libc::__u32,
    pub statx_flags: libc::__u32,
}

#[allow(non_camel_case_types)]
pub type __kernel_rwf_t = libc::c_int;

#[repr(C)]
union buf_index {
    buf_index: libc::__u16,
    __pad2: [libc::__u64; 3],
}

#[repr(C)]
pub struct io_uring_cq {
    khead: *mut libc::c_uint,
    ktail: *mut libc::c_uint,
    kring_mask: *mut libc::c_uint,
    kring_entries: *mut libc::c_uint,
    koverflow: *mut libc::c_uint,
    cqes: *mut io_uring_cqe,

    ring_sz: libc::size_t,
    ring_ptr: *mut libc::c_void,
}

impl io_uring_cq {
    access!(move self.khead -> *mut libc::c_uint = self.khead);
    access!(move self.ktail -> *mut libc::c_uint = self.ktail);
    access!(move self.kring_mask -> *mut libc::c_uint = self.kring_mask);
    access!(move self.kring_entries -> *mut libc::c_uint = self.kring_entries);
    access!(move self.koverflow -> *mut libc::c_uint = self.koverflow);
    access!(move self.cqes -> *mut io_uring_cqe = self.cqes);

    access!(move self.ring_sz -> libc::size_t = self.ring_sz);
    access!(move self.ring_ptr -> *mut libc::c_void = self.ring_ptr);
}

#[repr(C)]
pub struct io_uring_cqe {
    user_data: libc::__u64, /* sqe->data submission passed back */
    res: libc::__s32,       /* result code for this event */
    flags: libc::__u32,
}

impl io_uring_cqe {
    access!(move self.user_data -> libc::__u64 = self.user_data);
    access!(move self.res -> libc::__s32 = self.res);
    access!(move self.flags -> libc::__u32 = self.flags);
}

#[repr(C)]
pub struct io_uring_params {
    sq_entries: libc::__u32,
    cq_entries: libc::__u32,
    flags: libc::__u32,
    sq_thread_cpu: libc::__u32,
    sq_thread_idle: libc::__u32,
    features: libc::__u32,
    resv: [libc::__u32; 4],
    sq_off: io_sqring_offsets,
    cq_off: io_cqring_offsets,
}

impl io_uring_params {
    #[inline(always)]
    pub fn zeroed() -> io_uring_params {
        unsafe { core::mem::zeroed() }
    }

    access!(self.sq_entries/sq_entries_mut -> libc::__u32 = self.sq_entries);
    access!(self.cq_entries/cq_entries_mut -> libc::__u32 = self.cq_entries);
    access!(self.flags/flags_mut -> libc::__u32 = self.flags);
    access!(self.sq_thread_cpu/sq_thread_cpu_mut -> libc::__u32 = self.sq_thread_cpu);
    access!(self.sq_thread_idle/sq_thread_idle_mut -> libc::__u32 = self.sq_thread_idle);
    access!(self.features/features_mut -> libc::__u32 = self.features);
    access!(self.sq_off -> io_sqring_offsets = self.sq_off);
    access!(self.cq_off -> io_cqring_offsets = self.cq_off);
}

#[repr(C)]
pub struct io_sqring_offsets {
    head: libc::__u32,
    tail: libc::__u32,
    ring_mask: libc::__u32,
    ring_entries: libc::__u32,
    flags: libc::__u32,
    dropped: libc::__u32,
    array: libc::__u32,
    resv1: libc::__u32,
    resv2: libc::__u64,
}

impl io_sqring_offsets {
    access!(move self.head -> libc::__u32 = self.head);
    access!(move self.tail -> libc::__u32 = self.tail);
    access!(move self.ring_mask -> libc::__u32 = self.ring_mask);
    access!(move self.ring_entries -> libc::__u32 = self.ring_entries);
    access!(move self.flags -> libc::__u32 = self.flags);
    access!(move self.dropped -> libc::__u32 = self.dropped);
    access!(move self.array -> libc::__u32 = self.array);
}

#[repr(C)]
pub struct io_cqring_offsets {
    head: libc::__u32,
    tail: libc::__u32,
    ring_mask: libc::__u32,
    ring_entries: libc::__u32,
    overflow: libc::__u32,
    cqes: libc::__u32,
    resv: [libc::__u64; 2],
}

impl io_cqring_offsets {
    access!(move self.head -> libc::__u32 = self.head);
    access!(move self.tail -> libc::__u32 = self.tail);
    access!(move self.ring_mask -> libc::__u32 = self.ring_mask);
    access!(move self.ring_entries -> libc::__u32 = self.ring_entries);
    access!(move self.overflow -> libc::__u32 = self.overflow);
    access!(move self.cqes -> libc::__u32 = self.cqes);
}

use core::time::Duration;

#[repr(C)]
pub struct __kernel_timespec {
    tv_sec: i64,
    tv_nsec: libc::c_longlong,
}

impl From<Duration> for __kernel_timespec {
    #[inline(always)]
    fn from(duration: Duration) -> __kernel_timespec {
        assert!(duration.as_secs() <= core::i64::MAX as u64);
        __kernel_timespec {
            tv_sec: duration.as_secs() as i64,
            tv_nsec: duration.subsec_nanos() as libc::c_longlong,
        }
    }
}
