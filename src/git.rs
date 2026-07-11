use octocrab::{self, Page, models};
use tokio;
use url::Url;
use http::uri::Uri;

#[derive(Default, Clone)]
pub enum GithubResult {
    Page(Page<models::Repository>),
    Repository(models::Repository),
    #[default]
    None,
}

#[derive(Default)]
pub struct Explorer {
    page_stack: Vec<GithubResult>,
    pub current_page: GithubResult,
}

impl Explorer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_result(&mut self, result: GithubResult) {
        self.page_stack.push(self.current_page.clone());
        self.current_page = result;
    }

    pub fn back(&mut self) -> Option<GithubResult> {
        self.page_stack.pop()
    }

    pub async fn get(&mut self, request: Url) {
        let octocrab = octocrab::instance();
        //let response = octocrab._get(request).await;
        //if let Ok(r) = response {
        //    println!("{:?}", r);
        //}
    }
    pub async fn follow(&mut self, request: Uri) {
        let octocrab = octocrab::instance();
        //let response : octocrab::models::Author= octocrab.get(request, None::<&()>).await.expect("error");
        //    println!("{:?}", response);
        
    }
    pub async fn search_repo(&mut self, text: &str) {
        let page = octocrab::instance().search().repositories(text).sort("stars").order("desc").send().await.unwrap();
        self.add_result(GithubResult::Page(page));

    }
}

// Repository -> Branches
//         \       v (sha)
//          \ -> Trees -> Files
