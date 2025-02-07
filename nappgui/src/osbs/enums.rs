pub use nappgui_sys::platform_t as Platform;

// /// Operating systems supported by NAppGUI.
// pub enum Platform {
//     /// Microsoft Windows.
//     Windows,
//     /// GNU/Linux.
//     Linux,
//     /// Apple macOS.
//     MacOS,
//     /// Apple iOS.
//     IOS,
// }

// impl Platform {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Platform::Windows => _platform_t_ekWINDOWS,
//             Platform::Linux => _platform_t_ekLINUX,
//             Platform::MacOS => _platform_t_ekMACOS,
//             Platform::IOS => _platform_t_ekIOS,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _platform_t_ekWINDOWS => Platform::Windows,
//             _platform_t_ekLINUX => Platform::Linux,
//             _platform_t_ekMACOS => Platform::MacOS,
//             _platform_t_ekIOS => Platform::IOS,
//             _ => panic!("Invalid platform"),
//         }
//     }
// }

// /// Device type.
// pub enum Device {
//     /// Desktop or laptop computer.
//     Desktop,
//     /// Phone.
//     Phone,
//     /// Tablet.
//     Tablet,
// }

// impl Device {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Device::Desktop => _device_t_ekDESKTOP,
//             Device::Phone => _device_t_ekPHONE,
//             Device::Tablet => _device_t_ekTABLET,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _device_t_ekDESKTOP => Device::Desktop,
//             _device_t_ekPHONE => Device::Phone,
//             _device_t_ekTABLET => Device::Tablet,
//             _ => panic!("Invalid device"),
//         }
//     }
// }

// /// Microsoft Windows versions.
// pub enum Win {
//     /// Windows 95, 98 or ME.
//     Win9x,
//     /// Windows NT4.
//     WinNT4,
//     /// Windows 2000.
//     Win2K,
//     /// Windows XP.
//     WinXP,
//     /// Windows XP Service Pack 1.
//     WinXP1,
//     /// Windows XP Service Pack 2.
//     WinXP2,
//     /// Windows XP Service Pack 3.
//     WinXP3,
//     /// Windows Vista.
//     WinVI,
//     /// Windows Vista Service Pack 1.
//     WinVI1,
//     /// Windows Vista Service Pack 2.
//     WinVI2,
//     /// Windows 7.
//     Win7,
//     /// Windows 7 Service Pack 1.
//     Win71,
//     /// Windows 8.
//     Win8,
//     /// Windows 8.1.
//     Win81,
//     /// Windows 10.
//     Win10,
//     /// The system is not Windows.
//     WinNo,
// }

// impl Win {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Win::Win9x => _win_t_ekWIN_9x,
//             Win::WinNT4 => _win_t_ekWIN_NT4,
//             Win::Win2K => _win_t_ekWIN_2K,
//             Win::WinXP => _win_t_ekWIN_XP,
//             Win::WinXP1 => _win_t_ekWIN_XP1,
//             Win::WinXP2 => _win_t_ekWIN_XP2,
//             Win::WinXP3 => _win_t_ekWIN_XP3,
//             Win::WinVI => _win_t_ekWIN_VI,
//             Win::WinVI1 => _win_t_ekWIN_VI1,
//             Win::WinVI2 => _win_t_ekWIN_VI2,
//             Win::Win7 => _win_t_ekWIN_7,
//             Win::Win71 => _win_t_ekWIN_71,
//             Win::Win8 => _win_t_ekWIN_8,
//             Win::Win81 => _win_t_ekWIN_81,
//             Win::Win10 => _win_t_ekWIN_10,
//             Win::WinNo => _win_t_ekWIN_NO,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _win_t_ekWIN_9x => Win::Win9x,
//             _win_t_ekWIN_NT4 => Win::WinNT4,
//             _win_t_ekWIN_2K => Win::Win2K,
//             _win_t_ekWIN_XP => Win::WinXP,
//             _win_t_ekWIN_XP1 => Win::WinXP1,
//             _win_t_ekWIN_XP2 => Win::WinXP2,
//             _win_t_ekWIN_XP3 => Win::WinXP3,
//             _win_t_ekWIN_VI => Win::WinVI,
//             _win_t_ekWIN_VI1 => Win::WinVI1,
//             _win_t_ekWIN_VI2 => Win::WinVI2,
//             _win_t_ekWIN_7 => Win::Win7,
//             _win_t_ekWIN_71 => Win::Win71,
//             _win_t_ekWIN_8 => Win::Win8,
//             _win_t_ekWIN_81 => Win::Win81,
//             _win_t_ekWIN_10 => Win::Win10,
//             _win_t_ekWIN_NO => Win::WinNo,
//             _ => panic!("Invalid Win"),
//         }
//     }
// }

