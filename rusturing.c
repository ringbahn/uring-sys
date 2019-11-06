#include "liburing.h"

extern inline void rust_io_uring_cq_advance(struct io_uring *ring, unsigned nr)
{
    io_uring_cq_advance(ring, nr);
}
