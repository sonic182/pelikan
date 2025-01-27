use rustcommon_metrics::*;

counter!(GET, "total number of get requests");
heatmap!(
    GET_CARDINALITY,
    super::request::MAX_BATCH_SIZE,
    "distribution of key cardinality for get requests"
);
counter!(GET_EX, "number of get requests resulting in an exception");
counter!(GET_KEY, "total number of keys fetched in get requests");
counter!(
    GET_KEY_HIT,
    "number of keys fetched in get requests that resulted in a cache hit"
);
counter!(
    GET_KEY_MISS,
    "number of keys fetched in get requests that resulted in a cache miss"
);

counter!(GETS);
counter!(GETS_EX);
counter!(GETS_KEY);
counter!(GETS_KEY_HIT);
counter!(GETS_KEY_MISS);
counter!(SET);
counter!(SET_EX);
counter!(SET_STORED);
counter!(SET_NOT_STORED);
counter!(ADD);
counter!(ADD_EX);
counter!(ADD_STORED);
counter!(ADD_NOT_STORED);
counter!(REPLACE);
counter!(REPLACE_EX);
counter!(REPLACE_STORED);
counter!(REPLACE_NOT_STORED);
counter!(APPEND);
counter!(APPEND_EX);
counter!(APPEND_STORED);
counter!(APPEND_NOT_STORED);
counter!(PREPEND);
counter!(PREPEND_EX);
counter!(PREPEND_STORED);
counter!(PREPEND_NOT_STORED);
counter!(DELETE);
counter!(DELETE_EX);
counter!(DELETE_DELETED);
counter!(DELETE_NOT_FOUND);
counter!(INCR);
counter!(INCR_EX);
counter!(INCR_NOT_FOUND);
counter!(DECR);
counter!(DECR_EX);
counter!(DECR_NOT_FOUND);
counter!(CAS);
counter!(CAS_EX);
counter!(CAS_EXISTS);
counter!(CAS_NOT_FOUND);
counter!(CAS_STORED);
