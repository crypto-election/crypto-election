#[derive(Debug)]
struct PrincipalIterator<T>
where
    T: IndexAccess,
{
    index: ProofMapIndex<T, PublicKey, Administration>,
    key: Option<PublicKey>,
}

impl<T> Iterator for PrincipalIterator<T>
where
    T: IndexAccess,
{
    type Item = Administration;

    fn next(&mut self) -> Option<Self::Item> {
        self.key.map(|key| {
            let principal = self
                .index
                .get(&key)
                .expect("Unable to find administration by public key.");
            self.key = principal.principal_key.0;
            principal
        })
    }
}