// /// Represents the Byte order, or how multi-byte data is stored in memory.
// pub enum Endian {
//     /// Little endian. The lowest byte first.
//     Litend,
//     /// Big endian. The highest byte first.
//     Bigend,
// }

// impl Endian {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Endian::Litend => _endian_t_ekLITEND,
//             Endian::Bigend => _endian_t_ekBIGEND,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _endian_t_ekLITEND => Endian::Litend,
//             _endian_t_ekBIGEND => Endian::Bigend,
//             _ => panic!("Invalid Endian"),
//         }
//     }
// }

// /// Weekday.
// pub enum Weekday {
//     /// Sunday.
//     Sunday,
//     /// Monday.
//     Monday,
//     /// Tuesday.
//     Tuesday,
//     /// Wednesday.
//     Wednesday,
//     /// Thursday.
//     Thursday,
//     /// Friday.
//     Friday,
//     /// Saturday.
//     Saturday,
// }

// impl Weekday {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Weekday::Sunday => _week_day_t_ekSUNDAY,
//             Weekday::Monday => _week_day_t_ekMONDAY,
//             Weekday::Tuesday => _week_day_t_ekTUESDAY,
//             Weekday::Wednesday => _week_day_t_ekWEDNESDAY,
//             Weekday::Thursday => _week_day_t_ekTHURSDAY,
//             Weekday::Friday => _week_day_t_ekFRIDAY,
//             Weekday::Saturday => _week_day_t_ekSATURDAY,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _week_day_t_ekSUNDAY => Weekday::Sunday,
//             _week_day_t_ekMONDAY => Weekday::Monday,
//             _week_day_t_ekTUESDAY => Weekday::Tuesday,
//             _week_day_t_ekWEDNESDAY => Weekday::Wednesday,
//             _week_day_t_ekTHURSDAY => Weekday::Thursday,
//             _week_day_t_ekFRIDAY => Weekday::Friday,
//             _week_day_t_ekSATURDAY => Weekday::Saturday,
//             _ => panic!("Invalid Weekday"),
//         }
//     }
// }

// /// Month
// pub enum Month {
//     /// January.
//     January,
//     /// February.
//     February,
//     /// March.
//     March,
//     /// April.
//     April,
//     /// May.
//     May,
//     /// June.
//     June,
//     /// July.
//     July,
//     /// August.
//     August,
//     /// September.
//     September,
//     /// October.
//     October,
//     /// November.
//     November,
//     /// December.
//     December,
// }

