window.SIDEBAR_ITEMS = {"fn":[["copy","Copies the file or directory at source into the target path. If the source is a file and the target is not a directory, the source file is copied to the target path. If the source is a file and the target is a directory, the source file is copied into the target directory. If the source is a directory and the target is not a file, the source directory and all files in it are copied recursively into the target directory. For example, with source=dir1 and target=dir2, dir1/file would be copied to dir2/dir1/file. If the source is a directory and the target is a file, an error is returned."],["create_dir",""],["create_dir_all",""],["create_file","Note: creates all intermediary directories if needed."],["create_file_lock","Creates a file and wraps it in a lock. If a file already exists at the path, it acquires a lock on it first and then recreates it. Note: creates all intermediary directories if needed."],["named_temp_file",""],["open_file",""],["open_file_lock","Opens and locks the given file. Note: Does not work on directories on Windows."],["read_dir",""],["read_file",""],["read_file_to_string",""],["read_file_to_string_lossy",""],["read_to_file","Reads all of the data from source and writes it into a new file at target."],["remove_all","Removes whatever is at the path, whether it is a directory or file. The _all suffix hopefully makes the function sound at least slightly dangerous."],["remove_dir_all",""],["remove_dir_empty",""],["remove_file",""],["rename",""],["temp_file",""],["write_to_file",""],["write_to_writer",""]],"macro":[["lock","Convenience macro for locking a path."]],"struct":[["FdLockWrapper",""],["FileLock","Wrapper for fd_lock::FdLock. Used to lock files/directories to prevent concurrent access from multiple instances of tmc-langs."],["FileLockGuard","Guard that holds the locked file."]]};