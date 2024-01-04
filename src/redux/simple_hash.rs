
pub type SimpleHash = u64;

pub trait SimpleHashable {
    fn simple_hash(&self) -> SimpleHash;
}
