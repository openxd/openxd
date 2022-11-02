pub struct VersionReq;

pub enum PackageReq {
    LocalPackage(String),
    ExternalPackage(String, VersionReq)
}
