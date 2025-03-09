pub trait Hashable {
    fn hash(&self, domain_separator: String) -> [u8;  32];
}