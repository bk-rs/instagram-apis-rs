//! [Ref](https://developers.facebook.com/docs/instagram-basic-display-api/overview#long-lived-access-tokens)

use core::time::Duration;

//
//
//
pub const LONG_LIVED_USER_ACCESS_TOKEN_LIFETIME: Duration = Duration::from_secs(3600 * 24 * 60);
pub const SHORT_LIVED_USER_ACCESS_TOKEN_LIFETIME: Duration = Duration::from_secs(3600);

wrapping_macro::wrapping! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LongLivedUserAccessToken(String);
}
impl core::fmt::Display for LongLivedUserAccessToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<&str> for LongLivedUserAccessToken {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}
impl From<&String> for LongLivedUserAccessToken {
    fn from(s: &String) -> Self {
        Self(s.into())
    }
}

wrapping_macro::wrapping! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ShortLivedUserAccessToken(String);
}
impl core::fmt::Display for ShortLivedUserAccessToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<&str> for ShortLivedUserAccessToken {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}
impl From<&String> for ShortLivedUserAccessToken {
    fn from(s: &String) -> Self {
        Self(s.into())
    }
}

wrapping_macro::wrapping! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UserAccessToken(String);
}
impl core::fmt::Display for UserAccessToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<&str> for UserAccessToken {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}
impl From<&String> for UserAccessToken {
    fn from(s: &String) -> Self {
        Self(s.into())
    }
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
