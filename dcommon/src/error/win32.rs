use crate::error::{fixme, Error, Status};

use winapi::shared::winerror::*;

/// Generic Codes
impl Status {
    /// Generic result for Success. This is the most common success status.
    pub const OK: Status = Status(S_OK);
    /// Result of the operation is FALSE.
    pub const FALSE: Status = Status(S_FALSE);
}

/// Generic Codes
impl Error {
    /// Incorrect function.
    pub const WIN32_INVALID_FUNCTION: Error = Error(hresult_from_win32(ERROR_INVALID_FUNCTION));
    /// The system cannot find the file specified.
    pub const WIN32_FILE_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_FILE_NOT_FOUND));
    /// The system cannot find the path specified.
    pub const WIN32_PATH_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_PATH_NOT_FOUND));
    /// The system cannot open the file.
    pub const WIN32_TOO_MANY_OPEN_FILES: Error =
        Error(hresult_from_win32(ERROR_TOO_MANY_OPEN_FILES));
    /// Access is denied.
    pub const WIN32_ACCESS_DENIED: Error = Error(hresult_from_win32(ERROR_ACCESS_DENIED));
    /// The handle is invalid.
    pub const WIN32_INVALID_HANDLE: Error = Error(hresult_from_win32(ERROR_INVALID_HANDLE));
    /// The storage control blocks were destroyed.
    pub const WIN32_ARENA_TRASHED: Error = Error(hresult_from_win32(ERROR_ARENA_TRASHED));
    /// Not enough memory resources are available to process this command.
    pub const WIN32_NOT_ENOUGH_MEMORY: Error = Error(hresult_from_win32(ERROR_NOT_ENOUGH_MEMORY));
    /// The storage control block address is invalid.
    pub const WIN32_INVALID_BLOCK: Error = Error(hresult_from_win32(ERROR_INVALID_BLOCK));
    /// The environment is incorrect.
    pub const WIN32_BAD_ENVIRONMENT: Error = Error(hresult_from_win32(ERROR_BAD_ENVIRONMENT));
    /// An attempt was made to load a program with an incorrect format.
    pub const WIN32_BAD_FORMAT: Error = Error(hresult_from_win32(ERROR_BAD_FORMAT));
    /// The access code is invalid.
    pub const WIN32_INVALID_ACCESS: Error = Error(hresult_from_win32(ERROR_INVALID_ACCESS));
    /// The data is invalid.
    pub const WIN32_INVALID_DATA: Error = Error(hresult_from_win32(ERROR_INVALID_DATA));
    /// Not enough memory resources are available to complete this operation.
    pub const WIN32_OUTOFMEMORY: Error = Error(hresult_from_win32(ERROR_OUTOFMEMORY));
    /// The system cannot find the drive specified.
    pub const WIN32_INVALID_DRIVE: Error = Error(hresult_from_win32(ERROR_INVALID_DRIVE));
    /// The directory cannot be removed.
    pub const WIN32_CURRENT_DIRECTORY: Error = Error(hresult_from_win32(ERROR_CURRENT_DIRECTORY));
    /// The system cannot move the file to a different disk drive.
    pub const WIN32_NOT_SAME_DEVICE: Error = Error(hresult_from_win32(ERROR_NOT_SAME_DEVICE));
    /// There are no more files.
    pub const WIN32_NO_MORE_FILES: Error = Error(hresult_from_win32(ERROR_NO_MORE_FILES));
    /// The media is write protected.
    pub const WIN32_WRITE_PROTECT: Error = Error(hresult_from_win32(ERROR_WRITE_PROTECT));
    /// The system cannot find the device specified.
    pub const WIN32_BAD_UNIT: Error = Error(hresult_from_win32(ERROR_BAD_UNIT));
    /// The device is not ready.
    pub const WIN32_NOT_READY: Error = Error(hresult_from_win32(ERROR_NOT_READY));
    /// The device does not recognize the command.
    pub const WIN32_BAD_COMMAND: Error = Error(hresult_from_win32(ERROR_BAD_COMMAND));
    /// Data error (cyclic redundancy check).
    pub const WIN32_CRC: Error = Error(hresult_from_win32(ERROR_CRC));
    /// The program issued a command but the command length is incorrect.
    pub const WIN32_BAD_LENGTH: Error = Error(hresult_from_win32(ERROR_BAD_LENGTH));
    /// The drive cannot locate a specific area or track on the disk.
    pub const WIN32_SEEK: Error = Error(hresult_from_win32(ERROR_SEEK));
    /// The specified disk or diskette cannot be accessed.
    pub const WIN32_NOT_DOS_DISK: Error = Error(hresult_from_win32(ERROR_NOT_DOS_DISK));
    /// The drive cannot find the sector requested.
    pub const WIN32_SECTOR_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_SECTOR_NOT_FOUND));
    /// The printer is out of paper.
    pub const WIN32_OUT_OF_PAPER: Error = Error(hresult_from_win32(ERROR_OUT_OF_PAPER));
    /// The system cannot write to the specified device.
    pub const WIN32_WRITE_FAULT: Error = Error(hresult_from_win32(ERROR_WRITE_FAULT));
    /// The system cannot read from the specified device.
    pub const WIN32_READ_FAULT: Error = Error(hresult_from_win32(ERROR_READ_FAULT));
    /// A device attached to the system is not functioning.
    pub const WIN32_GEN_FAILURE: Error = Error(hresult_from_win32(ERROR_GEN_FAILURE));
    /// The process cannot access the file because it is being used by another process.
    pub const WIN32_SHARING_VIOLATION: Error = Error(hresult_from_win32(ERROR_SHARING_VIOLATION));
    /// The process cannot access the file because another process has locked a portion of the file.
    pub const WIN32_LOCK_VIOLATION: Error = Error(hresult_from_win32(ERROR_LOCK_VIOLATION));
    /// The wrong diskette is in the drive.
    /// Insert %2 (Volume Serial Number: %3) into drive %1.
    pub const WIN32_WRONG_DISK: Error = Error(hresult_from_win32(ERROR_WRONG_DISK));
    /// Too many files opened for sharing.
    pub const WIN32_SHARING_BUFFER_EXCEEDED: Error =
        Error(hresult_from_win32(ERROR_SHARING_BUFFER_EXCEEDED));
    /// Reached the end of the file.
    pub const WIN32_HANDLE_EOF: Error = Error(hresult_from_win32(ERROR_HANDLE_EOF));
    /// The disk is full.
    pub const WIN32_HANDLE_DISK_FULL: Error = Error(hresult_from_win32(ERROR_HANDLE_DISK_FULL));
    /// The request is not supported.
    pub const WIN32_NOT_SUPPORTED: Error = Error(hresult_from_win32(ERROR_NOT_SUPPORTED));
    /// Windows cannot find the network path. Verify that the network path is correct and the destination computer is not busy or turned off. If Windows still cannot find the network path, contact your network administrator.
    pub const WIN32_REM_NOT_LIST: Error = Error(hresult_from_win32(ERROR_REM_NOT_LIST));
    /// You were not connected because a duplicate name exists on the network. If joining a domain, go to System in Control Panel to change the computer name and try again. If joining a workgroup, choose another workgroup name.
    pub const WIN32_DUP_NAME: Error = Error(hresult_from_win32(ERROR_DUP_NAME));
    /// The network path was not found.
    pub const WIN32_BAD_NETPATH: Error = Error(hresult_from_win32(ERROR_BAD_NETPATH));
    /// The network is busy.
    pub const WIN32_NETWORK_BUSY: Error = Error(hresult_from_win32(ERROR_NETWORK_BUSY));
    /// The specified network resource or device is no longer available.
    pub const WIN32_DEV_NOT_EXIST: Error = Error(hresult_from_win32(ERROR_DEV_NOT_EXIST));
    /// The network BIOS command limit has been reached.
    pub const WIN32_TOO_MANY_CMDS: Error = Error(hresult_from_win32(ERROR_TOO_MANY_CMDS));
    /// A network adapter hardware error occurred.
    pub const WIN32_ADAP_HDW_ERR: Error = Error(hresult_from_win32(ERROR_ADAP_HDW_ERR));
    /// The specified server cannot perform the requested operation.
    pub const WIN32_BAD_NET_RESP: Error = Error(hresult_from_win32(ERROR_BAD_NET_RESP));
    /// An unexpected network error occurred.
    pub const WIN32_UNEXP_NET_ERR: Error = Error(hresult_from_win32(ERROR_UNEXP_NET_ERR));
    /// The remote adapter is not compatible.
    pub const WIN32_BAD_REM_ADAP: Error = Error(hresult_from_win32(ERROR_BAD_REM_ADAP));
    /// The printer queue is full.
    pub const WIN32_PRINTQ_FULL: Error = Error(hresult_from_win32(ERROR_PRINTQ_FULL));
    /// Space to store the file waiting to be printed is not available on the server.
    pub const WIN32_NO_SPOOL_SPACE: Error = Error(hresult_from_win32(ERROR_NO_SPOOL_SPACE));
    /// Your file waiting to be printed was deleted.
    pub const WIN32_PRINT_CANCELLED: Error = Error(hresult_from_win32(ERROR_PRINT_CANCELLED));
    /// The specified network name is no longer available.
    pub const WIN32_NETNAME_DELETED: Error = Error(hresult_from_win32(ERROR_NETNAME_DELETED));
    /// Network access is denied.
    pub const WIN32_NETWORK_ACCESS_DENIED: Error =
        Error(hresult_from_win32(ERROR_NETWORK_ACCESS_DENIED));
    /// The network resource type is not correct.
    pub const WIN32_BAD_DEV_TYPE: Error = Error(hresult_from_win32(ERROR_BAD_DEV_TYPE));
    /// The network name cannot be found.
    pub const WIN32_BAD_NET_NAME: Error = Error(hresult_from_win32(ERROR_BAD_NET_NAME));
    /// The name limit for the local computer network adapter card was exceeded.
    pub const WIN32_TOO_MANY_NAMES: Error = Error(hresult_from_win32(ERROR_TOO_MANY_NAMES));
    /// The network BIOS session limit was exceeded.
    pub const WIN32_TOO_MANY_SESS: Error = Error(hresult_from_win32(ERROR_TOO_MANY_SESS));
    /// The remote server has been paused or is in the process of being started.
    pub const WIN32_SHARING_PAUSED: Error = Error(hresult_from_win32(ERROR_SHARING_PAUSED));
    /// No more connections can be made to this remote computer at this time because there are already as many connections as the computer can accept.
    pub const WIN32_REQ_NOT_ACCEP: Error = Error(hresult_from_win32(ERROR_REQ_NOT_ACCEP));
    /// The specified printer or disk device has been paused.
    pub const WIN32_REDIR_PAUSED: Error = Error(hresult_from_win32(ERROR_REDIR_PAUSED));
    /// The file exists.
    pub const WIN32_FILE_EXISTS: Error = Error(hresult_from_win32(ERROR_FILE_EXISTS));
    /// The directory or file cannot be created.
    pub const WIN32_CANNOT_MAKE: Error = Error(hresult_from_win32(ERROR_CANNOT_MAKE));
    /// Fail on INT 24.
    pub const WIN32_FAIL_I24: Error = Error(hresult_from_win32(ERROR_FAIL_I24));
    /// Storage to process this request is not available.
    pub const WIN32_OUT_OF_STRUCTURES: Error = Error(hresult_from_win32(ERROR_OUT_OF_STRUCTURES));
    /// The local device name is already in use.
    pub const WIN32_ALREADY_ASSIGNED: Error = Error(hresult_from_win32(ERROR_ALREADY_ASSIGNED));
    /// The specified network password is not correct.
    pub const WIN32_INVALID_PASSWORD: Error = Error(hresult_from_win32(ERROR_INVALID_PASSWORD));
    /// The parameter is incorrect.
    pub const WIN32_INVALID_PARAMETER: Error = Error(hresult_from_win32(ERROR_INVALID_PARAMETER));
    /// A write fault occurred on the network.
    pub const WIN32_NET_WRITE_FAULT: Error = Error(hresult_from_win32(ERROR_NET_WRITE_FAULT));
    /// The system cannot start another process at this time.
    pub const WIN32_NO_PROC_SLOTS: Error = Error(hresult_from_win32(ERROR_NO_PROC_SLOTS));
    /// Cannot create another system semaphore.
    pub const WIN32_TOO_MANY_SEMAPHORES: Error =
        Error(hresult_from_win32(ERROR_TOO_MANY_SEMAPHORES));
    /// The exclusive semaphore is owned by another process.
    pub const WIN32_EXCL_SEM_ALREADY_OWNED: Error =
        Error(hresult_from_win32(ERROR_EXCL_SEM_ALREADY_OWNED));
    /// The semaphore is set and cannot be closed.
    pub const WIN32_SEM_IS_SET: Error = Error(hresult_from_win32(ERROR_SEM_IS_SET));
    /// The semaphore cannot be set again.
    pub const WIN32_TOO_MANY_SEM_REQUESTS: Error =
        Error(hresult_from_win32(ERROR_TOO_MANY_SEM_REQUESTS));
    /// Cannot request exclusive semaphores at interrupt time.
    pub const WIN32_INVALID_AT_INTERRUPT_TIME: Error =
        Error(hresult_from_win32(ERROR_INVALID_AT_INTERRUPT_TIME));
    /// The previous ownership of this semaphore has ended.
    pub const WIN32_SEM_OWNER_DIED: Error = Error(hresult_from_win32(ERROR_SEM_OWNER_DIED));
    /// Insert the diskette for drive %1.
    pub const WIN32_SEM_USER_LIMIT: Error = Error(hresult_from_win32(ERROR_SEM_USER_LIMIT));
    /// The program stopped because an alternate diskette was not inserted.
    pub const WIN32_DISK_CHANGE: Error = Error(hresult_from_win32(ERROR_DISK_CHANGE));
    /// The disk is in use or locked by another process.
    pub const WIN32_DRIVE_LOCKED: Error = Error(hresult_from_win32(ERROR_DRIVE_LOCKED));
    /// The pipe has been ended.
    pub const WIN32_BROKEN_PIPE: Error = Error(hresult_from_win32(ERROR_BROKEN_PIPE));
    /// The system cannot open the device or file specified.
    pub const WIN32_OPEN_FAILED: Error = Error(hresult_from_win32(ERROR_OPEN_FAILED));
    /// The file name is too long.
    pub const WIN32_BUFFER_OVERFLOW: Error = Error(hresult_from_win32(ERROR_BUFFER_OVERFLOW));
    /// There is not enough space on the disk.
    pub const WIN32_DISK_FULL: Error = Error(hresult_from_win32(ERROR_DISK_FULL));
    /// No more internal file identifiers available.
    pub const WIN32_NO_MORE_SEARCH_HANDLES: Error =
        Error(hresult_from_win32(ERROR_NO_MORE_SEARCH_HANDLES));
    /// The target internal file identifier is incorrect.
    pub const WIN32_INVALID_TARGET_HANDLE: Error =
        Error(hresult_from_win32(ERROR_INVALID_TARGET_HANDLE));
    /// The IOCTL call made by the application program is not correct.
    pub const WIN32_INVALID_CATEGORY: Error = Error(hresult_from_win32(ERROR_INVALID_CATEGORY));
    /// The verify-on-write switch parameter value is not correct.
    pub const WIN32_INVALID_VERIFY_SWITCH: Error =
        Error(hresult_from_win32(ERROR_INVALID_VERIFY_SWITCH));
    /// The system does not support the command requested.
    pub const WIN32_BAD_DRIVER_LEVEL: Error = Error(hresult_from_win32(ERROR_BAD_DRIVER_LEVEL));
    /// This function is not supported on this system.
    pub const WIN32_CALL_NOT_IMPLEMENTED: Error =
        Error(hresult_from_win32(ERROR_CALL_NOT_IMPLEMENTED));
    /// The semaphore timeout period has expired.
    pub const WIN32_SEM_TIMEOUT: Error = Error(hresult_from_win32(ERROR_SEM_TIMEOUT));
    /// The data area passed to a system call is too small.
    pub const WIN32_INSUFFICIENT_BUFFER: Error =
        Error(hresult_from_win32(ERROR_INSUFFICIENT_BUFFER));
    /// The filename, directory name, or volume label syntax is incorrect.
    pub const WIN32_INVALID_NAME: Error = Error(hresult_from_win32(ERROR_INVALID_NAME));
    /// The system call level is not correct.
    pub const WIN32_INVALID_LEVEL: Error = Error(hresult_from_win32(ERROR_INVALID_LEVEL));
    /// The disk has no volume label.
    pub const WIN32_NO_VOLUME_LABEL: Error = Error(hresult_from_win32(ERROR_NO_VOLUME_LABEL));
    /// The specified module could not be found.
    pub const WIN32_MOD_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_MOD_NOT_FOUND));
    /// The specified procedure could not be found.
    pub const WIN32_PROC_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_PROC_NOT_FOUND));
    /// There are no child processes to wait for.
    pub const WIN32_WAIT_NO_CHILDREN: Error = Error(hresult_from_win32(ERROR_WAIT_NO_CHILDREN));
    /// The %1 application cannot be run in Win32 mode.
    pub const WIN32_CHILD_NOT_COMPLETE: Error = Error(hresult_from_win32(ERROR_CHILD_NOT_COMPLETE));
    /// Attempt to use a file handle to an open disk partition for an operation other than raw disk I/O.
    pub const WIN32_DIRECT_ACCESS_HANDLE: Error =
        Error(hresult_from_win32(ERROR_DIRECT_ACCESS_HANDLE));
    /// An attempt was made to move the file pointer before the beginning of the file.
    pub const WIN32_NEGATIVE_SEEK: Error = Error(hresult_from_win32(ERROR_NEGATIVE_SEEK));
    /// The file pointer cannot be set on the specified device or file.
    pub const WIN32_SEEK_ON_DEVICE: Error = Error(hresult_from_win32(ERROR_SEEK_ON_DEVICE));
    /// A JOIN or SUBST command cannot be used for a drive that contains previously joined drives.
    pub const WIN32_IS_JOIN_TARGET: Error = Error(hresult_from_win32(ERROR_IS_JOIN_TARGET));
    /// An attempt was made to use a JOIN or SUBST command on a drive that has already been joined.
    pub const WIN32_IS_JOINED: Error = Error(hresult_from_win32(ERROR_IS_JOINED));
    /// An attempt was made to use a JOIN or SUBST command on a drive that has already been substituted.
    pub const WIN32_IS_SUBSTED: Error = Error(hresult_from_win32(ERROR_IS_SUBSTED));
    /// The system tried to delete the JOIN of a drive that is not joined.
    pub const WIN32_NOT_JOINED: Error = Error(hresult_from_win32(ERROR_NOT_JOINED));
    /// The system tried to delete the substitution of a drive that is not substituted.
    pub const WIN32_NOT_SUBSTED: Error = Error(hresult_from_win32(ERROR_NOT_SUBSTED));
    /// The system tried to join a drive to a directory on a joined drive.
    pub const WIN32_JOIN_TO_JOIN: Error = Error(hresult_from_win32(ERROR_JOIN_TO_JOIN));
    /// The system tried to substitute a drive to a directory on a substituted drive.
    pub const WIN32_SUBST_TO_SUBST: Error = Error(hresult_from_win32(ERROR_SUBST_TO_SUBST));
    /// The system tried to join a drive to a directory on a substituted drive.
    pub const WIN32_JOIN_TO_SUBST: Error = Error(hresult_from_win32(ERROR_JOIN_TO_SUBST));
    /// The system tried to SUBST a drive to a directory on a joined drive.
    pub const WIN32_SUBST_TO_JOIN: Error = Error(hresult_from_win32(ERROR_SUBST_TO_JOIN));
    /// The system cannot perform a JOIN or SUBST at this time.
    pub const WIN32_BUSY_DRIVE: Error = Error(hresult_from_win32(ERROR_BUSY_DRIVE));
    /// The system cannot join or substitute a drive to or for a directory on the same drive.
    pub const WIN32_SAME_DRIVE: Error = Error(hresult_from_win32(ERROR_SAME_DRIVE));
    /// The directory is not a subdirectory of the root directory.
    pub const WIN32_DIR_NOT_ROOT: Error = Error(hresult_from_win32(ERROR_DIR_NOT_ROOT));
    /// The directory is not empty.
    pub const WIN32_DIR_NOT_EMPTY: Error = Error(hresult_from_win32(ERROR_DIR_NOT_EMPTY));
    /// The path specified is being used in a substitute.
    pub const WIN32_IS_SUBST_PATH: Error = Error(hresult_from_win32(ERROR_IS_SUBST_PATH));
    /// Not enough resources are available to process this command.
    pub const WIN32_IS_JOIN_PATH: Error = Error(hresult_from_win32(ERROR_IS_JOIN_PATH));
    /// The path specified cannot be used at this time.
    pub const WIN32_PATH_BUSY: Error = Error(hresult_from_win32(ERROR_PATH_BUSY));
    /// An attempt was made to join or substitute a drive for which a directory on the drive is the target of a previous substitute.
    pub const WIN32_IS_SUBST_TARGET: Error = Error(hresult_from_win32(ERROR_IS_SUBST_TARGET));
    /// System trace information was not specified in your CONFIG.SYS file, or tracing is disallowed.
    pub const WIN32_SYSTEM_TRACE: Error = Error(hresult_from_win32(ERROR_SYSTEM_TRACE));
    /// The number of specified semaphore events for DosMuxSemWait is not correct.
    pub const WIN32_INVALID_EVENT_COUNT: Error =
        Error(hresult_from_win32(ERROR_INVALID_EVENT_COUNT));
    /// DosMuxSemWait did not execute; too many semaphores are already set.
    pub const WIN32_TOO_MANY_MUXWAITERS: Error =
        Error(hresult_from_win32(ERROR_TOO_MANY_MUXWAITERS));
    /// The DosMuxSemWait list is not correct.
    pub const WIN32_INVALID_LIST_FORMAT: Error =
        Error(hresult_from_win32(ERROR_INVALID_LIST_FORMAT));
    /// The volume label you entered exceeds the label character limit of the target file system.
    pub const WIN32_LABEL_TOO_LONG: Error = Error(hresult_from_win32(ERROR_LABEL_TOO_LONG));
    /// Cannot create another thread.
    pub const WIN32_TOO_MANY_TCBS: Error = Error(hresult_from_win32(ERROR_TOO_MANY_TCBS));
    /// The recipient process has refused the signal.
    pub const WIN32_SIGNAL_REFUSED: Error = Error(hresult_from_win32(ERROR_SIGNAL_REFUSED));
    /// The segment is already discarded and cannot be locked.
    pub const WIN32_DISCARDED: Error = Error(hresult_from_win32(ERROR_DISCARDED));
    /// The segment is already unlocked.
    pub const WIN32_NOT_LOCKED: Error = Error(hresult_from_win32(ERROR_NOT_LOCKED));
    /// The address for the thread ID is not correct.
    pub const WIN32_BAD_THREADID_ADDR: Error = Error(hresult_from_win32(ERROR_BAD_THREADID_ADDR));
    /// One or more arguments are not correct.
    pub const WIN32_BAD_ARGUMENTS: Error = Error(hresult_from_win32(ERROR_BAD_ARGUMENTS));
    /// The specified path is invalid.
    pub const WIN32_BAD_PATHNAME: Error = Error(hresult_from_win32(ERROR_BAD_PATHNAME));
    /// A signal is already pending.
    pub const WIN32_SIGNAL_PENDING: Error = Error(hresult_from_win32(ERROR_SIGNAL_PENDING));
    /// No more threads can be created in the system.
    pub const WIN32_MAX_THRDS_REACHED: Error = Error(hresult_from_win32(ERROR_MAX_THRDS_REACHED));
    /// Unable to lock a region of a file.
    pub const WIN32_LOCK_FAILED: Error = Error(hresult_from_win32(ERROR_LOCK_FAILED));
    /// The requested resource is in use.
    pub const WIN32_BUSY: Error = Error(hresult_from_win32(ERROR_BUSY));
    /// Device's command support detection is in progress.
    pub const WIN32_DEVICE_SUPPORT_IN_PROGRESS: Error =
        Error(hresult_from_win32(ERROR_DEVICE_SUPPORT_IN_PROGRESS));
    /// A lock request was not outstanding for the supplied cancel region.
    pub const WIN32_CANCEL_VIOLATION: Error = Error(hresult_from_win32(ERROR_CANCEL_VIOLATION));
    /// The file system does not support atomic changes to the lock type.
    pub const WIN32_ATOMIC_LOCKS_NOT_SUPPORTED: Error =
        Error(hresult_from_win32(ERROR_ATOMIC_LOCKS_NOT_SUPPORTED));
    /// The system detected a segment number that was not correct.
    pub const WIN32_INVALID_SEGMENT_NUMBER: Error =
        Error(hresult_from_win32(ERROR_INVALID_SEGMENT_NUMBER));
    /// The operating system cannot run %1.
    pub const WIN32_INVALID_ORDINAL: Error = Error(hresult_from_win32(ERROR_INVALID_ORDINAL));
    /// Cannot create a file when that file already exists.
    pub const WIN32_ALREADY_EXISTS: Error = Error(hresult_from_win32(ERROR_ALREADY_EXISTS));
    /// The flag passed is not correct.
    pub const WIN32_INVALID_FLAG_NUMBER: Error =
        Error(hresult_from_win32(ERROR_INVALID_FLAG_NUMBER));
    /// The specified system semaphore name was not found.
    pub const WIN32_SEM_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_SEM_NOT_FOUND));
    /// The operating system cannot run %1.
    pub const WIN32_INVALID_STARTING_CODESEG: Error =
        Error(hresult_from_win32(ERROR_INVALID_STARTING_CODESEG));
    /// The operating system cannot run %1.
    pub const WIN32_INVALID_STACKSEG: Error = Error(hresult_from_win32(ERROR_INVALID_STACKSEG));
    /// The operating system cannot run %1.
    pub const WIN32_INVALID_MODULETYPE: Error = Error(hresult_from_win32(ERROR_INVALID_MODULETYPE));
    /// Cannot run %1 in Win32 mode.
    pub const WIN32_INVALID_EXE_SIGNATURE: Error =
        Error(hresult_from_win32(ERROR_INVALID_EXE_SIGNATURE));
    /// The operating system cannot run %1.
    pub const WIN32_EXE_MARKED_INVALID: Error = Error(hresult_from_win32(ERROR_EXE_MARKED_INVALID));
    /// %1 is not a valid Win32 application.
    pub const WIN32_BAD_EXE_FORMAT: Error = Error(hresult_from_win32(ERROR_BAD_EXE_FORMAT));
    /// The operating system cannot run %1.
    pub const WIN32_ITERATED_DATA_EXCEEDS_64K: Error =
        Error(hresult_from_win32(ERROR_ITERATED_DATA_EXCEEDS_64k));
    /// The operating system cannot run %1.
    pub const WIN32_INVALID_MINALLOCSIZE: Error =
        Error(hresult_from_win32(ERROR_INVALID_MINALLOCSIZE));
    /// The operating system cannot run this application program.
    pub const WIN32_DYNLINK_FROM_INVALID_RING: Error =
        Error(hresult_from_win32(ERROR_DYNLINK_FROM_INVALID_RING));
    /// The operating system is not presently configured to run this application.
    pub const WIN32_IOPL_NOT_ENABLED: Error = Error(hresult_from_win32(ERROR_IOPL_NOT_ENABLED));
    /// The operating system cannot run %1.
    pub const WIN32_INVALID_SEGDPL: Error = Error(hresult_from_win32(ERROR_INVALID_SEGDPL));
    /// The operating system cannot run this application program.
    pub const WIN32_AUTODATASEG_EXCEEDS_64K: Error =
        Error(hresult_from_win32(ERROR_AUTODATASEG_EXCEEDS_64k));
    /// The code segment cannot be greater than or equal to 64K.
    pub const WIN32_RING2SEG_MUST_BE_MOVABLE: Error =
        Error(hresult_from_win32(ERROR_RING2SEG_MUST_BE_MOVABLE));
    /// The operating system cannot run %1.
    pub const WIN32_RELOC_CHAIN_XEEDS_SEGLIM: Error =
        Error(hresult_from_win32(ERROR_RELOC_CHAIN_XEEDS_SEGLIM));
    /// The operating system cannot run %1.
    pub const WIN32_INFLOOP_IN_RELOC_CHAIN: Error =
        Error(hresult_from_win32(ERROR_INFLOOP_IN_RELOC_CHAIN));
    /// The system could not find the environment option that was entered.
    pub const WIN32_ENVVAR_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_ENVVAR_NOT_FOUND));
    /// No process in the command subtree has a signal handler.
    pub const WIN32_NO_SIGNAL_SENT: Error = Error(hresult_from_win32(ERROR_NO_SIGNAL_SENT));
    /// The filename or extension is too long.
    pub const WIN32_FILENAME_EXCED_RANGE: Error =
        Error(hresult_from_win32(ERROR_FILENAME_EXCED_RANGE));
    /// The ring 2 stack is in use.
    pub const WIN32_RING2_STACK_IN_USE: Error = Error(hresult_from_win32(ERROR_RING2_STACK_IN_USE));
    /// The global filename characters, * or ?, are entered incorrectly or too many global filename characters are specified.
    pub const WIN32_META_EXPANSION_TOO_LONG: Error =
        Error(hresult_from_win32(ERROR_META_EXPANSION_TOO_LONG));
    /// The signal being posted is not correct.
    pub const WIN32_INVALID_SIGNAL_NUMBER: Error =
        Error(hresult_from_win32(ERROR_INVALID_SIGNAL_NUMBER));
    /// The signal handler cannot be set.
    pub const WIN32_THREAD_1_INACTIVE: Error = Error(hresult_from_win32(ERROR_THREAD_1_INACTIVE));
    /// The segment is locked and cannot be reallocated.
    pub const WIN32_LOCKED: Error = Error(hresult_from_win32(ERROR_LOCKED));
    /// Too many dynamic-link modules are attached to this program or dynamic-link module.
    pub const WIN32_TOO_MANY_MODULES: Error = Error(hresult_from_win32(ERROR_TOO_MANY_MODULES));
    /// Cannot nest calls to LoadModule.
    pub const WIN32_NESTING_NOT_ALLOWED: Error =
        Error(hresult_from_win32(ERROR_NESTING_NOT_ALLOWED));
    /// This version of %1 is not compatible with the version of Windows you're running. Check your computer's system information and then contact the software publisher.
    pub const WIN32_EXE_MACHINE_TYPE_MISMATCH: Error =
        Error(hresult_from_win32(ERROR_EXE_MACHINE_TYPE_MISMATCH));
    /// The image file %1 is signed, unable to modify.
    pub const WIN32_EXE_CANNOT_MODIFY_SIGNED_BINARY: Error =
        Error(hresult_from_win32(ERROR_EXE_CANNOT_MODIFY_SIGNED_BINARY));
    /// The image file %1 is strong signed, unable to modify.
    pub const WIN32_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY: Error = Error(hresult_from_win32(
        ERROR_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY,
    ));
    /// This file is checked out or locked for editing by another user.
    pub const WIN32_FILE_CHECKED_OUT: Error = Error(hresult_from_win32(ERROR_FILE_CHECKED_OUT));
    /// The file must be checked out before saving changes.
    pub const WIN32_CHECKOUT_REQUIRED: Error = Error(hresult_from_win32(ERROR_CHECKOUT_REQUIRED));
    /// The file type being saved or retrieved has been blocked.
    pub const WIN32_BAD_FILE_TYPE: Error = Error(hresult_from_win32(ERROR_BAD_FILE_TYPE));
    /// The file size exceeds the limit allowed and cannot be saved.
    pub const WIN32_FILE_TOO_LARGE: Error = Error(hresult_from_win32(ERROR_FILE_TOO_LARGE));
    /// Access Denied. Before opening files in this location, you must first add the web site to your trusted sites list, browse to the web site, and select the option to login automatically.
    pub const WIN32_FORMS_AUTH_REQUIRED: Error =
        Error(hresult_from_win32(ERROR_FORMS_AUTH_REQUIRED));
    /// Operation did not complete successfully because the file contains a virus or potentially unwanted software.
    pub const WIN32_VIRUS_INFECTED: Error = Error(hresult_from_win32(ERROR_VIRUS_INFECTED));
    /// This file contains a virus or potentially unwanted software and cannot be opened. Due to the nature of this virus or potentially unwanted software, the file has been removed from this location.
    pub const WIN32_VIRUS_DELETED: Error = Error(hresult_from_win32(ERROR_VIRUS_DELETED));
    /// The pipe is local.
    pub const WIN32_PIPE_LOCAL: Error = Error(hresult_from_win32(ERROR_PIPE_LOCAL));
    /// The pipe state is invalid.
    pub const WIN32_BAD_PIPE: Error = Error(hresult_from_win32(ERROR_BAD_PIPE));
    /// All pipe instances are busy.
    pub const WIN32_PIPE_BUSY: Error = Error(hresult_from_win32(ERROR_PIPE_BUSY));
    /// The pipe is being closed.
    pub const WIN32_NO_DATA: Error = Error(hresult_from_win32(ERROR_NO_DATA));
    /// No process is on the other end of the pipe.
    pub const WIN32_PIPE_NOT_CONNECTED: Error = Error(hresult_from_win32(ERROR_PIPE_NOT_CONNECTED));
    /// More data is available.
    pub const WIN32_MORE_DATA: Error = Error(hresult_from_win32(ERROR_MORE_DATA));
    /// The action requested resulted in no work being done. Error-style clean-up has been performed.
    pub const WIN32_NO_WORK_DONE: Error = Error(hresult_from_win32(fixme::ERROR_NO_WORK_DONE));
    /// The session was canceled.
    pub const WIN32_VC_DISCONNECTED: Error = Error(hresult_from_win32(ERROR_VC_DISCONNECTED));
    /// The specified extended attribute name was invalid.
    pub const WIN32_INVALID_EA_NAME: Error = Error(hresult_from_win32(ERROR_INVALID_EA_NAME));
    /// The extended attributes are inconsistent.
    pub const WIN32_EA_LIST_INCONSISTENT: Error =
        Error(hresult_from_win32(ERROR_EA_LIST_INCONSISTENT));
    /// No more data is available.
    pub const WIN32_NO_MORE_ITEMS: Error = Error(hresult_from_win32(ERROR_NO_MORE_ITEMS));
    /// The copy functions cannot be used.
    pub const WIN32_CANNOT_COPY: Error = Error(hresult_from_win32(ERROR_CANNOT_COPY));
    /// The directory name is invalid.
    pub const WIN32_DIRECTORY: Error = Error(hresult_from_win32(ERROR_DIRECTORY));
    /// The extended attributes did not fit in the buffer.
    pub const WIN32_EAS_DIDNT_FIT: Error = Error(hresult_from_win32(ERROR_EAS_DIDNT_FIT));
    /// The extended attribute file on the mounted file system is corrupt.
    pub const WIN32_EA_FILE_CORRUPT: Error = Error(hresult_from_win32(ERROR_EA_FILE_CORRUPT));
    /// The extended attribute table file is full.
    pub const WIN32_EA_TABLE_FULL: Error = Error(hresult_from_win32(ERROR_EA_TABLE_FULL));
    /// The specified extended attribute handle is invalid.
    pub const WIN32_INVALID_EA_HANDLE: Error = Error(hresult_from_win32(ERROR_INVALID_EA_HANDLE));
    /// The mounted file system does not support extended attributes.
    pub const WIN32_EAS_NOT_SUPPORTED: Error = Error(hresult_from_win32(ERROR_EAS_NOT_SUPPORTED));
    /// Too many posts were made to a semaphore.
    pub const WIN32_TOO_MANY_POSTS: Error = Error(hresult_from_win32(ERROR_TOO_MANY_POSTS));
    /// Only part of a ReadProcessMemory or WriteProcessMemory request was completed.
    pub const WIN32_PARTIAL_COPY: Error = Error(hresult_from_win32(ERROR_PARTIAL_COPY));
    /// The oplock request is denied.
    pub const WIN32_OPLOCK_NOT_GRANTED: Error = Error(hresult_from_win32(ERROR_OPLOCK_NOT_GRANTED));
    /// An invalid oplock acknowledgment was received by the system.
    pub const WIN32_INVALID_OPLOCK_PROTOCOL: Error =
        Error(hresult_from_win32(ERROR_INVALID_OPLOCK_PROTOCOL));
    /// The volume is too fragmented to complete this operation.
    pub const WIN32_DISK_TOO_FRAGMENTED: Error =
        Error(hresult_from_win32(ERROR_DISK_TOO_FRAGMENTED));
    /// The file cannot be opened because it is in the process of being deleted.
    pub const WIN32_DELETE_PENDING: Error = Error(hresult_from_win32(ERROR_DELETE_PENDING));
    /// Short name settings may not be changed on this volume due to the global registry setting.
    pub const WIN32_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING: Error = Error(
        hresult_from_win32(ERROR_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING),
    );
    /// Short names are not enabled on this volume.
    pub const WIN32_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: Error =
        Error(hresult_from_win32(ERROR_SHORT_NAMES_NOT_ENABLED_ON_VOLUME));
    /// The security stream for the given volume is in an inconsistent state.
    /// Please run CHKDSK on the volume.
    pub const WIN32_SECURITY_STREAM_IS_INCONSISTENT: Error =
        Error(hresult_from_win32(ERROR_SECURITY_STREAM_IS_INCONSISTENT));
    /// A requested file lock operation cannot be processed due to an invalid byte range.
    pub const WIN32_INVALID_LOCK_RANGE: Error = Error(hresult_from_win32(ERROR_INVALID_LOCK_RANGE));
    /// The subsystem needed to support the image type is not present.
    pub const WIN32_IMAGE_SUBSYSTEM_NOT_PRESENT: Error =
        Error(hresult_from_win32(ERROR_IMAGE_SUBSYSTEM_NOT_PRESENT));
    /// The specified file already has a notification GUID associated with it.
    pub const WIN32_NOTIFICATION_GUID_ALREADY_DEFINED: Error =
        Error(hresult_from_win32(ERROR_NOTIFICATION_GUID_ALREADY_DEFINED));
    /// An invalid exception handler routine has been detected.
    pub const WIN32_INVALID_EXCEPTION_HANDLER: Error =
        Error(hresult_from_win32(ERROR_INVALID_EXCEPTION_HANDLER));
    /// Duplicate privileges were specified for the token.
    pub const WIN32_DUPLICATE_PRIVILEGES: Error =
        Error(hresult_from_win32(ERROR_DUPLICATE_PRIVILEGES));
    /// No ranges for the specified operation were able to be processed.
    pub const WIN32_NO_RANGES_PROCESSED: Error =
        Error(hresult_from_win32(ERROR_NO_RANGES_PROCESSED));
    /// Operation is not allowed on a file system internal file.
    pub const WIN32_NOT_ALLOWED_ON_SYSTEM_FILE: Error =
        Error(hresult_from_win32(ERROR_NOT_ALLOWED_ON_SYSTEM_FILE));
    /// The physical resources of this disk have been exhausted.
    pub const WIN32_DISK_RESOURCES_EXHAUSTED: Error =
        Error(hresult_from_win32(ERROR_DISK_RESOURCES_EXHAUSTED));
    /// The token representing the data is invalid.
    pub const WIN32_INVALID_TOKEN: Error = Error(hresult_from_win32(ERROR_INVALID_TOKEN));
    /// The device does not support the command feature.
    pub const WIN32_DEVICE_FEATURE_NOT_SUPPORTED: Error =
        Error(hresult_from_win32(ERROR_DEVICE_FEATURE_NOT_SUPPORTED));
    /// The system cannot find message text for message number 0x%1 in the message file for %2.
    pub const WIN32_MR_MID_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_MR_MID_NOT_FOUND));
    /// The scope specified was not found.
    pub const WIN32_SCOPE_NOT_FOUND: Error = Error(hresult_from_win32(ERROR_SCOPE_NOT_FOUND));
    /// The Central Access Policy specified is not defined on the target machine.
    pub const WIN32_UNDEFINED_SCOPE: Error = Error(hresult_from_win32(ERROR_UNDEFINED_SCOPE));
    /// The Central Access Policy obtained from Active Directory is invalid.
    pub const WIN32_INVALID_CAP: Error = Error(hresult_from_win32(ERROR_INVALID_CAP));
    /// The device is unreachable.
    pub const WIN32_DEVICE_UNREACHABLE: Error = Error(hresult_from_win32(ERROR_DEVICE_UNREACHABLE));
    /// The target device has insufficient resources to complete the operation.
    pub const WIN32_DEVICE_NO_RESOURCES: Error =
        Error(hresult_from_win32(ERROR_DEVICE_NO_RESOURCES));
    /// A data integrity checksum error occurred. Data in the file stream is corrupt.
    pub const WIN32_DATA_CHECKSUM_ERROR: Error =
        Error(hresult_from_win32(ERROR_DATA_CHECKSUM_ERROR));
    /// An attempt was made to modify both a KERNEL and normal Extended Attribute (EA) in the same operation.
    pub const WIN32_INTERMIXED_KERNEL_EA_OPERATION: Error =
        Error(hresult_from_win32(ERROR_INTERMIXED_KERNEL_EA_OPERATION));
    /// Device does not support file-level TRIM.
    pub const WIN32_FILE_LEVEL_TRIM_NOT_SUPPORTED: Error =
        Error(hresult_from_win32(ERROR_FILE_LEVEL_TRIM_NOT_SUPPORTED));
    /// The command specified a data offset that does not align to the device's granularity/alignment.
    pub const WIN32_OFFSET_ALIGNMENT_VIOLATION: Error =
        Error(hresult_from_win32(ERROR_OFFSET_ALIGNMENT_VIOLATION));
    /// The command specified an invalid field in its parameter list.
    pub const WIN32_INVALID_FIELD_IN_PARAMETER_LIST: Error =
        Error(hresult_from_win32(ERROR_INVALID_FIELD_IN_PARAMETER_LIST));
    /// An operation is currently in progress with the device.
    pub const WIN32_OPERATION_IN_PROGRESS: Error =
        Error(hresult_from_win32(ERROR_OPERATION_IN_PROGRESS));
    /// An attempt was made to send down the command via an invalid path to the target device.
    pub const WIN32_BAD_DEVICE_PATH: Error = Error(hresult_from_win32(ERROR_BAD_DEVICE_PATH));
    /// The command specified a number of descriptors that exceeded the maximum supported by the device.
    pub const WIN32_TOO_MANY_DESCRIPTORS: Error =
        Error(hresult_from_win32(ERROR_TOO_MANY_DESCRIPTORS));
    /// Scrub is disabled on the specified file.
    pub const WIN32_SCRUB_DATA_DISABLED: Error =
        Error(hresult_from_win32(ERROR_SCRUB_DATA_DISABLED));
    /// The storage device does not provide redundancy.
    pub const WIN32_NOT_REDUNDANT_STORAGE: Error =
        Error(hresult_from_win32(ERROR_NOT_REDUNDANT_STORAGE));
    /// An operation is not supported on a resident file.
    pub const WIN32_RESIDENT_FILE_NOT_SUPPORTED: Error =
        Error(hresult_from_win32(ERROR_RESIDENT_FILE_NOT_SUPPORTED));
    /// An operation is not supported on a compressed file.
    pub const WIN32_COMPRESSED_FILE_NOT_SUPPORTED: Error =
        Error(hresult_from_win32(ERROR_COMPRESSED_FILE_NOT_SUPPORTED));
    /// An operation is not supported on a directory.
    pub const WIN32_DIRECTORY_NOT_SUPPORTED: Error =
        Error(hresult_from_win32(ERROR_DIRECTORY_NOT_SUPPORTED));
    /// The specified copy of the requested data could not be read.
    pub const WIN32_NOT_READ_FROM_COPY: Error = Error(hresult_from_win32(ERROR_NOT_READ_FROM_COPY));
    /// The specified data could not be written to any of the copies.
    pub const WIN32_FT_WRITE_FAILURE: Error = Error(hresult_from_win32(ERROR_FT_WRITE_FAILURE));
    /// One or more copies of data on this device may be out of sync. No writes may be performed until a data integrity scan is completed.
    pub const WIN32_FT_DI_SCAN_REQUIRED: Error =
        Error(hresult_from_win32(ERROR_FT_DI_SCAN_REQUIRED));
    /// The supplied kernel information version is invalid.
    pub const WIN32_INVALID_KERNEL_INFO_VERSION: Error =
        Error(hresult_from_win32(ERROR_INVALID_KERNEL_INFO_VERSION));
    /// The supplied PEP information version is invalid.
    pub const WIN32_INVALID_PEP_INFO_VERSION: Error =
        Error(hresult_from_win32(ERROR_INVALID_PEP_INFO_VERSION));
    /// This object is not externally backed by any provider.
    pub const WIN32_OBJECT_NOT_EXTERNALLY_BACKED: Error =
        Error(hresult_from_win32(ERROR_OBJECT_NOT_EXTERNALLY_BACKED));
    /// The external backing provider is not recognized.
    pub const WIN32_EXTERNAL_BACKING_PROVIDER_UNKNOWN: Error =
        Error(hresult_from_win32(ERROR_EXTERNAL_BACKING_PROVIDER_UNKNOWN));
    /// No action was taken as a system reboot is required.
    pub const WIN32_FAIL_NOACTION_REBOOT: Error =
        Error(hresult_from_win32(ERROR_FAIL_NOACTION_REBOOT));
    /// The shutdown operation failed.
    pub const WIN32_FAIL_SHUTDOWN: Error = Error(hresult_from_win32(ERROR_FAIL_SHUTDOWN));
    /// The restart operation failed.
    pub const WIN32_FAIL_RESTART: Error = Error(hresult_from_win32(ERROR_FAIL_RESTART));
    /// The maximum number of sessions has been reached.
    pub const WIN32_MAX_SESSIONS_REACHED: Error =
        Error(hresult_from_win32(ERROR_MAX_SESSIONS_REACHED));
    /// The thread is already in background processing mode.
    pub const WIN32_THREAD_MODE_ALREADY_BACKGROUND: Error =
        Error(hresult_from_win32(ERROR_THREAD_MODE_ALREADY_BACKGROUND));
    /// The thread is not in background processing mode.
    pub const WIN32_THREAD_MODE_NOT_BACKGROUND: Error =
        Error(hresult_from_win32(ERROR_THREAD_MODE_NOT_BACKGROUND));
    /// The process is already in background processing mode.
    pub const WIN32_PROCESS_MODE_ALREADY_BACKGROUND: Error =
        Error(hresult_from_win32(ERROR_PROCESS_MODE_ALREADY_BACKGROUND));
    /// The process is not in background processing mode.
    pub const WIN32_PROCESS_MODE_NOT_BACKGROUND: Error =
        Error(hresult_from_win32(ERROR_PROCESS_MODE_NOT_BACKGROUND));
    /// The Windows Subsystem for Linux has not been enabled.
    pub const WIN32_LINUX_SUBSYSTEM_NOT_PRESENT: Error =
        Error(hresult_from_win32(fixme::ERROR_LINUX_SUBSYSTEM_NOT_PRESENT));
}

const fn hresult_from_win32(x: u32) -> HRESULT {
    ((x & 0x0000FFFF) | ((FACILITY_WIN32 as u32) << 16) | 0x80000000) as HRESULT
}