// impl Month {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Month::January => _month_t_ekJANUARY,
//             Month::February => _month_t_ekFEBRUARY,
//             Month::March => _month_t_ekMARCH,
//             Month::April => _month_t_ekAPRIL,
//             Month::May => _month_t_ekMAY,
//             Month::June => _month_t_ekJUNE,
//             Month::July => _month_t_ekJULY,
//             Month::August => _month_t_ekAUGUST,
//             Month::September => _month_t_ekSEPTEMBER,
//             Month::October => _month_t_ekOCTOBER,
//             Month::November => _month_t_ekNOVEMBER,
//             Month::December => _month_t_ekDECEMBER,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _month_t_ekJANUARY => Month::January,
//             _month_t_ekFEBRUARY => Month::February,
//             _month_t_ekMARCH => Month::March,
//             _month_t_ekAPRIL => Month::April,
//             _month_t_ekMAY => Month::May,
//             _month_t_ekJUNE => Month::June,
//             _month_t_ekJULY => Month::July,
//             _month_t_ekAUGUST => Month::August,
//             _month_t_ekSEPTEMBER => Month::September,
//             _month_t_ekOCTOBER => Month::October,
//             _month_t_ekNOVEMBER => Month::November,
//             _month_t_ekDECEMBER => Month::December,
//             _ => panic!("Invalid Month"),
//         }
//     }
// }

// /// File type.
// pub enum FileType {
//     /// Ordinary file.
//     Archive,
//     /// Directory.
//     Directory,
//     /// Another type of file reserved for the operating system (devices, pipes, etc.)
//     OtherFile,
// }

// impl FileType {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             FileType::Archive => _file_type_t_ekARCHIVE,
//             FileType::Directory => _file_type_t_ekDIRECTORY,
//             FileType::OtherFile => _file_type_t_ekOTHERFILE,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _file_type_t_ekARCHIVE => FileType::Archive,
//             _file_type_t_ekDIRECTORY => FileType::Directory,
//             _file_type_t_ekOTHERFILE => FileType::OtherFile,
//             _ => panic!("Invalid FileType"),
//         }
//     }
// }

// /// Different ways to open a file.
// pub enum FileMode {
//     /// Read only.
//     Read,
//     /// Write only.
//     Write,
//     /// Writing at the end of the file.
//     Append,
// }

// impl FileMode {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             FileMode::Read => _file_mode_t_ekREAD,
//             FileMode::Write => _file_mode_t_ekWRITE,
//             FileMode::Append => _file_mode_t_ekAPPEND,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _file_mode_t_ekREAD => FileMode::Read,
//             _file_mode_t_ekWRITE => FileMode::Write,
//             _file_mode_t_ekAPPEND => FileMode::Append,
//             _ => panic!("Invalid FileMode"),
//         }
//     }
// }

// /// Initial position of the pointer in bfile_seek.
// pub enum FileSeek {
//     /// Start of file.
//     SeekSet,
//     /// Current position.
//     SeekCur,
//     /// End of file.
//     SeekEnd,
// }

// impl FileSeek {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             FileSeek::SeekSet => _file_seek_t_ekSEEKSET,
//             FileSeek::SeekCur => _file_seek_t_ekSEEKCUR,
//             FileSeek::SeekEnd => _file_seek_t_ekSEEKEND,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _file_seek_t_ekSEEKSET => FileSeek::SeekSet,
//             _file_seek_t_ekSEEKCUR => FileSeek::SeekCur,
//             _file_seek_t_ekSEEKEND => FileSeek::SeekEnd,
//             _ => panic!("Invalid Seek"),
//         }
//     }
// }

// /// Error codes manipulating files.
// pub enum Ferror {
//     /// The file already exists.
//     Exists,
//     /// The directory does not exist.
//     NoPath,
//     /// The file does not exists.
//     NoFile,
//     /// The name of the file exceeds the capacity of the buffer to store it.
//     BigName,
//     /// There are no more files when we travel through a directory. bfile_dir_get.
//     NoFiles,
//     /// You are trying to delete a non-empty directory. hfile_dir_destroy.
//     NoEmpty,
//     /// The file can not be accessed (possibly due to lack of permissions).
//     NoAccess,
//     /// The file is being used by another process.
//     Lock,
//     /// The file is so big. It may appear in functions that can not handle files larger than 4Gb.
//     Big,
//     /// Negative position within a file. See bfile_seek.
//     SeekNeg,
//     /// There is no more information about the error.
//     Undef,
//     /// There is no error.
//     Ok,
// }

