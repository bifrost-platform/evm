use utils::bigint::M256;
use utils::read_hex;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Address([u8; 20]);

impl Default for Address {
    fn default() -> Address {
        Address([0u8; 20])
    }
}

impl Into<M256> for Address {
    fn into(self) -> M256 {
        M256::from(self.0.as_ref())
    }
}

impl From<M256> for Option<Address> {
    fn from(mut val: M256) -> Option<Address> {
        let max: M256 = M256::one() << 161;
        if val >= max {
            None
        } else {
            let mut i = 20;
            let mut a = [0u8; 20];

            while i != 0 {
                let u: u64 = (val & 0xFF.into()).into();
                a[i-1] = u as u8;

                i -= 1;
                val = val >> 8;
            }

            Some(Address(a))
        }
    }
}

impl Address {
    pub fn from_str(s: &str) -> Option<Address> {
        let v = read_hex(s);
        if v.is_none() { return None; }
        let v = v.unwrap();

        if v.len() == 20 {
            let mut a = [0u8; 20];
            for i in 0..20 {
                a[i] = v[i];
            }
            Some(Address(a))
        } else {
            None
        }
    }
}