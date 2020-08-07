use thiserror::Error;

/// Represents a file size
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSize {
	bytes: u64,
}

impl FileSize {
	/// Create a new file size from the given number of bytes
	pub fn from_bytes(bytes: u64) -> Self {
		Self { bytes }
	}

	/// Get the number of bytes
	pub fn bytes(self) -> u64 { self.bytes }

	/// Get the number of kilobytes, rounded down
	pub fn kb(self) -> u64 { self.bytes / 1_000 }

	/// Get the number of megabytes, rounded down
	pub fn mb(self) -> u64 { self.bytes / 1_000_000 }

	/// Get the number of gigabytes, rounded down
	pub fn gb(self) -> u64 { self.bytes / 1_000_000_000 }

	/// Get the number of terabytes, rounded down
	pub fn tb(self) -> u64 { self.bytes / 1_000_000_000_000 }
}

/// Error returned from `FileSize::from_str`
#[derive(Debug, Error)]
pub enum FileSizeParseError {
	#[error("Given string was empty")]
	EmptyString,
	#[error("Error while parsing the filesize number")]
	InvalidNumber(#[source] std::num::ParseFloatError),
	#[error("No KB/MB/... ending")]
	NoEnding,
	#[error("Unknown ending (the KB/MB/... thingy)")]
	UnexpectedEnding(String),
}

impl std::str::FromStr for FileSize {
	type Err = FileSizeParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut token_iter = s.split_whitespace();
		let number: f64 = token_iter.next().ok_or(FileSizeParseError::EmptyString)?
			.parse().map_err(FileSizeParseError::InvalidNumber)?;
		let ending = token_iter.next().ok_or(FileSizeParseError::NoEnding)?;

		let ending = ending.to_lowercase();
		let multiplier: u64 = match &ending as &str {
			"b"	  => 1,
			"kb"  => 1000,
			"kib" => 1024,
			"mb"  => 1000 * 1000,
			"mib" => 1024 * 1024,
			"gb"  => 1000 * 1000 * 1000,
			"gib" => 1024 * 1024 * 1024,
			"tb"  => 1000 * 1000 * 1000 * 1000,
			"tib" => 1024 * 1024 * 1024 * 1024,
			_ => return Err(FileSizeParseError::UnexpectedEnding(ending)),
		};

		Ok(Self::from_bytes((number * multiplier as f64) as u64))
	}
}