// impl Ferror {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Ferror::Exists => _ferror_t_ekFEXISTS,
//             Ferror::NoPath => _ferror_t_ekFNOPATH,
//             Ferror::NoFile => _ferror_t_ekFNOFILE,
//             Ferror::BigName => _ferror_t_ekFBIGNAME,
//             Ferror::NoFiles => _ferror_t_ekFNOFILES,
//             Ferror::NoEmpty => _ferror_t_ekFNOEMPTY,
//             Ferror::NoAccess => _ferror_t_ekFNOACCESS,
//             Ferror::Lock => _ferror_t_ekFLOCK,
//             Ferror::Big => _ferror_t_ekFBIG,
//             Ferror::SeekNeg => _ferror_t_ekFSEEKNEG,
//             Ferror::Undef => _ferror_t_ekFUNDEF,
//             Ferror::Ok => _ferror_t_ekFOK,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _ferror_t_ekFEXISTS => Ferror::Exists,
//             _ferror_t_ekFNOPATH => Ferror::NoPath,
//             _ferror_t_ekFNOFILE => Ferror::NoFile,
//             _ferror_t_ekFBIGNAME => Ferror::BigName,
//             _ferror_t_ekFNOFILES => Ferror::NoFiles,
//             _ferror_t_ekFNOEMPTY => Ferror::NoEmpty,
//             _ferror_t_ekFNOACCESS => Ferror::NoAccess,
//             _ferror_t_ekFLOCK => Ferror::Lock,
//             _ferror_t_ekFBIG => Ferror::Big,
//             _ferror_t_ekFSEEKNEG => Ferror::SeekNeg,
//             _ferror_t_ekFUNDEF => Ferror::Undef,
//             _ferror_t_ekFOK => Ferror::Ok,
//             _ => panic!("Invalid Ferror"),
//         }
//     }
// }

// /// Error codes working with processes.
// pub enum Perror {
//     /// Error in the standard I/O channel.
//     Pipe,
//     /// Error when launching the process. Surely the command is invalid.
//     Exec,
//     /// There is no error.
//     Ok,
// }

// impl Perror {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Perror::Pipe => _perror_t_ekPPIPE,
//             Perror::Exec => _perror_t_ekPEXEC,
//             Perror::Ok => _perror_t_ekPOK,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _perror_t_ekPPIPE => Perror::Pipe,
//             _perror_t_ekPEXEC => Perror::Exec,
//             _perror_t_ekPOK => Perror::Ok,
//             _ => panic!("Invalid Perror"),
//         }
//     }
// }

// /// Error code in network communications.
// pub enum Serror {
//     /// There is no Internet connection on the device.
//     NoNet,
//     /// Unable to connect to the remote server.
//     NoHost,
//     /// The maximum wait time for the connection has been exceeded.
//     Timeout,
//     /// Error in the I/O channel when reading or writing.
//     Stream,
//     /// There is no more information about the error.
//     Undef,
//     /// There is no error.
//     Ok,
// }

// impl Serror {
//     pub(crate) fn to_i32(&self) -> i32 {
//         match self {
//             Serror::NoNet => _serror_t_ekSNONET,
//             Serror::NoHost => _serror_t_ekSNOHOST,
//             Serror::Timeout => _serror_t_ekSTIMEOUT,
//             Serror::Stream => _serror_t_ekSSTREAM,
//             Serror::Undef => _serror_t_ekSUNDEF,
//             Serror::Ok => _serror_t_ekSOK,
//         }
//     }

//     pub(crate) fn from_i32(i: i32) -> Self {
//         match i {
//             _serror_t_ekSNONET => Serror::NoNet,
//             _serror_t_ekSNOHOST => Serror::NoHost,
//             _serror_t_ekSTIMEOUT => Serror::Timeout,
//             _serror_t_ekSSTREAM => Serror::Stream,
//             _serror_t_ekSUNDEF => Serror::Undef,
//             _serror_t_ekSOK => Serror::Ok,
//             _ => panic!("Invalid Serror"),
//         }
//     }
// }
