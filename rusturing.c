#include "liburing.h"

extern inline void rust_io_uring_cq_advance(struct io_uring *ring,
                                            unsigned nr)
{
    if (nr) {
        struct io_uring_cq *cq = &ring->cq;
        io_uring_smp_store_release(cq->khead, *cq->khead + nr);
    }
}
