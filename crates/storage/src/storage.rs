use crate::resolver::FabricResolver;

pub struct FabricStorage {
    resolver: FabricResolver,
}

impl FabricStorage {
    pub fn new() -> Self {
        let resolver = FabricResolver::new_inmemory();

        Self { resolver }
    }

    pub fn get_resolver(&self) -> &FabricResolver {
        &self.resolver
    }
}
