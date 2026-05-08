/// Events for mpsc
pub enum Event {
    GetChannelStarted(String), // channel url
    GetChannelFailed(String, String), // channel url, error message
    GetChannelFinished(String), // channel url

    GetPostsStarted(String), // channel url
    GetPostsFailed(String, String), // channel url, error message
    GetPostsFinished(String), // channel url

    DownloadPostStarted(String), // post name
    DownloadPostFailed(String, String), // post name, error message
    DownloadPostSkipped(String), // post name (already exists, skipping)
    DownloadPostFinished(String), // post name

    Done,
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::GetChannelStarted(url) => write!(f, "GetChannelStarted({})", url),
            Event::GetChannelFailed(url, err) => write!(f, "GetChannelFailed({}, {})", url, err),
            Event::GetChannelFinished(url) => write!(f, "GetChannelFinished({})", url),
            Event::GetPostsStarted(url) => write!(f, "GetPostsStarted({})", url),
            Event::GetPostsFailed(url, err) => write!(f, "GetPostsFailed({}, {})", url, err),
            Event::GetPostsFinished(url) => write!(f, "GetPostsFinished({})", url),
            Event::DownloadPostStarted(name) => write!(f, "DownloadPostStarted({})", name),
            Event::DownloadPostFailed(name, err) => write!(f, "DownloadPostFailed({}, {})", name, err),
            Event::DownloadPostSkipped(name) => write!(f, "DownloadPostSkipped({})", name),
            Event::DownloadPostFinished(name) => write!(f, "DownloadPostFinished({})", name),
            Event::Done => write!(f, "Done"),
        }
    }
}
