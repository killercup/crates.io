use std::collections::HashMap;

use serialize::{Decodable, Decoder, Encoder, Encodable};
use semver;
use dependency::Kind as DependencyKind;

use keyword::Keyword as CrateKeyword;
use krate::Crate;

#[deriving(Decodable, Encodable)]
pub struct NewCrate {
    pub name: CrateName,
    pub vers: CrateVersion,
    pub deps: Vec<CrateDependency>,
    pub features: HashMap<CrateName, Vec<Feature>>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub keywords: Option<KeywordList>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
}

#[deriving(PartialEq, Eq, Hash)]
pub struct CrateName(pub String);
pub struct CrateVersion(pub semver::Version);
pub struct CrateVersionReq(pub semver::VersionReq);
pub struct KeywordList(pub Vec<Keyword>);
pub struct Keyword(pub String);
pub struct Feature(pub String);

#[deriving(Decodable, Encodable)]
pub struct CrateDependency {
    pub optional: bool,
    pub default_features: bool,
    pub name: CrateName,
    pub features: Vec<Feature>,
    pub version_req: CrateVersionReq,
    pub target: Option<String>,
    pub kind: Option<DependencyKind>,
}

impl<E, D: Decoder<E>> Decodable<D, E> for CrateName {
    fn decode(d: &mut D) -> Result<CrateName, E> {
        let s = raw_try!(d.read_str());
        if !Crate::valid_name(s.as_slice()) {
            return Err(d.error(format!("invalid crate name specified: {}",
                                       s).as_slice()))
        }
        Ok(CrateName(s))
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for Keyword {
    fn decode(d: &mut D) -> Result<Keyword, E> {
        let s = raw_try!(d.read_str());
        if !CrateKeyword::valid_name(s.as_slice()) {
            return Err(d.error(format!("invalid keyword specified: {}",
                                       s).as_slice()))
        }
        Ok(Keyword(s))
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for Feature {
    fn decode(d: &mut D) -> Result<Feature, E> {
        let s = raw_try!(d.read_str());
        if !Crate::valid_feature_name(s.as_slice()) {
            return Err(d.error(format!("invalid feature name specified: {}",
                                       s).as_slice()))
        }
        Ok(Feature(s))
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for CrateVersion {
    fn decode(d: &mut D) -> Result<CrateVersion, E> {
        let s = raw_try!(d.read_str());
        match semver::Version::parse(s.as_slice()) {
            Ok(v) => Ok(CrateVersion(v)),
            Err(..) => Err(d.error(format!("invalid semver: {}", s).as_slice())),
        }
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for CrateVersionReq {
    fn decode(d: &mut D) -> Result<CrateVersionReq, E> {
        let s = raw_try!(d.read_str());
        match semver::VersionReq::parse(s.as_slice()) {
            Ok(v) => Ok(CrateVersionReq(v)),
            Err(..) => Err(d.error(format!("invalid version req: {}",
                                           s).as_slice())),
        }
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for KeywordList {
    fn decode(d: &mut D) -> Result<KeywordList, E> {
        let inner: Vec<Keyword> = raw_try!(Decodable::decode(d));
        if inner.len() > 5 {
            return Err(d.error("a maximum of 5 keywords per crate are allowed"))
        }
        for val in inner.iter() {
            if val.len() > 20 {
                return Err(d.error("keywords must contain less than 20 \
                                    characters"))
            }
        }
        Ok(KeywordList(inner))
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for DependencyKind {
    fn decode(d: &mut D) -> Result<DependencyKind, E> {
        let s: String = raw_try!(Decodable::decode(d));
        match s.as_slice() {
            "dev" => Ok(DependencyKind::Dev),
            "build" => Ok(DependencyKind::Build),
            "normal" => Ok(DependencyKind::Normal),
            s => Err(d.error(format!("invalid dependency kind `{}`, must be \
                                      one of dev, build, or normal",
                                     s).as_slice())),
        }
    }
}

impl<E, D: Encoder<E>> Encodable<D, E> for CrateName {
    fn encode(&self, d: &mut D) -> Result<(), E> {
        d.emit_str(self.as_slice())
    }
}

impl<E, D: Encoder<E>> Encodable<D, E> for Keyword {
    fn encode(&self, d: &mut D) -> Result<(), E> {
        d.emit_str(self.as_slice())
    }
}

impl<E, D: Encoder<E>> Encodable<D, E> for Feature {
    fn encode(&self, d: &mut D) -> Result<(), E> {
        d.emit_str(self.as_slice())
    }
}

impl<E, D: Encoder<E>> Encodable<D, E> for CrateVersion {
    fn encode(&self, d: &mut D) -> Result<(), E> {
        d.emit_str((**self).to_string().as_slice())
    }
}

impl<E, D: Encoder<E>> Encodable<D, E> for CrateVersionReq {
    fn encode(&self, d: &mut D) -> Result<(), E> {
        d.emit_str((**self).to_string().as_slice())
    }
}

impl<E, D: Encoder<E>> Encodable<D, E> for KeywordList {
    fn encode(&self, d: &mut D) -> Result<(), E> {
        let KeywordList(ref inner) = *self;
        inner.encode(d)
    }
}

impl<E, D: Encoder<E>> Encodable<D, E> for DependencyKind {
    fn encode(&self, d: &mut D) -> Result<(), E> {
        match *self {
            DependencyKind::Normal => "normal".encode(d),
            DependencyKind::Build => "build".encode(d),
            DependencyKind::Dev => "dev".encode(d),
        }
    }
}

impl Deref<str> for CrateName {
    fn deref<'a>(&'a self) -> &'a str {
        let CrateName(ref s) = *self;
        s.as_slice()
    }
}

impl Deref<str> for Keyword {
    fn deref<'a>(&'a self) -> &'a str {
        let Keyword(ref s) = *self;
        s.as_slice()
    }
}

impl Deref<str> for Feature {
    fn deref<'a>(&'a self) -> &'a str {
        let Feature(ref s) = *self;
        s.as_slice()
    }
}

impl Deref<semver::Version> for CrateVersion {
    fn deref<'a>(&'a self) -> &'a semver::Version {
        let CrateVersion(ref s) = *self; s
    }
}

impl Deref<semver::VersionReq> for CrateVersionReq {
    fn deref<'a>(&'a self) -> &'a semver::VersionReq {
        let CrateVersionReq(ref s) = *self; s
    }
}

impl Deref<[Keyword]> for KeywordList {
    fn deref<'a>(&'a self) -> &'a [Keyword] {
        let KeywordList(ref s) = *self;
        s.as_slice()
    }
}
