use libc::{size_t, uint64_t};
// use time::PreciseTime;

const MIN_PWD_LEN: usize = 64;
const MIN_SALT_LEN: usize = 32;

extern "C" {
    fn pbkdf2(
        password: *const u8,
        password_len: size_t,
        salt: *const u8,
        salt_len: size_t,
        rounds: uint64_t,
        out: *const u8,
        out_len: size_t,
    );
}

pub fn derive_key(
    password: &[u8],
    salt: &[u8],
    rounds: usize,
    out_len: usize,
) -> Vec<u8> {
    assert!(rounds > 0);
    assert!(out_len > 0);
    assert!(password.len() >= MIN_PWD_LEN);
    assert!(salt.len() >= MIN_SALT_LEN);

    let mut hash = Vec::with_capacity(out_len);
    unsafe {
        pbkdf2(
            password.as_ptr(),
            password.len() as size_t,
            salt.as_ptr(),
            salt.len() as size_t,
            rounds as uint64_t,
            hash.as_mut_ptr(),
            out_len as size_t,
        );
        hash.set_len(out_len);
    }
    hash
}

#[test]
fn test_derive_key() {
    let pwd: [u8; MIN_PWD_LEN] = [0; MIN_PWD_LEN];
    let salt: [u8; MIN_SALT_LEN] = [0; MIN_SALT_LEN];
    let rounds = 13;
    let out_len = 64;
    // let start = PreciseTime::now();
    let key = derive_key(&pwd, &salt, rounds, out_len);
    // let end = PreciseTime::now();
    // println!("pbkdf2: {} seconds", start.to(end));
}
