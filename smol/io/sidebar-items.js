window.SIDEBAR_ITEMS = {"enum":[["ErrorKind","A list specifying general categories of I/O error."],["SeekFrom","Enumeration of possible methods to seek within an I/O object."]],"fn":[["copy","Copies the entire contents of a reader into a writer."],["empty","Creates an empty reader."],["repeat","Creates an infinite reader that reads the same byte repeatedly."],["sink","Creates a writer that consumes and drops all data."],["split","Splits a stream into [`AsyncRead`] and [`AsyncWrite`] halves."]],"struct":[["AssertAsync","Asserts that a type implementing [`std::io`] traits can be used as an async type."],["BlockOn","Blocks on all async I/O operations and implements [`std::io`] traits."],["BufReader","Adds buffering to a reader."],["BufWriter","Adds buffering to a writer."],["Bytes","Reader for the [`AsyncReadExt::bytes()`] method."],["Chain","Reader for the [`AsyncReadExt::chain()`] method."],["CloseFuture","Future for the [`AsyncWriteExt::close()`] method."],["Cursor","Gives an in-memory buffer a cursor for reading and writing."],["Empty","Reader for the [`empty()`] function."],["Error","The error type for I/O operations of the `Read`, `Write`, `Seek`, and associated traits."],["FillBuf","Future for the [`AsyncBufReadExt::fill_buf()`] method."],["FlushFuture","Future for the [`AsyncWriteExt::flush()`] method."],["Lines","Stream for the [`AsyncBufReadExt::lines()`] method."],["ReadExactFuture","Future for the [`AsyncReadExt::read_exact()`] method."],["ReadFuture","Future for the [`AsyncReadExt::read()`] method."],["ReadHalf","The read half returned by [`split()`]."],["ReadLineFuture","Future for the [`AsyncBufReadExt::read_line()`] method."],["ReadToEndFuture","Future for the [`AsyncReadExt::read_to_end()`] method."],["ReadToStringFuture","Future for the [`AsyncReadExt::read_to_string()`] method."],["ReadUntilFuture","Future for the [`AsyncBufReadExt::read_until()`] method."],["ReadVectoredFuture","Future for the [`AsyncReadExt::read_vectored()`] method."],["Repeat","Reader for the [`repeat()`] function."],["SeekFuture","Future for the [`AsyncSeekExt::seek()`] method."],["Sink","Writer for the [`sink()`] function."],["Split","Stream for the [`AsyncBufReadExt::split()`] method."],["Take","Reader for the [`AsyncReadExt::take()`] method."],["WriteAllFuture","Future for the [`AsyncWriteExt::write_all()`] method."],["WriteFuture","Future for the [`AsyncWriteExt::write()`] method."],["WriteHalf","The write half returned by [`split()`]."],["WriteVectoredFuture","Future for the [`AsyncWriteExt::write_vectored()`] method."]],"trait":[["AsyncBufRead","Read bytes asynchronously."],["AsyncBufReadExt","Extension trait for [`AsyncBufRead`]."],["AsyncRead","Read bytes asynchronously."],["AsyncReadExt","Extension trait for [`AsyncRead`]."],["AsyncSeek","Seek bytes asynchronously."],["AsyncSeekExt","Extension trait for [`AsyncSeek`]."],["AsyncWrite","Write bytes asynchronously."],["AsyncWriteExt","Extension trait for [`AsyncWrite`]."]],"type":[["BoxedReader","Type alias for `Pin<Box<dyn AsyncRead + Send + 'static>>`."],["BoxedWriter","Type alias for `Pin<Box<dyn AsyncWrite + Send + 'static>>`."],["Result","A specialized `Result` type for I/O operations."]]};