use std::error::Error;

use async_trait::async_trait;

use crate::date::Date;
use crate::diary_repository::DiaryRepositoryTrait;

#[async_trait]
pub trait DiaryServiceTrait {
    async fn create_diary_page(&self, id: &str, date: &Date) -> Result<(), Box<dyn Error>>;
}

pub struct DiaryService<R: DiaryRepositoryTrait> {
    diary_repository: R,
}

impl<R: DiaryRepositoryTrait> DiaryService<R> {
    pub fn new(diary_repository: R) -> Self {
        DiaryService { diary_repository }
    }
}

#[async_trait]
impl<R: DiaryRepositoryTrait + Sync + Send> DiaryServiceTrait for DiaryService<R> {
    async fn create_diary_page(&self, id: &str, date: &Date) -> Result<(), Box<dyn Error>> {
        println!("Creating diary page started.");

        let exist = self.diary_repository.exist(id, date).await?;
        if exist {
            println!("Today's diary page was already created.");
            return Ok(());
        }

        self.diary_repository.create_page(id, date).await?;

        println!("Today's diary page was created successfully.");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::*;

    use crate::diary_repository::MockDiaryRepositoryTrait;

    use super::*;

    #[tokio::test]
    async fn test_create_diary_page_when_page_exists() {
        let id = "id";
        let date = Date::new(2023, 3, 29, 0, 0, 0);

        let mut diary_repository = MockDiaryRepositoryTrait::new();
        diary_repository
            .expect_exist()
            .with(predicate::eq(id), predicate::always())
            .times(1)
            .returning(|_, _| Ok(true));
        diary_repository.expect_create_page().times(0);
        let diary_service = DiaryService::new(diary_repository);

        let result = diary_service.create_diary_page(id, &date).await.unwrap();

        assert_eq!(result, ());
    }

    #[tokio::test]
    async fn test_create_diary_page_when_no_page_exists() {
        let id = "id";
        let date = Date::new(2023, 3, 29, 0, 0, 0);

        let mut diary_repository = MockDiaryRepositoryTrait::new();
        diary_repository
            .expect_exist()
            .with(predicate::eq(id), predicate::always())
            .times(1)
            .returning(|_, _| Ok(false));
        diary_repository
            .expect_create_page()
            .with(predicate::eq(id), predicate::always())
            .times(1)
            .returning(|_, _| Ok(()));
        let diary_service = DiaryService::new(diary_repository);

        let result = diary_service.create_diary_page(id, &date).await.unwrap();

        assert_eq!(result, ());
    }
}
