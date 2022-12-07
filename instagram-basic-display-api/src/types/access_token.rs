//! [Ref](https://developers.facebook.com/docs/instagram-basic-display-api/overview#long-lived-access-tokens)

use core::time::Duration;

//
//
//
pub const LONG_LIVED_USER_ACCESS_TOKEN_LIFETIME: Duration = Duration::from_secs(3600 * 24 * 60);
pub const SHORT_LIVED_USER_ACCESS_TOKEN_LIFETIME: Duration = Duration::from_secs(3600);

wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LongLivedUserAccessToken(String);
}

wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ShortLivedUserAccessToken(String);
}

wrapping_macro::wrapping_string! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UserAccessToken(String);
}

impl From<LongLivedUserAccessToken> for UserAccessToken {
    fn from(t: LongLivedUserAccessToken) -> Self {
        Self(t.into_inner())
    }
}

impl From<&LongLivedUserAccessToken> for UserAccessToken {
    fn from(t: &LongLivedUserAccessToken) -> Self {
        Self(t.inner().into())
    }
}

impl From<ShortLivedUserAccessToken> for UserAccessToken {
    fn from(t: ShortLivedUserAccessToken) -> Self {
        Self(t.into_inner())
    }
}

impl From<&ShortLivedUserAccessToken> for UserAccessToken {
    fn from(t: &ShortLivedUserAccessToken) -> Self {
        Self(t.inner().into())
    }
}
