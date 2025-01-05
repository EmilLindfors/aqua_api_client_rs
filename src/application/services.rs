use crate::adapters::repository::AkvaregRepository;
use crate::error::Error;



pub struct AquacultureService<R, S>
where
    R: BarentswatchApiPort,
    S: StoragePort,
{
    repository: R,
    arreg: AkvaregRepository,
    storage: S,
}

impl<R, S> AquacultureService<R, S>
where
    R: BarentswatchApiPort,
    S: StoragePort,
{
    pub fn new(repository: R, storage: S) -> Self {
        Self {
            repository,
            arreg: AkvaregRepository {},
            storage,
        }
    }

    pub async fn get_and_store_vessels(
        &mut self,
    ) -> Result<(), Error> {
        let vessels = self.repository.fetch_vessels()?;
        self.storage.store_data(&vessels.0, "vessels")?;
        Ok(())
    }
}