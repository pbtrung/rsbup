use hc256;
use skein3fish;

pub const CTR_COUNTER_LEN: usize = 32;
pub const HC256_KEY_LEN: usize = 32;
pub const HC256_IV_LEN: usize = 32;

pub struct CTR {
    t3f: skein3fish::Threefish,
    hc256: hc256::HC256,
    is_hc256_used: bool,
}
impl CTR {
    pub fn new(
        t3f_key: &[u8; skein3fish::T3F_KEY_LEN],
        t3f_tweak: &[u8; skein3fish::T3F_TWEAK_LEN],
        hc256_key: &[u8],
        hc256_iv: &[u8],
    ) -> CTR {
        assert_eq!(hc256_key.len(), HC256_KEY_LEN);
        assert_eq!(hc256_iv.len(), HC256_IV_LEN);

        let t3f = skein3fish::Threefish::new(&t3f_key, &t3f_tweak);
        let hc256 = hc256::HC256::new(&hc256_key, &hc256_iv);
        CTR {
            t3f: t3f,
            hc256: hc256,
            is_hc256_used: false,
        }
    }
    pub fn reset_hc256(&mut self, hc256_key: &[u8], hc256_iv: &[u8]) {
        assert_eq!(hc256_key.len(), HC256_KEY_LEN);
        assert_eq!(hc256_iv.len(), HC256_IV_LEN);

        let hc256 = hc256::HC256::new(&hc256_key, &hc256_iv);
        self.hc256 = hc256;
        self.is_hc256_used = false;
    }
    pub fn encrypt(
        &mut self,
        input: &[u8],
        ctr_iv: &[u8; skein3fish::T3F_BLOCK_LEN],
    ) -> Vec<u8> {
        if self.is_hc256_used == true {
            panic!("{}", "Error: ctr::encrypt: Must reset HC256");
        }
        let mut output = Vec::with_capacity(input.len() as usize);
        unsafe {
            output.set_len(input.len() as usize);
        }
        let mut counter = [0u8; CTR_COUNTER_LEN];
        let mut ctr_iv_counter: [u8; skein3fish::T3F_BLOCK_LEN] =
            ctr_iv.clone();
        let mut t3f_buf = [0u8; skein3fish::T3F_BLOCK_LEN];
        let mut i = 0;
        let mut in_len = input.len();

        while in_len >= skein3fish::T3F_BLOCK_LEN {
            self.hc256.process(&[0; CTR_COUNTER_LEN], &mut counter);
            for j in 0..CTR_COUNTER_LEN {
                ctr_iv_counter[j] = ctr_iv[j] ^ counter[j];
            }
            t3f_buf = self.t3f.block_encrypt(&ctr_iv_counter);
            for j in 0..skein3fish::T3F_BLOCK_LEN {
                output[i * skein3fish::T3F_BLOCK_LEN + j] =
                    t3f_buf[j] ^ input[i * skein3fish::T3F_BLOCK_LEN + j];
            }
            i = i + 1;
            in_len -= skein3fish::T3F_BLOCK_LEN;
        }
        if in_len > 0 {
            self.hc256.process(&[0; CTR_COUNTER_LEN], &mut counter);
            for j in 0..CTR_COUNTER_LEN {
                ctr_iv_counter[j] = ctr_iv[j] ^ counter[j];
            }
            t3f_buf = self.t3f.block_encrypt(&ctr_iv_counter);
            for j in 0..in_len {
                output[i * skein3fish::T3F_BLOCK_LEN + j] =
                    t3f_buf[j] ^ input[i * skein3fish::T3F_BLOCK_LEN + j];
            }
        }
        self.is_hc256_used = true;
        output
    }
    pub fn decrypt(
        &mut self,
        input: &[u8],
        ctr_iv: &[u8; skein3fish::T3F_BLOCK_LEN],
    ) -> Vec<u8> {
        self.encrypt(&input, &ctr_iv)
    }
}

#[test]
fn test_ctr() {
    let t3f_key: [u8; skein3fish::T3F_KEY_LEN] = [0; skein3fish::T3F_KEY_LEN];
    let t3f_tweak: [u8; skein3fish::T3F_TWEAK_LEN] =
        [0; skein3fish::T3F_TWEAK_LEN];
    let hc256_key: [u8; HC256_KEY_LEN] = [0; HC256_KEY_LEN];
    let hc256_iv: [u8; HC256_IV_LEN] = [0; HC256_IV_LEN];
    let ctr_iv: [u8; skein3fish::T3F_BLOCK_LEN] =
        [0; skein3fish::T3F_BLOCK_LEN];
    let input: [u8; skein3fish::T3F_BLOCK_LEN * 2 + 56] =
        [0; skein3fish::T3F_BLOCK_LEN * 2 + 56];

    let mut ctr = CTR::new(&t3f_key, &t3f_tweak, &hc256_key, &hc256_iv);
    let output = ctr.encrypt(&input, &ctr_iv);
    ctr.reset_hc256(&hc256_key, &hc256_iv);
    let dec_output = ctr.decrypt(&output, &ctr_iv);

    for i in 0..input.len() {
        assert_eq!(input[i], dec_output[i]);
    }
}
