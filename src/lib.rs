pub mod syscalls;

mod types;

pub use types::*;

pub const LIBURING_UDATA_TIMEOUT: libc::__u64 = libc::__u64::max_value();

// sqe opcode constants
#[repr(C)]
#[non_exhaustive]
#[allow(nonstandard_style)]
pub enum IoRingOp {
    IORING_OP_NOP,
    IORING_OP_READV,
    IORING_OP_WRITEV,
    IORING_OP_FSYNC,
    IORING_OP_READ_FIXED,
    IORING_OP_WRITE_FIXED,
    IORING_OP_POLL_ADD,
    IORING_OP_POLL_REMOVE,
    IORING_OP_SYNC_FILE_RANGE,
    IORING_OP_SENDMSG,
    IORING_OP_RECVMSG,
    IORING_OP_TIMEOUT,
    IORING_OP_TIMEOUT_REMOVE,
    IORING_OP_ACCEPT,
    IORING_OP_ASYNC_CANCEL,
    IORING_OP_LINK_TIMEOUT,
    IORING_OP_CONNECT,
    IORING_OP_FALLOCATE,
    IORING_OP_OPENAT,
    IORING_OP_CLOSE,
    IORING_OP_FILES_UPDATE,
    IORING_OP_STATX,
}

// sqe.flags
pub const IOSQE_FIXED_FILE:             libc::__u8 = 1 << 0;	/* use fixed fileset */
pub const IOSQE_IO_DRAIN:               libc::__u8 = 1 << 1;	/* issue after inflight IO */
pub const IOSQE_IO_LINK:                libc::__u8 = 1 << 2;	/* links next sqe */
pub const IOSQE_IO_HARDLINK:            libc::__u8 = 1 << 3;	/* like LINK, but stronger */
pub const IOSQE_ASYNC:                  libc::__u8 = 1 << 4;    /* always go async */

// sqe.cmd_flags.fsync_flags
pub const IORING_FSYNC_DATASYNC:        libc::__u32 = 1 << 0;

// sqe.cmd_flags.timeout_flags
pub const IORING_TIMEOUT_ABS:           libc::__u32 = 1 << 0;

// io_uring_setup flags
pub const IORING_SETUP_IOPOLL:	        libc::c_uint = 1 << 0;	/* io_context is polled */
pub const IORING_SETUP_SQPOLL:	        libc::c_uint = 1 << 1;	/* SQ poll thread */
pub const IORING_SETUP_SQ_AFF:	        libc::c_uint = 1 << 2;	/* sq_thread_cpu is valid */
pub const IORING_SETUP_CQSIZE:	        libc::c_uint = 1 << 3;    /* app defines CQ size */

// Magic offsets for the application to mmap the data it needs
pub const IORING_OFF_SQ_RING:           libc::__u64 = 0;
pub const IORING_OFF_CQ_RING:           libc::__u64 = 0x8000000;
pub const IORING_OFF_SQES:              libc::__u64 = 0x10000000;

// sq_ring.kflags
pub const IORING_SQ_NEED_WAKEUP:        libc::c_uint = 1 << 0;

// io_uring_enter flags
pub const IORING_ENTER_GETEVENTS:       libc::c_uint = 1 << 0;
pub const IORING_ENTER_SQ_WAKEUP:       libc::c_uint = 1 << 1;

// io_uring_params.features flags
pub const IORING_FEAT_SINGLE_MMAP:      libc::__u32 = 1 << 0;
pub const IORING_FEAT_NODROP:           libc::__u32 = 1 << 1;
pub const IORING_FEAT_SUBMIT_STABLE:    libc::__u32 = 1 << 2;

// io_uring_register opcodes and arguments
pub const IORING_REGISTER_BUFFERS:      libc::c_uint = 0;
pub const IORING_UNREGISTER_BUFFERS:    libc::c_uint = 1;
pub const IORING_REGISTER_FILES:        libc::c_uint = 2;
pub const IORING_UNREGISTER_FILES:      libc::c_uint = 3;
pub const IORING_REGISTER_EVENTFD:      libc::c_uint = 4;
pub const IORING_UNREGISTER_EVENTFD:    libc::c_uint = 5;
pub const IORING_REGISTER_FILES_UPDATE: libc::c_uint = 6;

