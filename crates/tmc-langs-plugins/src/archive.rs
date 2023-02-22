//! Contains types that abstract over the various archive formats.

use std::{
    io::{Cursor, Seek, Write},
    path::Path,
};
use tar::Builder;
use tmc_langs_framework::{Compression, TmcError};
use tmc_langs_util::file_util;
use zip::{write::FileOptions, ZipWriter};

pub enum ArchiveBuilder<W: Write + Seek> {
    Tar(Builder<W>),
    TarZstd(W, Builder<Cursor<Vec<u8>>>),
    Zip(ZipWriter<W>),
}

impl<W: Write + Seek> ArchiveBuilder<W> {
    pub fn new(writer: W, compression: Compression) -> Self {
        match compression {
            Compression::Tar => Self::Tar(Builder::new(writer)),
            Compression::TarZstd => Self::TarZstd(writer, Builder::new(Cursor::new(vec![]))),
            Compression::Zip => Self::Zip(ZipWriter::new(writer)),
        }
    }

    /// Does not include any files within the directory.
    pub fn add_directory(&mut self, path: &str) -> Result<(), TmcError> {
        log::trace!("adding directory {}", path);
        match self {
            Self::Tar(builder) => {
                builder.append_dir(path, path).map_err(TmcError::TarWrite)?;
            }
            Self::TarZstd(_, builder) => {
                builder.append_dir(path, path).map_err(TmcError::TarWrite)?;
            }
            Self::Zip(builder) => {
                builder.add_directory(path, FileOptions::default().unix_permissions(0o755))?
            }
        }
        Ok(())
    }

    pub fn add_file(&mut self, source: &Path, target: &str) -> Result<(), TmcError> {
        log::trace!("writing file {} as {}", source.display(), target);
        match self {
            Self::Tar(builder) => builder
                .append_path_with_name(source, target)
                .map_err(TmcError::TarWrite)?,
            Self::TarZstd(_, builder) => builder
                .append_path_with_name(source, target)
                .map_err(TmcError::TarWrite)?,
            Self::Zip(builder) => {
                let bytes = file_util::read_file(source)?;
                builder.start_file(target, FileOptions::default().unix_permissions(0o755))?;
                builder
                    .write_all(&bytes)
                    .map_err(|e| TmcError::ZipWrite(source.into(), e))?;
            }
        }
        Ok(())
    }

    pub fn finish(self) -> Result<W, TmcError> {
        let res = match self {
            Self::Tar(builder) => builder.into_inner().map_err(TmcError::TarWrite)?,
            Self::TarZstd(mut writer, builder) => {
                let tar_data = builder.into_inner().map_err(TmcError::TarWrite)?;
                zstd::stream::copy_encode(tar_data.get_ref().as_slice(), &mut writer, 0)
                    .map_err(TmcError::ZstdWrite)?;
                writer
            }
            Self::Zip(mut builder) => builder.finish()?,
        };
        Ok(res)
    }
}
