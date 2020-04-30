/// A builder pattern struct for constructing an Octocrab request to create a
/// pull request.
/// ```no_run
/// # async fn run() -> octocrab::Result<()> {
/// # let octocrab = octocrab::Octocrab::default();
/// let pr = octocrab
///     .pulls("owner", "repo")
///     .create("title", "head", "base")
///     .body("hello world!")
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(serde::Serialize)]
pub struct CreatePullRequestBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b super::PullRequestHandler<'octo>,
    title: String,
    head: String,
    base: String,
    body: Option<String>,
    draft: Option<bool>,
    maintainer_can_modify: Option<bool>,
}

impl<'octo, 'b> CreatePullRequestBuilder<'octo, 'b> {
    /// Creates a new `CreatePullRequestBuilder` with the required arguments.
    /// - `title` — The title of the new pull request.
    /// - `head` — The name of the branch where your changes are implemented.
    ///   For cross-repository pull requests in the same network, namespace head
    ///   with a user like this: `username:branch`.
    /// - `base` — The name of the branch you want the changes pulled into. This
    ///   should be an existing branch on the current repository. You cannot
    ///   submit a pull request to one repository that requests a merge to a
    ///   base of another repository.
    pub fn new(
        handler: &'b super::PullRequestHandler<'octo>,
        title: impl Into<String>,
        head: impl Into<String>,
        base: impl Into<String>,
    ) -> Self {
        Self {
            handler,
            title: title.into(),
            head: head.into(),
            base: base.into(),
            body: None,
            draft: None,
            maintainer_can_modify: None,
        }
    }

    /// The contents of the pull request.
    pub fn body<A: Into<String>>(mut self, body: impl Into<Option<A>>) -> Self {
        self.body = body.into().map(A::into);
        self
    }

    /// Indicates whether the pull request is a draft.
    pub fn draft(mut self, draft: impl Into<Option<bool>>) -> Self {
        self.draft = draft.into();
        self
    }

    /// Indicates whether `maintainers` can modify the pull request.
    pub fn maintainer_can_modify(mut self, maintainer_can_modify: impl Into<Option<bool>>) -> Self {
        self.maintainer_can_modify = maintainer_can_modify.into();
        self
    }

    /// Sends the request to create the pull request.
    pub async fn send(self) -> crate::Result<crate::models::PullRequest> {
        let url = format!(
            "/repos/{owner}/{repo}/pulls",
            owner = self.handler.owner,
            repo = self.handler.repo
        );

        self.handler.crab.post(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .create("test-pr", "master", "branch")
            .body(String::from("testing..."))
            .draft(true)
            .maintainer_can_modify(true);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "title": "test-pr",
                "head": "master",
                "base": "branch",
                "body": "testing...",
                "draft": true,
                "maintainer_can_modify": true,
            })
        )
    }
}
