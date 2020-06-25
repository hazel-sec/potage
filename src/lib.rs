use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;
use std::fs::File;
use std::io::{BufReader, Error, Write};

use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::prelude::*;

pub struct HashDigest {
    sha256: String,
    sha1: String,
    md5: String,
}

impl HashDigest {
    pub fn from_file(path: impl AsRef<str>) -> Result<Self, Error> {
        let f = File::open(path.as_ref())?;
        let mut f = BufReader::new(f);
        let mut w = Writer::new();
        std::io::copy(&mut f, &mut w)?;
        Ok(w.digest())
    }

    pub async fn from_file_async(path: impl AsRef<str>) -> Result<Self, Error> {
        let f = tokio::fs::File::open(path.as_ref()).await?;
        let mut f = tokio::io::BufReader::new(f);
        let mut w = Writer::new();
        tokio::io::copy(&mut f, &mut w).await?;
        Ok(w.digest())
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    pub fn sha1(&self) -> &str {
        &self.sha1
    }

    pub fn md5(&self) -> &str {
        &self.md5
    }
}

pub struct Writer {
    sha256: Sha256,
    sha1: Sha1,
    md5: Md5,
}

impl Default for Writer {
    fn default() -> Self {
        Self {
            sha256: Sha256::new(),
            sha1: Sha1::new(),
            md5: Md5::new(),
        }
    }
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn digest(&mut self) -> HashDigest {
        HashDigest {
            sha256: self.sha256.result_str(),
            sha1: self.sha1.result_str(),
            md5: self.md5.result_str(),
        }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.sha256.input(buf);
        self.sha1.input(buf);
        self.md5.input(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl AsyncWrite for Writer {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        let s = self.get_mut();
        s.sha256.input(buf);
        s.sha1.input(buf);
        s.md5.input(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn works() {
        {
            let mut w = Writer::new();
            let d = w.digest();
            assert_eq!(
                d.sha256(),
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            );
            assert_eq!(d.sha1(), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
            assert_eq!(d.md5(), "d41d8cd98f00b204e9800998ecf8427e")
        }
        {
            let mut w = Writer::new();
            std::io::Write::write(&mut w, b"Hello world!\n").unwrap();
            let d = w.digest();
            assert_eq!(
                d.sha256(),
                "0ba904eae8773b70c75333db4de2f3ac45a8ad4ddba1b242f0b3cfc199391dd8"
            );
            assert_eq!(d.sha1(), "47a013e660d408619d894b20806b1d5086aab03b");
            assert_eq!(d.md5(), "59ca0efa9f5633cb0371bbc0355478d8");
        }
        {
            std::fs::write("./test_works.txt", b"Hello world!\n").unwrap();
            let d = HashDigest::from_file("./test_works.txt").unwrap();
            let _ = std::fs::remove_file("./test_works.txt");
            assert_eq!(
                d.sha256(),
                "0ba904eae8773b70c75333db4de2f3ac45a8ad4ddba1b242f0b3cfc199391dd8"
            );
            assert_eq!(d.sha1(), "47a013e660d408619d894b20806b1d5086aab03b");
            assert_eq!(d.md5(), "59ca0efa9f5633cb0371bbc0355478d8");
        }
    }

    #[tokio::test]
    async fn works_async() {
        std::fs::write("./test_works_async.txt", b"Hello world!\n").unwrap();
        let d = HashDigest::from_file_async("./test_works_async.txt")
            .await
            .unwrap();
        let _ = std::fs::remove_file("./test_works_async.txt");
        assert_eq!(
            d.sha256(),
            "0ba904eae8773b70c75333db4de2f3ac45a8ad4ddba1b242f0b3cfc199391dd8"
        );
        assert_eq!(d.sha1(), "47a013e660d408619d894b20806b1d5086aab03b");
        assert_eq!(d.md5(), "59ca0efa9f5633cb0371bbc0355478d8");
    }
}
