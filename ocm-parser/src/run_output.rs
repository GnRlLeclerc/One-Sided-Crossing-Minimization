//! Serialize program output to a file for analytics using external programs.
//! Ex: python plotting and analysis

use std::{fs, io::Write};

/// Output of a run of the program
pub struct RunOutput {
    filename: String,
    algorithm: String,
    dataset: String,
    initial_crossings: u64,
    final_crossings: u64,
    elapsed_nanos: u64,
}

impl RunOutput {
    /// Create a new RunOutput
    pub fn new(
        filename: &str,
        algorithm: &str,
        dataset: &str,
        initial_crossings: u64,
        final_crossings: u64,
        elapsed_nanos: u64,
    ) -> Self {
        RunOutput {
            filename: filename.to_string(),
            algorithm: algorithm.to_string(),
            dataset: dataset.to_string(),
            initial_crossings,
            final_crossings,
            elapsed_nanos,
        }
    }

    /// Save the output to a file
    /// Format:
    /// <initial_crossings>
    /// <final_crossings>
    /// <elapsed_nanos>
    pub fn save_to_file(&self) {
        let out_path = format!("analytics/{}/{}", self.algorithm, self.dataset);

        if fs::metadata(&out_path).is_err() {
            fs::create_dir_all(&out_path).unwrap(); // Create the analytics directory if it doesn't exist
        }

        let mut file = std::fs::File::create(format!(
            "{}/{}.txt",
            out_path,
            self.filename.split('/').last().unwrap()
        ))
        .unwrap();

        file.write_all(format!("{}\n", self.initial_crossings).as_bytes())
            .unwrap();
        file.write_all(format!("{}\n", self.final_crossings).as_bytes())
            .unwrap();
        file.write_all(format!("{}\n", self.elapsed_nanos).as_bytes())
            .unwrap();
    }
}
