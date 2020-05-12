#include "liburing.h"

extern inline void rust_io_uring_cq_advance(struct io_uring *ring, unsigned nr)
{
    io_uring_cq_advance(ring, nr);
}

extern inline void rust_io_uring_cqe_seen(struct io_uring *ring, struct io_uring_cqe *cqe)
{
    io_uring_cqe_seen(ring, cqe);
}

extern inline void rust_io_uring_sqe_set_data(struct io_uring_sqe *sqe, void *data)
{
    io_uring_sqe_set_data(sqe, data);
}

extern inline void *rust_io_uring_cqe_get_data(struct io_uring_cqe *cqe)
{
    return io_uring_cqe_get_data(cqe);
}

extern inline void rust_io_uring_sqe_set_flags(struct io_uring_sqe *sqe, unsigned flags)
{
    io_uring_sqe_set_flags(sqe, flags);
}

extern inline void rust_io_uring_prep_rw(int op,
                                         struct io_uring_sqe *sqe,
                                         int fd,
                                         const void *addr,
                                         unsigned len,
                                         __u64 offset)
{
    io_uring_prep_rw(op, sqe, fd, addr, len, offset);
}

extern inline void rust_io_uring_prep_readv(struct io_uring_sqe *sqe,
                                            int fd,
                                            const struct iovec *iovecs,
                                            unsigned nr_vecs,
                                            off_t offset)
{
    io_uring_prep_readv(sqe, fd, iovecs, nr_vecs, offset);
}

extern inline void rust_io_uring_prep_read_fixed(struct io_uring_sqe *sqe,
                                                 int fd,
                                                 void *buf,
                                                 unsigned nbytes,
                                                 off_t offset,
                                                 int buf_index)
{
    io_uring_prep_read_fixed(sqe, fd, buf, nbytes, offset, buf_index);
}

extern inline void rust_io_uring_prep_writev(struct io_uring_sqe *sqe,
                                             int fd,
                                             const struct iovec *iovecs,
                                             unsigned nr_vecs,
                                             off_t offset)
{
    io_uring_prep_writev(sqe, fd, iovecs, nr_vecs, offset);
}

extern inline void rust_io_uring_prep_write_fixed(struct io_uring_sqe *sqe,
                                                  int fd,
                                                  const void *buf,
                                                  unsigned nbytes,
                                                  off_t offset,
                                                  int buf_index)
{
    io_uring_prep_write_fixed(sqe, fd, buf, nbytes, offset, buf_index);
}

extern inline void rust_io_uring_prep_recvmsg(struct io_uring_sqe *sqe,
                                              int fd,
                                              struct msghdr *msg,
                                              unsigned flags)
{
    io_uring_prep_recvmsg(sqe, fd, msg, flags);
}

extern inline void rust_io_uring_prep_sendmsg(struct io_uring_sqe *sqe,
                                              int fd,
                                              const struct msghdr *msg,
                                              unsigned flags)
{
    io_uring_prep_sendmsg(sqe, fd, msg, flags);
}

extern inline void rust_io_uring_prep_poll_add(struct io_uring_sqe *sqe, int fd, short poll_mask)
{
    io_uring_prep_poll_add(sqe, fd, poll_mask);
}

extern inline void rust_io_uring_prep_poll_remove(struct io_uring_sqe *sqe, void *user_data)
{
    io_uring_prep_poll_remove(sqe, user_data);
}

extern inline void rust_io_uring_prep_fsync(struct io_uring_sqe *sqe, int fd, unsigned fsync_flags)
{
    io_uring_prep_fsync(sqe, fd, fsync_flags);
}

extern inline void rust_io_uring_prep_nop(struct io_uring_sqe *sqe)
{
    io_uring_prep_nop(sqe);
}

extern inline void rust_io_uring_prep_timeout(struct io_uring_sqe *sqe,
                                              struct __kernel_timespec *ts,
                                              unsigned count,
                                              unsigned flags)
{
    io_uring_prep_timeout(sqe, ts, count, flags);
}

extern inline void rust_io_uring_prep_timeout_remove(struct io_uring_sqe *sqe,
                                                     __u64 user_data,
                                                     unsigned flags)
{
    io_uring_prep_timeout_remove(sqe, user_data, flags);
}

extern inline void rust_io_uring_prep_accept(struct io_uring_sqe *sqe,
                                             int fd,
                                             struct sockaddr *addr,
                                             socklen_t *addrlen,
                                             int flags)
{
    io_uring_prep_accept(sqe, fd, addr, addrlen, flags);
}

extern inline void rust_io_uring_prep_cancel(struct io_uring_sqe *sqe,
                                             void *user_data,
                                             int flags)
{
    io_uring_prep_cancel(sqe, user_data, flags);
}

extern inline void rust_io_uring_prep_link_timeout(struct io_uring_sqe *sqe,
                                                   struct __kernel_timespec *ts,
                                                   unsigned flags)
{
    io_uring_prep_link_timeout(sqe, ts, flags);
}

extern inline void rust_io_uring_prep_connect(struct io_uring_sqe *sqe,
                                              int fd,
                                              struct sockaddr *addr,
                                              socklen_t addrlen)
{
    io_uring_prep_connect(sqe, fd, addr, addrlen);
}

extern inline void rust_io_uring_prep_files_update(struct io_uring_sqe *sqe,
                                                   int *fds,
                                                   unsigned nr_fds,
                                                   int offset)
{
    io_uring_prep_files_update(sqe, fds, nr_fds, offset);
}

extern inline void rust_io_uring_prep_fallocate(struct io_uring_sqe *sqe,
                                                int fd,
                                                int mode,
                                                off_t offset,
                                                off_t len)
{
    io_uring_prep_fallocate(sqe, fd, mode, offset, len);
}

extern inline void rust_io_uring_prep_openat(struct io_uring_sqe *sqe,
                                             int dfd,
                                             const char *path,
                                             int flags,
                                             mode_t mode)
{
    io_uring_prep_openat(sqe, dfd, path, flags, mode);
}

extern inline void rust_io_uring_prep_close(struct io_uring_sqe *sqe, int fd) {
    io_uring_prep_close(sqe, fd);
}

extern inline void rust_io_uring_prep_statx(struct io_uring_sqe *sqe,
                                            int dfd,
                                            const char *path,
                                            int flags,
                                            unsigned mask,
                                            struct statx *statxbuf)
{
    io_uring_prep_statx(sqe, dfd, path, flags, mask, statxbuf);
}

extern inline unsigned rust_io_uring_sq_ready(struct io_uring *ring)
{
    return io_uring_sq_ready(ring);
}

extern inline unsigned rust_io_uring_sq_space_left(struct io_uring *ring)
{
    return io_uring_sq_space_left(ring);
}

extern inline unsigned rust_io_uring_cq_ready(struct io_uring *ring)
{
    return io_uring_cq_ready(ring);
}

extern inline int rust_io_uring_wait_cqe_nr(struct io_uring *ring,
                                            struct io_uring_cqe **cqe_ptr,
                                            unsigned wait_nr)
{
    return io_uring_wait_cqe_nr(ring, cqe_ptr, wait_nr);
}

extern inline int rust_io_uring_peek_cqe(struct io_uring *ring, struct io_uring_cqe **cqe_ptr)
{
    return io_uring_peek_cqe(ring, cqe_ptr);
}

extern inline int rust_io_uring_wait_cqe(struct io_uring *ring, struct io_uring_cqe **cqe_ptr)
{
    return io_uring_wait_cqe(ring, cqe_ptr);
}
