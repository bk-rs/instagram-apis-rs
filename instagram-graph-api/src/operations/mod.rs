//
pub mod ig_comment;
pub mod ig_container;
pub mod ig_media;
pub mod ig_user;

pub use ig_comment::{
    deleting::Deleting as IgCommentDeleting,
    reading::{BulkReading as IgCommentBulkReading, Reading as IgCommentReading},
    replies::creating::Creating as IgCommentRepliesCreating,
    updating::UpdatingWithHideOrUnhide as IgCommentUpdatingWithHideOrUnhide,
};
pub use ig_container::reading::Reading as IgContainerReading;
pub use ig_media::{
    comments::{
        creating::Creating as IgMediaCommentsCreating,
        reading::{
            Reading as IgMediaCommentsReading,
            ReadingResponseBodyOkJson as IgMediaCommentsReadingResponseBodyOkJson,
        },
    },
    updating::UpdatingWithEnableOrDisableComments as IgMediaUpdatingWithEnableOrDisableComments,
};
pub use ig_user::{
    business_discovery::reading::Reading as IgUserBusinessDiscoveryReading,
    media::creating::{
        CreatingWithCarousel as IgUserMediaCreatingWithCarousel,
        CreatingWithCarouselItemImage as IgUserMediaCreatingWithCarouselItemImage,
        CreatingWithCarouselItemVideo as IgUserMediaCreatingWithCarouselItemVideo,
        CreatingWithImage as IgUserMediaCreatingWithImage,
        CreatingWithReels as IgUserMediaCreatingWithReels,
        CreatingWithVideo as IgUserMediaCreatingWithVideo,
        ExtInfoError as IgUserMediaCreatingExtInfoError,
        ValueUserTag as IgUserMediaCreatingValueUserTag,
    },
    media_publish::creating::{
        Creating as IgUserMediaPublishCreating,
        CreatingResponseBodyOkJson as IgUserMediaPublishCreatingResponseBodyOkJson,
    },
};

//
pub mod common;

pub use common::{EndpointError, EndpointRet};

//
pub const URL_BASE: &str = "https://graph.facebook.com";
pub const VERSION: &str = "v15.0";
