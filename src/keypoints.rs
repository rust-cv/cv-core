use derive_more::{AsMut, AsRef, Constructor, Deref, DerefMut, From, Into};
use nalgebra::Point2;

/// A point on an image frame. This type should only be used when
/// the point location is on the image frame in pixel coordinates.
/// This means the keypoint is neither undistorted nor normalized.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    AsMut,
    AsRef,
    Constructor,
    Deref,
    DerefMut,
    From,
    Into,
)]
pub struct ImageKeyPoint(pub Point2<f32>);

/// A point in normalized image coordinates. This keypoint has been corrected
/// for distortion and normalized based on the camrea intrinsic matrix.
/// Please note that the intrinsic matrix accounts for the natural focal length
/// and any magnification to the image. Ultimately, the key points must be
/// represented by their position on the camera sensor and normalized to the
/// focal length of the camera.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    AsMut,
    AsRef,
    Constructor,
    Deref,
    DerefMut,
    From,
    Into,
)]
pub struct NormalizedKeyPoint(pub Point2<f32>);