#[link(name = "uring")]
extern {
    pub fn io_uring_queue_init(
        entries: libc::c_uint,
        ring: *mut io_uring,
        flags: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_queue_init_params(
        entries: libc::c_uint,
        ring: *mut io_uring,
        params: *mut io_uring_params,
    ) -> libc::c_int;

    pub fn io_uring_queue_mmap(
        fd: libc::c_int,
        params: *mut io_uring_params,
        ring: *mut io_uring,
    ) -> libc::c_int;

    pub fn io_uring_queue_exit(ring: *mut io_uring);

    pub fn io_uring_peek_batch_cqe(
        ring: *mut io_uring,
        cqes: *mut *mut io_uring_cqe,
        count: libc::c_uint
    ) -> libc::c_uint;

    pub fn io_uring_wait_cqes(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        wait_nr: libc::c_uint,
        ts: *const __kernel_timespec,
        sigmask: *const libc::sigset_t
    ) -> libc::c_int;

    pub fn io_uring_wait_cqe_timeout(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        ts: *mut __kernel_timespec
    ) -> libc::c_int;

    pub fn io_uring_submit(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_submit_and_wait(ring: *mut io_uring, wait_nr: libc::c_uint) -> libc::c_int;

    pub fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;

    pub fn io_uring_register_buffers(
        ring: *mut io_uring,
        iovecs: *const libc::iovec,
        nr_iovecs: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_unregister_buffers(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_files(
        ring: *mut io_uring,
        files: *const libc::c_int,
        nr_files: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_unregister_files(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_files_update(
        ring: *mut io_uring,
        off: libc::c_uint,
        files: *const libc::c_int,
        nr_files: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_register_eventfd(ring: *mut io_uring, fd: libc::c_int) -> libc::c_int;

    pub fn io_uring_unregister_eventfd(ring: *mut io_uring) -> libc::c_int;
}

#[link(name = "rusturing")]
extern {
    #[link_name = "rust_io_uring_cq_advance"]
    pub fn io_uring_cq_advance(ring: *mut io_uring, nr: libc::c_uint);

    #[link_name = "rust_io_uring_cqe_seen"]
    pub fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe);

    #[link_name = "rust_io_uring_sqe_set_data"]
    pub fn io_uring_sqe_set_data(sqe: *mut io_uring_sqe, data: *mut libc::c_void);

    #[link_name = "rust_io_uring_cqe_get_data"]
    pub fn io_uring_cqe_get_data(cqe: *mut io_uring_cqe) -> *mut libc::c_void;

    #[link_name = "rust_io_uring_sqe_set_flags"]
    pub fn io_uring_sqe_set_flags(sqe: *mut io_uring_sqe, flags: libc::c_uint);

    #[link_name = "rust_io_uring_prep_rw"]
    pub fn io_uring_prep_rw(
        op: libc::c_int,
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *const libc::c_void,
        len: libc::c_uint,
        offset: libc::__u64,
    );

    #[link_name = "rust_io_uring_prep_readv"]
    pub fn io_uring_prep_readv(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::off_t,
    );

    #[link_name = "rust_io_uring_prep_read_fixed"]
    pub fn io_uring_prep_read_fixed(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::off_t,
        buf_index: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_writev"]
    pub fn io_uring_prep_writev(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::off_t,
    );

    #[link_name = "rust_io_uring_prep_write_fixed"]
    pub fn io_uring_prep_write_fixed(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *const libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::off_t,
        buf_index: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_recvmsg"]
    pub fn io_uring_prep_recvmsg(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        msg: *mut libc::msghdr,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_sendmsg"]
    pub fn io_uring_prep_sendmsg(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        msg: *const libc::msghdr,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_poll_add"]
    pub fn io_uring_prep_poll_add(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        poll_mask: libc::c_short,
    );

    #[link_name = "rust_io_uring_prep_poll_remove"]
    pub fn io_uring_prep_poll_remove(sqe: *mut io_uring_sqe, user_data: *mut libc::c_void);

    #[link_name = "rust_io_uring_prep_fsync"]
    pub fn io_uring_prep_fsync(sqe: *mut io_uring_sqe, fd: libc::c_int, fsync_flags: libc::c_uint);

    #[link_name = "rust_io_uring_prep_nop"]
    pub fn io_uring_prep_nop(sqe: *mut io_uring_sqe);

    #[link_name = "rust_io_uring_prep_timeout"]
    pub fn io_uring_prep_timeout(
        sqe: *mut io_uring_sqe,
        ts: *mut __kernel_timespec,
        count: libc::c_uint,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_timeout_remove"]
    pub fn io_uring_prep_timeout_remove(
        sqe: *mut io_uring_sqe,
        user_data: libc::__u64,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_accept"]
    pub fn io_uring_prep_accept(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_cancel"]
    pub fn io_uring_prep_cancel(
        sqe: *mut io_uring_sqe,
        user_data: *mut libc::c_void,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_link_timeout"]
    pub fn io_uring_prep_link_timeout(
        sqe: *mut io_uring_sqe,
        ts: *mut __kernel_timespec,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_connect"]
    pub fn io_uring_prep_connect(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: libc::socklen_t,
    );

    #[link_name = "rust_io_uring_prep_files_update"]
    pub fn io_uring_prep_files_update(
        sqe: *mut io_uring_sqe,
        fds: *mut libc::c_int,
        nr_fds: libc::c_uint
    );

    #[link_name = "rust_io_uring_prep_fallocate"]
    pub fn io_uring_prep_fallocate(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        mode: libc::c_int,
        offset: libc::off_t,
        len: libc::off_t,
    );

    #[link_name = "rust_io_uring_prep_openat"]
    pub fn io_uring_prep_openat(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        flags: libc::c_int,
        mode: libc::mode_t,
    );

    #[link_name = "rust_io_uring_prep_close"]
    pub fn io_uring_prep_close(sqe: *mut io_uring_sqe, fd: libc::c_int);

    #[link_name = "rust_io_uring_prep_statx"]
    pub fn io_uring_prep_statx(
        sqe: *mut io_uring_sqe,
        dfd: libc::c_int,
        path: *const libc::c_char,
        flags: libc::c_int,
        mask: libc::c_uint,
        statx: *mut libc::statx,
    );

    #[link_name = "rust_io_uring_sq_ready"]
    pub fn io_uring_sq_ready(ring: *mut io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_sq_space_left"]
    pub fn io_uring_sq_space_left(ring: *mut io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_cq_ready"]
    pub fn io_uring_cq_ready(ring: *mut io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_wait_cqe_nr"]
    pub fn io_uring_wait_cqe_nr(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        wait_nr: libc::c_uint,
    ) -> libc::c_int;

    #[link_name = "rust_io_uring_peek_cqe"]
    pub fn io_uring_peek_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> libc::c_int;

    #[link_name = "rust_io_uring_wait_cqe"]
    pub fn io_uring_wait_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> libc::c_int;
}
