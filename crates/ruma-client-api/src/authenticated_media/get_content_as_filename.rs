//! `GET /_matrix/client/*/media/download/{serverName}/{mediaId}/{fileName}`
//!
//! Retrieve content from the media store, specifying a filename to return.

pub mod v1 {
    //! `/v1/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixclientv1mediadownloadservernamemediaidfilename

    use std::{borrow::Cow, time::Duration};

    use http::header::{CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_TYPE};
    use ruma_common::{
        api::{request, response, Metadata},
        http_headers::ContentDisposition,
        metadata, IdParseError, Mxc, MxcUri, OwnedServerName,
    };

    use crate::http_headers::CROSS_ORIGIN_RESOURCE_POLICY;

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/org.matrix.msc3916/media/download/:server_name/:media_id/:filename",
            1.11 => "/_matrix/client/v1/media/download/:server_name/:media_id/:filename",
        }
    };

    /// Request type for the `get_media_content_as_filename` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The server name from the mxc:// URI (the authoritory component).
        #[ruma_api(path)]
        pub server_name: OwnedServerName,

        /// The media ID from the mxc:// URI (the path component).
        #[ruma_api(path)]
        pub media_id: String,

        /// The filename to return in the `Content-Disposition` header.
        #[ruma_api(path)]
        pub filename: String,

        /// The maximum duration that the client is willing to wait to start receiving data, in the
        /// case that the content has not yet been uploaded.
        ///
        /// The default value is 20 seconds.
        #[ruma_api(query)]
        #[serde(
            with = "ruma_common::serde::duration::ms",
            default = "ruma_common::media::default_download_timeout",
            skip_serializing_if = "ruma_common::media::is_default_download_timeout"
        )]
        pub timeout_ms: Duration,
    }

    /// Response type for the `get_media_content_as_filename` endpoint.
    #[response(error = crate::Error)]
    pub struct Response {
        /// The content that was previously uploaded.
        #[ruma_api(raw_body)]
        pub file: Vec<u8>,

        /// The content type of the file that was previously uploaded.
        #[ruma_api(header = CONTENT_TYPE)]
        pub content_type: Option<Cow<'static, str>>,

        /// The value of the `Content-Disposition` HTTP header, possibly containing the name of the
        /// file that was previously uploaded.
        #[ruma_api(header = CONTENT_DISPOSITION)]
        pub content_disposition: Option<ContentDisposition>,

        /// The value of the `Cross-Origin-Resource-Policy` HTTP header.
        ///
        /// See [MDN] for the syntax.
        ///
        /// [MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Resource-Policy#syntax
        ///
        /// TODO: make this use Cow static str's
        #[ruma_api(header = CROSS_ORIGIN_RESOURCE_POLICY)]
        pub cross_origin_resource_policy: Option<Cow<'static, str>>,

        /// The value of the `Cache-Control` HTTP header.
        ///
        /// See [MDN] for the syntax.
        ///
        /// [MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control#syntax
        ///
        /// TODO: make this use Cow static str's
        #[ruma_api(header = CACHE_CONTROL)]
        pub cache_control: Option<Cow<'static, str>>,
    }

    impl Request {
        /// Creates a new `Request` with the given media ID, server name and filename.
        pub fn new(media_id: String, server_name: OwnedServerName, filename: String) -> Self {
            Self {
                media_id,
                server_name,
                filename,
                timeout_ms: ruma_common::media::default_download_timeout(),
            }
        }

        /// Creates a new `Request` with the given URI and filename.
        pub fn from_uri(uri: &MxcUri, filename: String) -> Result<Self, IdParseError> {
            let Mxc { server_name, media_id } = uri.parts()?;

            Ok(Self::new(media_id.to_owned(), server_name.to_owned(), filename))
        }
    }

    impl Response {
        /// Creates a new `Response` with the given file.
        pub fn new(file: Vec<u8>) -> Self {
            Self {
                file,
                content_type: None,
                content_disposition: None,
                cross_origin_resource_policy: None,
                cache_control: None,
            }
        }
    }
}
