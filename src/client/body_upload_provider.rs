use crate::{Buffer, Destroy, UploadDataProvider, UploadDataProviderHandler, UploadDataSink};

use super::Body;

type RewindFn = dyn Fn() -> Body;

pub struct BodyUploadDataProvider<'a> {
    /// The HTTP request body to be uploaded.
    body: Body,

    /// A function that returns a new body to be used for rewinding.
    /// This is used for retrying requests when the server times out stale sockets.
    /// Note that rewinding is also required to follow redirects that preserve the upload body.
    /// If this is `None`, rewinding is not supported.
    rewind: Option<&'a RewindFn>,
}

impl<'a> UploadDataProviderHandler for BodyUploadDataProvider<'a> {
    fn length(&self, _: UploadDataProvider) -> i64 {
        self.body.len().unwrap_or(0) as i64
    }

    fn read(&self, _: UploadDataProvider, sink: UploadDataSink, buffer: Buffer) {
        if let Some(bytes) = self.body.as_bytes() {
            let len = bytes.len() as u64;

            if len == 0 {
                sink.on_read_error("Empty body");
                return;
            }

            match buffer.write_slice(bytes, len) {
                Ok(_) => {
                    sink.on_read_succeeded(len, false); // TODO: implement chunked reads
                }
                Err(err) => {
                    sink.on_read_error(err);
                }
            }
        } else {
            sink.on_read_error("Invalid body");
        }
    }

    fn rewind(&mut self, _: UploadDataProvider, sink: UploadDataSink) {
        if let Some(rewind) = &self.rewind {
            self.body = rewind();
            sink.on_rewind_succeeded();
        } else {
            sink.on_rewind_error("Rewinding is not supported");
        }
    }

    fn close(&self, upload_data_provider: UploadDataProvider) {
        upload_data_provider.destroy();
    }
}

impl<'a> BodyUploadDataProvider<'a> {
    pub fn new(body: Body, rewind: Option<&'a RewindFn>) -> Self {
        Self { body, rewind }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::{
        client::Body, Buffer, Destroy, UploadDataProvider, UploadDataProviderHandler,
        UploadDataSink, UploadDataSinkCallbacks,
    };

    use super::BodyUploadDataProvider;

    #[test]
    fn test_body_upload_data_provider() {
        let expected = "test";

        let handler = BodyUploadDataProvider::new(Body::from(expected), None);
        let provider_dummy =
            UploadDataProvider::new(BodyUploadDataProvider::new(Body::from(""), None));
        let sink = UploadDataSink::new(UploadDataSinkCallbacks {
            on_read_succeeded: |_, _, _| {},
            on_read_error: |_, _| {},
            on_rewind_succeeded: |_| {},
            on_rewind_error: |_, _| {},
        });

        let buffer = Buffer::new();
        buffer.init_data_and_callback(
            Box::new(Bytes::new()),
            expected.len() as u64,
            crate::BufferCallback::new(|_, _| {}),
        );

        let ptr = buffer.ptr;

        handler.read(provider_dummy, sink, buffer);

        // Read the modified buffer again by its pointer.
        let buffer = Buffer { ptr };
        let actual = buffer.data::<&[u8]>();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(*actual, expected.as_bytes());

        buffer.destroy();
    }
}
