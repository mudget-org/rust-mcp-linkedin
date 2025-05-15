use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ShareContent {
    pub shareCommentary: ShareCommentary,
    pub shareMediaCategory: String,
}

#[derive(Debug, Serialize)]
pub struct ShareCommentary {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct PostVisibility {
    #[serde(rename = "com.linkedin.ugc.MemberNetworkVisibility")]
    pub visibility: String,
}

#[derive(Debug, Serialize)]
pub struct PostPayload {
    pub author: String,
    pub lifecycleState: String,
    pub specificContent: SpecificContent,
    pub visibility: PostVisibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduledAt: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SpecificContent {
    #[serde(rename = "com.linkedin.ugc.ShareContent")]
    pub share_content: ShareContent,
}

#[derive(Debug, Deserialize)]
pub struct PostResponse {
    pub id: String,
}
