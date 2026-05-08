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
